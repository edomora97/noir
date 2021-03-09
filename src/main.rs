#[macro_use]
extern crate log;

use tokio_stream::iter;

use rstream::config::EnvironmentConfig;
use rstream::environment::StreamEnvironment;
use rstream::operator::source;
use std::time::Instant;

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = EnvironmentConfig::local(4);
    // let config = EnvironmentConfig::remote("config.yml").await.unwrap();
    let mut env = StreamEnvironment::new(config);

    env.spawn_remote_workers().await;

    let source = source::StreamSource::new(iter(90..110u8));
    let stream = env
        .stream(source)
        .map(|x| x.to_string())
        .shuffle()
        .group_by(|s| s.len())
        .map(|(_k, s)| s.len())
        .unkey();
    let result = stream.collect_vec();

    let start = Instant::now();
    env.execute().await;
    let duration = start.elapsed();

    // info!("Output: {:?}", result.get());
    info!("Elapsed: {:?}", duration);
}
