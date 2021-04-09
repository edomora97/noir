use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

use std::sync::Arc;

use crate::block::NextStrategy;
use crate::operator::{DataKey, EndBlock};
use crate::operator::{KeyBy, Operator};
use crate::stream::{KeyValue, KeyedStream, Stream};

pub type Keyer<Key, Out> = Arc<dyn Fn(&Out) -> Key + Send + Sync>;

impl<OperatorChain> Stream<OperatorChain>
where
    OperatorChain: Operator + Send + 'static,
{
    pub fn group_by<Key: DataKey, Keyer>(
        self,
        keyer: Keyer,
    ) -> KeyedStream<impl Operator<Out = KeyValue<Key, OperatorChain::Out>>>
    where
        Keyer: Fn(&OperatorChain::Out) -> Key + Send + Sync + 'static,
    {
        let keyer = Arc::new(keyer);
        let keyer2 = keyer.clone();

        let next_strategy = NextStrategy::GroupBy(Arc::new(move |out| {
            let mut s = DefaultHasher::new();
            keyer2(out).hash(&mut s);
            s.finish() as usize
        }));

        let new_stream = self
            .add_block(EndBlock::new, next_strategy)
            .add_operator(|prev| KeyBy::new(prev, keyer));
        KeyedStream(new_stream)
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::config::EnvironmentConfig;
    use crate::environment::StreamEnvironment;
    use crate::operator::source;

    #[test]
    fn group_by_stream() {
        let mut env = StreamEnvironment::new(EnvironmentConfig::local(4));
        let source = source::IteratorSource::new(0..100u8);
        let res = env
            .stream(source)
            .group_by(|&n| n.to_string().chars().next().unwrap())
            .unkey()
            .collect_vec();
        env.execute();
        let res = res.get().unwrap().into_iter().sorted().collect_vec();
        let expected = (0..100u8)
            .map(|n| (n.to_string().chars().next().unwrap(), n))
            .sorted()
            .collect_vec();
        assert_eq!(res, expected);
    }
}
