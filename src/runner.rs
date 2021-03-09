use std::borrow::Cow;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
use std::path::Path;

use itertools::Itertools;
use ssh2::Session;
use tokio::task::spawn_blocking;

use crate::config::{RemoteHostConfig, RemoteRuntimeConfig};
use crate::scheduler::HostId;

/// Environment variable set by the runner with the host id of the process. If it's missing the
/// process will have to spawn the processes by itself.
pub(crate) const HOST_ID_ENV_VAR: &str = "RSTREAM_HOST_ID";
/// Environment variable set by the runner with the content of the config file so that it's not
/// required to have it on all the hosts.
pub(crate) const CONFIG_ENV_VAR: &str = "RSTREAM_CONFIG";
/// Size of the buffer used to send the executable file via SCP.
pub(crate) const SCP_BUFFER_SIZE: usize = 64 * 1024;

/// Spawn all the remote workers via ssh and wait until all of them complete, after that exit from
/// the process,
///
/// If this was already a spawned process to nothing.
pub(crate) async fn spawn_remote_workers(config: RemoteRuntimeConfig) {
    // if this process already comes from a the spawner do not spawn again!
    if is_spawned_process() {
        return;
    }
    // from now we are sure this is the process that should spawn the remote workers
    info!("Spawning {} remote workers", config.hosts.len());
    let config_str = serde_yaml::to_string(&config).unwrap();
    let mut join_handles = Vec::new();
    for (host_id, host) in config.hosts.into_iter().enumerate() {
        let config_str = config_str.clone();
        let join_handle = spawn_blocking(move || {
            let config_str = config_str.clone();
            spawn_remote_worker(host_id, host, config_str)
        });
        join_handles.push(join_handle);
    }
    for join_handle in join_handles {
        join_handle.await.unwrap();
    }
    // all the remote processes have finished, exit to avoid running the environment inside the
    // spawner process
    std::process::exit(0);
}

/// Check if this is a spawned process.
fn is_spawned_process() -> bool {
    std::env::var_os(HOST_ID_ENV_VAR).is_some()
}

/// Spawn the remote worker.
///
/// - Connect via SSH to the remote host
/// - Ask for a temporary file with `mktemp`
/// - Send the local executable using SCP
/// - Make it executable using `chmod`
/// - Spawn the worker setting the correct environment variables
/// - Redirect the remote stderr to the local one
/// - Remove the remote file on exit
///
/// This function is allowed to block (i.e. not be asynchronous) since it will be run inside a
/// `spawn_blocking`.
fn spawn_remote_worker(host_id: HostId, mut host: RemoteHostConfig, config_str: String) {
    if host.ssh.username.is_none() {
        host.ssh.username = Some(whoami::username());
    }
    info!("Spawning remote worker for host {}: {:#?}", host_id, host);

    // connect to the ssh server
    let address = (host.address.as_str(), host.ssh.ssh_port);
    let stream = TcpStream::connect(address).unwrap_or_else(|e| {
        panic!(
            "Failed to connect to remote SSH for host {} at {} port {}: {:?}",
            host_id, host.address, host.ssh.ssh_port, e
        )
    });
    let mut session = Session::new().unwrap();
    session.set_tcp_stream(stream);
    session.handshake().unwrap();
    debug!(
        "Connected to ssh server for host {}: {:?}",
        host_id, address
    );

    // try to authenticate
    let username = host.ssh.username.as_ref().unwrap().as_str();
    match (host.ssh.password.as_ref(), host.ssh.key_file.as_ref()) {
        (None, None) => {
            session.userauth_agent(username).unwrap();
        }
        (Some(password), None) => {
            session
                .userauth_password(username, password.as_str())
                .unwrap();
        }
        (None, Some(key_file)) => session
            .userauth_pubkey_file(
                username,
                None,
                key_file.as_path(),
                host.ssh.key_passphrase.as_deref(),
            )
            .unwrap(),
        (Some(_), Some(_)) => unreachable!("Cannot use both password and key"),
    }
    assert!(
        session.authenticated(),
        "Failed to authenticate to remote host {} at {:?}",
        host_id,
        address
    );
    debug!("Authentication succeeded to host {}", host_id);

    // generate a temporary file on remote host
    let (remote_path, exit_code) =
        run_remote_command(&mut session, "mktemp -p '' rstream2.XXXXXXXX");
    let remote_path = remote_path.trim();
    assert_eq!(
        exit_code, 0,
        "Failed to create temporary file on remote host {}",
        host_id
    );
    debug!(
        "On host {} the executable will be copied at {}",
        host_id, remote_path
    );

    let current_exe = std::env::current_exe().unwrap();
    debug!(
        "Locally the executable is located at {}",
        current_exe.display()
    );

    send_file(
        host_id,
        &mut session,
        &current_exe,
        Path::new(&remote_path),
        0o500,
    );

    // build the remote command
    let command = build_remote_command(host_id, config_str.as_str(), remote_path);
    debug!("Executing on host {}:\n{}", host_id, command);

    let mut channel = session.channel_session().unwrap();
    channel.exec(&command).unwrap();

    // copy to stderr the output of the remote process
    let reader = BufReader::new(channel.stderr());
    for line in reader.lines() {
        if let Ok(line) = line {
            // prefix each line with the id of the host
            eprintln!("{}|{}", host_id, line);
        }
    }
    channel.wait_close().unwrap();
    info!("Exit status: {}", channel.exit_status().unwrap());

    debug!(
        "Removing temporary binary file at host {}: {}",
        host_id, remote_path
    );
    let remove_binary = format!(
        "rm -f {}",
        shell_escape::escape(Cow::Borrowed(&remote_path))
    );
    let (_, exit_code) = run_remote_command(&mut session, &remove_binary);
    assert_eq!(
        exit_code, 0,
        "Failed to remove remote executable on host {} at {}",
        host_id, remote_path
    );
}

/// Execute a command remotely and return the standard output and the exit code.
fn run_remote_command(session: &mut Session, command: &str) -> (String, i32) {
    debug!("Running remote command: {}", command);
    let mut channel = session.channel_session().unwrap();
    channel.exec(command).unwrap();
    let mut stdout = String::new();
    channel.read_to_string(&mut stdout).unwrap();
    channel.wait_close().unwrap();
    let exit_code = channel.exit_status().unwrap();
    (stdout, exit_code)
}

/// Send a file remotely via SCP and change its mode.
fn send_file(
    host_id: HostId,
    session: &mut Session,
    local_path: &Path,
    remote_path: &Path,
    mode: i32,
) {
    let metadata = local_path.metadata().unwrap();
    debug!(
        "Sending file to host {}: {} -> {}, {} bytes",
        host_id,
        local_path.display(),
        remote_path.display(),
        metadata.len()
    );
    let mut local_file = File::open(local_path).unwrap();
    let mut remote_file = session
        .scp_send(remote_path, mode, metadata.len(), None)
        .unwrap();
    let mut buffer = [0u8; SCP_BUFFER_SIZE];
    while let Ok(n) = local_file.read(&mut buffer) {
        if n == 0 {
            break;
        }
        remote_file.write_all(&buffer[0..n]).unwrap();
    }
    remote_file.send_eof().unwrap();
    remote_file.wait_eof().unwrap();
    remote_file.close().unwrap();
    remote_file.wait_close().unwrap();

    // setting the file mode using scp_send seems unreliable
    let chmod = format!(
        "chmod {:03o} {}",
        mode,
        shell_escape::escape(remote_path.to_string_lossy())
    );
    run_remote_command(session, &chmod);
}

/// Build the command for running the remote worker.
///
/// This will export all the required variables before executing the binary.
fn build_remote_command(host_id: HostId, config_str: &str, binary_path: &str) -> String {
    let config_str = shell_escape::escape(Cow::Borrowed(config_str));
    let args = std::env::args()
        .skip(1)
        .map(|arg| shell_escape::escape(Cow::Owned(arg)))
        .join(" ");
    format!(
        "export {host_id_env}={host_id};
export {config_env}={config};
export RUST_LOG={rust_log};
export RUST_LOG_STYLE=always;
{binary_path} {args}",
        host_id_env = HOST_ID_ENV_VAR,
        host_id = host_id,
        config_env = CONFIG_ENV_VAR,
        config = config_str,
        binary_path = binary_path,
        args = args,
        rust_log = std::env::var("RUST_LOG").unwrap_or_default()
    )
}
