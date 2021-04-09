use crate::operator::window::generic_operator::GenericWindowOperator;
use crate::operator::{Data, DataKey, Operator, WindowDescription};
use crate::stream::{KeyValue, KeyedStream, WindowedStream};

impl<Key: DataKey, Out: Data + Ord, WindowDescr, OperatorChain>
    WindowedStream<Key, Out, OperatorChain, WindowDescr>
where
    WindowDescr: WindowDescription<Key, Out> + Clone + 'static,
    OperatorChain: Operator<Out = KeyValue<Key, Out>> + Send + 'static,
{
    pub fn min(self) -> KeyedStream<Key, Out, impl Operator<Out = KeyValue<Key, Out>>> {
        let stream = self.inner;
        let descr = self.descr;

        stream.add_operator(|prev| {
            GenericWindowOperator::new("Min", prev, descr, |window| {
                window.items().min().unwrap().clone()
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::config::EnvironmentConfig;
    use crate::environment::StreamEnvironment;
    use crate::operator::{source, CountWindow};

    #[test]
    fn test_min_window() {
        let mut env = StreamEnvironment::new(EnvironmentConfig::local(4));
        let source = source::IteratorSource::new(vec![2, 3, 0, 1, 7, 4, 5, 2, 6].into_iter());
        let res = env
            .stream(source)
            .group_by(|x| x % 2)
            .window(CountWindow::new(3, 2))
            .min()
            .unkey()
            .map(|(_, x)| x)
            .collect_vec();
        env.execute();
        let mut res = res.get().unwrap();
        res.sort_unstable();
        assert_eq!(res, vec![0, 1, 2, 5, 6]);
    }
}
