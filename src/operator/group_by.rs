use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;

use crate::block::NextStrategy;
use crate::operator::key_by::KeyBy;
use crate::operator::Operator;
use crate::stream::{KeyValue, KeyedStream, Stream};

pub(crate) type Keyer<Key, Out> = Arc<dyn Fn(&Out) -> Key + Send + Sync>;

impl<In, Out, OperatorChain> Stream<In, Out, OperatorChain>
where
    In: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    OperatorChain: Operator<Out> + Send + 'static,
{
    pub fn group_by<Key, Keyer>(
        mut self,
        keyer: Keyer,
    ) -> KeyedStream<Out, Key, Out, impl Operator<KeyValue<Key, Out>>>
    where
        Key: Clone + Serialize + DeserializeOwned + Send + Sync + Hash + Eq + 'static,
        Keyer: Fn(&Out) -> Key + Send + Sync + 'static,
    {
        let keyer = Arc::new(keyer);
        let keyer2 = keyer.clone();
        self.block.next_strategy = NextStrategy::GroupBy(Arc::new(move |out| {
            let mut s = DefaultHasher::new();
            keyer2(out).hash(&mut s);
            s.finish() as usize
        }));
        let new_stream = self
            .add_block()
            .add_operator(|prev| KeyBy::new(prev, keyer));
        KeyedStream(new_stream)
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use tokio::stream::from_iter;

    use crate::config::EnvironmentConfig;
    use crate::environment::StreamEnvironment;
    use crate::operator::source;

    #[tokio::test]
    async fn group_by_stream() {
        let mut env = StreamEnvironment::new(EnvironmentConfig::local(4));
        let source = source::StreamSource::new(from_iter(0..100u8));
        let res = env
            .stream(source)
            .group_by(|&n| n.to_string().chars().next().unwrap())
            .unkey()
            .collect_vec();
        env.execute().await;
        let res = res.get().unwrap().into_iter().sorted().collect_vec();
        let expected = (0..100u8)
            .map(|n| (n.to_string().chars().next().unwrap(), n))
            .sorted()
            .collect_vec();
        assert_eq!(res, expected);
    }
}
