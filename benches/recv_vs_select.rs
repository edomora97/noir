use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

const CHANNEL_CAPACITY: usize = 10;

fn crossbeam_run(size: usize, body: impl FnOnce(crossbeam_channel::Receiver<usize>)) {
    let (sender, receiver) = crossbeam_channel::bounded(CHANNEL_CAPACITY);
    let join_handle = std::thread::Builder::new()
        .name("recv producer".to_string())
        .spawn(move || {
            for i in 0..size {
                sender.send(i).unwrap();
            }
        })
        .unwrap();
    body(receiver);
    join_handle.join().unwrap();
}

fn crossbeam_recv(size: usize) {
    crossbeam_run(size, |recv| while recv.recv().is_ok() {});
}

fn crossbeam_select(size: usize) {
    crossbeam_run(size, |recv| {
        for _ in 0..size {
            let mut select = crossbeam_channel::Select::new();
            select.recv(&recv);
            let _ = select.ready();
            recv.recv().unwrap();
        }
    });
}

fn flume_run(size: usize, body: impl FnOnce(flume::Receiver<usize>)) {
    let (sender, receiver) = flume::bounded(CHANNEL_CAPACITY);
    let join_handle = std::thread::Builder::new()
        .name("recv producer".to_string())
        .spawn(move || {
            for i in 0..size {
                sender.send(i).unwrap();
            }
        })
        .unwrap();
    body(receiver);
    join_handle.join().unwrap();
}

fn flume_recv(size: usize) {
    flume_run(size, |recv| while recv.recv().is_ok() {});
}

fn flume_select(size: usize) {
    flume_run(size, |recv| {
        for _ in 0..size {
            flume::Selector::new().recv(&recv, |a| a).wait().unwrap();
        }
    });
}

fn recv_vs_select_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("recv_vs_select");
    const ELEMENTS: usize = 100_000;
    group.throughput(Throughput::Elements(ELEMENTS as u64));
    group.bench_function("crossbeam_recv", |b| {
        b.iter(|| crossbeam_recv(black_box(ELEMENTS)))
    });
    group.bench_function("crossbeam_select", |b| {
        b.iter(|| crossbeam_select(black_box(ELEMENTS)))
    });
    group.bench_function("flume_recv", |b| b.iter(|| flume_recv(black_box(ELEMENTS))));
    group.bench_function("flume_select", |b| {
        b.iter(|| flume_select(black_box(ELEMENTS)))
    });
    group.finish();
}

criterion_group!(benches, recv_vs_select_benchmark);
criterion_main!(benches);
