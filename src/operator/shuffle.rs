use crate::block::NextStrategy;
use crate::operator::{EndBlock, Operator};
use crate::stream::Stream;

impl<OperatorChain> Stream<OperatorChain>
where
    OperatorChain: Operator + Send + 'static,
{
    pub fn shuffle(self) -> Stream<impl Operator<Out = OperatorChain::Out>> {
        self.add_block(EndBlock::new, NextStrategy::Random)
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::config::EnvironmentConfig;
    use crate::environment::StreamEnvironment;
    use crate::operator::source;

    #[test]
    fn shuffle_stream() {
        let mut env = StreamEnvironment::new(EnvironmentConfig::local(4));
        let source = source::IteratorSource::new(0..1000u16);
        let res = env
            .stream(source)
            .shuffle()
            .shuffle()
            .shuffle()
            .shuffle()
            .shuffle()
            .collect_vec();
        env.execute();
        let res = res.get().unwrap();
        let res_sorted = res.clone().into_iter().sorted().collect_vec();
        let expected = (0..1000u16).collect_vec();
        assert_eq!(res_sorted, expected);
        assert_ne!(
            res, expected,
            "It's very improbable that going to the shuffles the result is sorted"
        );
    }
}
