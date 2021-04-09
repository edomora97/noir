use std::collections::HashMap;

use itertools::Itertools;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use crate::block::{BatchMode, InnerBlock};
use crate::channel::BoundedChannelSender;
use crate::config::{EnvironmentConfig, ExecutionRuntime, LocalRuntimeConfig, RemoteRuntimeConfig};
use crate::network::{Coord, NetworkTopology};
use crate::operator::Operator;
use crate::stream::BlockId;
use crate::worker::spawn_worker;

/// The identifier of an host.
pub type HostId = usize;
/// The identifier of a replica of a block in the execution graph.
pub type ReplicaId = usize;

/// Metadata associated to a block in the execution graph.
#[derive(Clone, Debug)]
pub struct ExecutionMetadata {
    /// The coordinate of the block (it's id, replica id, ...).
    pub(crate) coord: Coord,
    /// The total number of replicas of the block.
    pub(crate) num_replicas: usize,
    /// The global identifier of the replica (from 0 to num_replicas-1)
    pub(crate) global_id: usize,
    /// The total number of previous blocks inside the execution graph.
    pub(crate) prev: Vec<Coord>,
    /// A reference to the `NetworkTopology` that keeps the state of the network.
    pub(crate) network: Arc<Mutex<NetworkTopology>>,
    /// The batching mode to use inside this block.
    pub(crate) batch_mode: BatchMode,
}

/// Handle that the scheduler uses to start the computation of a block.
pub(crate) struct StartHandle {
    /// Sender for the `ExecutionMetadata` sent to the worker.
    starter: BoundedChannelSender<ExecutionMetadata>,
    /// `JoinHandle` used to wait until a block has finished working.
    join_handle: JoinHandle<()>,
}

/// Information about a block in the job graph.
#[derive(Debug, Clone)]
struct SchedulerBlockInfo {
    /// The identifier of the current block.
    block_id: BlockId,
    /// String representation of the block.
    repr: String,
    /// The total number of replicas of the block.
    num_replicas: usize,
    /// All the replicas, grouped by host.
    replicas: HashMap<HostId, Vec<Coord>>,
    /// All the global ids, grouped by coordinate.
    global_ids: HashMap<Coord, usize>,
    /// The batching mode to use inside this block.
    batch_mode: BatchMode,
}

/// The `Scheduler` is the entity that keeps track of all the blocks of the job graph and when the
/// execution starts it builds the execution graph and actually start the workers.
pub(crate) struct Scheduler {
    /// The configuration of the environment.
    config: EnvironmentConfig,
    /// Adjacency list of the job graph.
    next_blocks: HashMap<BlockId, Vec<BlockId>>,
    /// Reverse adjacency list of the job graph.
    prev_blocks: HashMap<BlockId, Vec<BlockId>>,
    /// Information about the blocks known to the scheduler.
    block_info: HashMap<BlockId, SchedulerBlockInfo>,
    /// The list of handles of each block in the execution graph.
    start_handles: Vec<(Coord, StartHandle)>,
    /// The network topology that keeps track of all the connections inside the execution graph.
    network: NetworkTopology,
}

impl Scheduler {
    pub fn new(config: EnvironmentConfig) -> Self {
        Self {
            next_blocks: Default::default(),
            prev_blocks: Default::default(),
            block_info: Default::default(),
            start_handles: Default::default(),
            network: NetworkTopology::new(config.clone()),
            config,
        }
    }

    /// Register a new block inside the scheduler.
    ///
    /// This spawns a worker for each replica of the block in the execution graph and saves its
    /// start handle. The handle will be later used to actually start the worker when the
    /// computation is asked to begin.
    pub(crate) fn add_block<OperatorChain>(&mut self, block: InnerBlock<OperatorChain>)
    where
        OperatorChain: Operator + Send + 'static,
    {
        let block_id = block.id;
        let info = self.block_info(&block);
        info!(
            "Adding new block id={}: {} {:?}",
            block_id,
            block.to_string(),
            info
        );

        // duplicate the block in the execution graph
        let mut blocks = vec![];
        let local_replicas = info.replicas(self.config.host_id.unwrap());
        blocks.reserve(local_replicas.len());
        if !local_replicas.is_empty() {
            let coord = local_replicas[0];
            blocks.push((coord, block));
        }
        // not all blocks can be cloned: clone only when necessary
        if local_replicas.len() > 1 {
            for coord in &local_replicas[1..] {
                blocks.push((*coord, blocks[0].1.clone()));
            }
        }

        self.block_info.insert(block_id, info);

        for (coord, block) in blocks {
            // spawn the actual worker
            let start_handle = spawn_worker(block);
            self.start_handles.push((coord, start_handle));
        }
    }

    /// Connect a pair of blocks inside the job graph.
    pub(crate) fn connect_blocks(&mut self, from: BlockId, to: BlockId) {
        info!("Connecting blocks: {} -> {}", from, to);
        self.next_blocks.entry(from).or_default().push(to);
        self.prev_blocks.entry(to).or_default().push(from);
    }

    /// Start the computation returning the list of handles used to join the workers.
    pub(crate) fn start(mut self) {
        info!("Starting scheduler: {:#?}", self.config);
        self.log_topology();
        self.build_execution_graph();
        self.network.finalize_topology();
        self.network.log_topology();

        let mut join = vec![];

        let prev: HashMap<_, _> = self
            .block_info
            .keys()
            .map(|&id| (id, self.prev(id)))
            .collect();
        let network = Arc::new(Mutex::new(self.network));
        // start the execution
        for (coord, handle) in self.start_handles {
            let block_info = &self.block_info[&coord.block_id];
            let num_replicas = block_info.num_replicas;
            let global_id = block_info.global_ids[&coord];
            let metadata = ExecutionMetadata {
                coord,
                num_replicas,
                global_id,
                prev: prev[&coord.block_id].clone(),
                network: network.clone(),
                batch_mode: block_info.batch_mode,
            };
            handle.starter.send(metadata).unwrap();
            join.push(handle.join_handle);
        }
        for handle in join {
            handle.join().unwrap();
        }
        network.lock().unwrap().stop_and_wait();
    }

    /// Get the list of replicas before the specified block.
    fn prev(&self, block_id: BlockId) -> Vec<Coord> {
        if let Some(prev) = self.prev_blocks.get(&block_id) {
            let mut result = Vec::new();
            for prev in prev {
                result.extend(self.block_info[prev].replicas.values().flatten());
            }
            result
        } else {
            Default::default()
        }
    }

    /// Get the ids of the previous blocks of a given block in the job graph
    pub(crate) fn prev_blocks(&self, block_id: BlockId) -> Option<Vec<BlockId>> {
        self.prev_blocks.get(&block_id).cloned()
    }

    /// Build the execution graph for the network topology, multiplying each block of the job graph
    /// into all its replicas.
    fn build_execution_graph(&mut self) {
        for (from_block_id, next) in self.next_blocks.iter() {
            let from = &self.block_info[from_block_id];
            for to_block_id in next.iter() {
                let to = &self.block_info[to_block_id];
                // for each pair (from -> to) inside the job graph, connect all the corresponding
                // jobs of the execution graph
                for &from_coord in from.replicas.values().flatten() {
                    for &to_coord in to.replicas.values().flatten() {
                        self.network.connect(from_coord, to_coord);
                    }
                }
            }
        }
    }

    fn log_topology(&self) {
        let mut topology = "Job graph:".to_string();
        for block_id in 0..self.block_info.len() {
            topology += &format!("\n  {}: {}", block_id, self.block_info[&block_id].repr);
            if let Some(next) = &self.next_blocks.get(&block_id) {
                let sorted = next.iter().sorted().collect_vec();
                topology += &format!("\n    -> {:?}", sorted);
            }
        }
        debug!("{}", topology);
        let mut assignments = "Replicas:".to_string();
        for block_id in 0..self.block_info.len() {
            assignments += &format!("\n  {}:", block_id);
            let replicas = self.block_info[&block_id]
                .replicas
                .values()
                .flatten()
                .sorted();
            for &coord in replicas {
                assignments += &format!(" {}", coord);
            }
        }
        debug!("{}", assignments);
    }

    /// Extract the `SchedulerBlockInfo` of a block.
    fn block_info<OperatorChain>(&self, block: &InnerBlock<OperatorChain>) -> SchedulerBlockInfo
    where
        OperatorChain: Operator,
    {
        match &self.config.runtime {
            ExecutionRuntime::Local(local) => self.local_block_info(block, local),
            ExecutionRuntime::Remote(remote) => self.remote_block_info(block, remote),
        }
    }

    /// Extract the `SchedulerBlockInfo` of a block that runs only locally.
    ///
    /// The number of replicas will be the minimum between:
    ///
    ///  - the number of logical cores.
    ///  - the `max_parallelism` of the block.
    fn local_block_info<OperatorChain>(
        &self,
        block: &InnerBlock<OperatorChain>,
        local: &LocalRuntimeConfig,
    ) -> SchedulerBlockInfo
    where
        OperatorChain: Operator,
    {
        let max_parallelism = block.scheduler_requirements.max_parallelism;
        let num_replicas = local.num_cores.min(max_parallelism.unwrap_or(usize::MAX));
        debug!(
            "Block {} will have {} local replicas (max_parallelism={:?})",
            block.id, num_replicas, max_parallelism
        );
        let host_id = self.config.host_id.unwrap();
        let replicas = (0..num_replicas).map(|r| Coord::new(block.id, host_id, r));
        let global_ids = (0..num_replicas).map(|r| (Coord::new(block.id, host_id, r), r));
        SchedulerBlockInfo {
            block_id: block.id,
            repr: block.to_string(),
            num_replicas,
            replicas: vec![(host_id, replicas.collect())].into_iter().collect(),
            global_ids: global_ids.into_iter().collect(),
            batch_mode: block.batch_mode,
        }
    }

    /// Extract the `SchedulerBlockInfo` of a block that runs remotely.
    ///
    /// The block can be replicated at most `max_parallelism` times (if specified). Assign the
    /// replicas starting from the first host giving as much replicas as possible..
    fn remote_block_info<OperatorChain>(
        &self,
        block: &InnerBlock<OperatorChain>,
        remote: &RemoteRuntimeConfig,
    ) -> SchedulerBlockInfo
    where
        OperatorChain: Operator,
    {
        let max_parallelism = block.scheduler_requirements.max_parallelism;
        debug!("Allocating block {} on remote runtime", block.id);
        // number of replicas we can assign at most
        let mut remaining_replicas = max_parallelism.unwrap_or(usize::MAX);
        let mut num_replicas = 0;
        let mut replicas: HashMap<_, Vec<_>> = HashMap::default();
        let mut global_ids = HashMap::default();
        for (host_id, host_info) in remote.hosts.iter().enumerate() {
            let num_host_replicas = host_info.num_cores.min(remaining_replicas);
            debug!(
                "Block {} will have {} replicas on {} (max_parallelism={:?}, num_cores={})",
                block.id,
                num_host_replicas,
                host_info.to_string(),
                max_parallelism,
                host_info.num_cores
            );
            remaining_replicas -= num_host_replicas;
            let host_replicas = replicas.entry(host_id).or_default();
            for replica_id in 0..num_host_replicas {
                let coord = Coord::new(block.id, host_id, replica_id);
                host_replicas.push(coord);
                global_ids.insert(coord, num_replicas + replica_id);
            }
            num_replicas += num_host_replicas;
        }
        SchedulerBlockInfo {
            block_id: block.id,
            repr: block.to_string(),
            num_replicas,
            replicas,
            global_ids,
            batch_mode: block.batch_mode,
        }
    }
}

impl StartHandle {
    pub(crate) fn new(
        starter: BoundedChannelSender<ExecutionMetadata>,
        join_handle: JoinHandle<()>,
    ) -> Self {
        Self {
            starter,
            join_handle,
        }
    }
}

impl SchedulerBlockInfo {
    /// The list of replicas of the block inside a given host.
    fn replicas(&self, host_id: HostId) -> Vec<Coord> {
        self.replicas.get(&host_id).cloned().unwrap_or_default()
    }
}
