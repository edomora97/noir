#[macro_use]
extern crate log;

use async_std::stream::from_iter;

use rstream::config::EnvironmentConfig;
use rstream::environment::StreamEnvironment;
use rstream::operator::source;

#[async_std::main]
async fn main() {
    env_logger::init();

    let config = EnvironmentConfig::local(4);
    // let config = EnvironmentConfig::remote("config.yml").await.unwrap();
    let mut env = StreamEnvironment::new(config);

    env.spawn_remote_workers().await;

    let source = source::StreamSource::new(from_iter(90..110u8));
    let stream = env
        .stream(source)
        .map(|x| x.to_string())
        .shuffle()
        .group_by(|s| s.len())
        .map(|(_k, s)| s.len())
        .unkey();
    let result = stream.collect_vec();

    env.execute().await;

    info!("Output: {:?}", result.get());
}
