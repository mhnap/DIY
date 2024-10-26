// https://medium.com/@datenlord/asynchronous-runtime-io-problem-analysis-9bb4744199d7

use bytes::{Bytes, BytesMut};
use criterion::{criterion_group, criterion_main};
use criterion::{BenchmarkId, Criterion};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::hint::black_box;
use std::io::{Read as _, Write};
use std::sync::mpsc;
use std::time::Duration;
use tokio::io::AsyncWriteExt;

const NUM_WRITES: usize = 10;
const WRITE_SIZE: usize = 256 * 100;
const NUM_THREAD: usize = 1;
const PATH: &str = "fs_write.log";

fn build_runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(NUM_THREAD)
        .enable_all()
        .build()
        .unwrap()
}

fn build_runtime_overload() -> tokio::runtime::Runtime {
    let rt = build_runtime();
    const INTERVAL_EACH_RUN: Duration = Duration::from_millis(1);
    const NUM_TASK: usize = 1000;

    for _ in 0..NUM_TASK {
        rt.spawn(async move {
            loop {
                black_box(cpu_task());
                tokio::time::sleep(INTERVAL_EACH_RUN).await;
            }
        });
    }

    rt
}

struct TokioUringRuntime {
    rt: tokio_uring::Runtime,
}

impl TokioUringRuntime {
    fn new() -> Self {
        Self { rt: tokio_uring::Runtime::new(&tokio_uring::builder()).unwrap() }
    }
}

impl criterion::async_executor::AsyncExecutor for TokioUringRuntime {
    fn block_on<T>(&self, future: impl std::future::Future<Output = T>) -> T {
        self.rt.block_on(future)
    }
}

impl criterion::async_executor::AsyncExecutor for &TokioUringRuntime {
    fn block_on<T>(&self, future: impl std::future::Future<Output = T>) -> T {
        self.rt.block_on(future)
    }
}

fn build_uring_runtime() -> TokioUringRuntime {
    TokioUringRuntime::new()
}

fn cpu_task() -> Vec<i64> {
    let mut rng = StdRng::from_seed([0; 32]);
    let mut v: Vec<_> = std::iter::repeat_with(|| rng.gen::<i64>()).take(100).collect();
    v.sort();
    v
}

type Task = Box<dyn FnOnce() + Send + 'static>;

struct Thread {
    task_tx: Option<mpsc::Sender<(Task, mpsc::Sender<()>)>>,
    handle: Option<std::thread::JoinHandle<()>>,
}

impl Thread {
    fn init() -> Self {
        let (tx, rx) = mpsc::channel();
        let handle = std::thread::spawn(move || loop {
            let Some((task, fin_tx)): Option<(Task, mpsc::Sender<()>)> = rx.recv().ok() else {
                break;
            };
            fin_tx.send(task()).unwrap();
        });
        Self { task_tx: Some(tx), handle: Some(handle) }
    }

    fn spawn_to_thread(&self, task: Box<dyn FnOnce() + Send + 'static>) {
        let (tx, rx) = mpsc::channel();
        self.task_tx.as_ref().unwrap().send((task, tx)).unwrap();
        rx.recv().unwrap();
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        drop(self.task_tx.take());
        self.handle.take().unwrap().join().unwrap();
    }
}

async fn fs_write_async() {
    let mut file =
        tokio::fs::OpenOptions::new().create(true).append(true).open(PATH).await.unwrap();
    for _ in 0..NUM_WRITES {
        file.write_all(&[0; WRITE_SIZE]).await.unwrap();
        file.sync_data().await.unwrap();
    }
    tokio::fs::remove_file(PATH).await.unwrap();
}

async fn fs_write_async_uring() {
    let file =
        tokio_uring::fs::OpenOptions::new().create(true).append(true).open(PATH).await.unwrap();
    for _ in 0..NUM_WRITES {
        file.write_all_at(Bytes::copy_from_slice(&[0; WRITE_SIZE]), 0).await.0.unwrap();
        file.sync_data().await.unwrap();
    }
    tokio_uring::fs::remove_file(PATH).await.unwrap();
}

async fn fs_write() {
    let mut file = std::fs::OpenOptions::new().create(true).append(true).open(PATH).unwrap();
    for _ in 0..NUM_WRITES {
        file.write_all(&[0; WRITE_SIZE]).unwrap();
        file.sync_data().unwrap();
    }
    std::fs::remove_file(PATH).unwrap();
}

fn fs_write_thread() {
    let mut file = std::fs::OpenOptions::new().create(true).append(true).open(PATH).unwrap();
    let mut buf = BytesMut::with_capacity(WRITE_SIZE);
    for _ in 0..NUM_WRITES {
        file.write_all(&[0; WRITE_SIZE]).unwrap();
        file.sync_data().unwrap();
        file.read_exact(&mut buf).unwrap();
    }
    std::fs::remove_file(PATH).unwrap();
}

fn fs_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("fs_benchmarks");

    let thread = Thread::init();
    group.bench_function("fs_write_thread", |b| {
        b.iter(|| thread.spawn_to_thread(Box::new(fs_write_thread)))
    });

    let rts = vec![(build_runtime(), "noload"), (build_runtime_overload(), "stress")];
    for (rt, name) in &rts {
        group.bench_function(BenchmarkId::new("fs_write", name), |b| {
            b.to_async(rt).iter(|| fs_write())
        });
        group.bench_function(BenchmarkId::new("fs_write_async", name), |b| {
            b.to_async(rt).iter(|| fs_write_async())
        });
    }

    let uring_rt = build_uring_runtime();
    group.bench_function(BenchmarkId::new("fs_write", "uring"), |b| {
        b.to_async(&uring_rt).iter(|| fs_write())
    });
    group.bench_function(BenchmarkId::new("fs_write_async_uring", "uring"), |b| {
        b.to_async(&uring_rt).iter(|| fs_write_async_uring())
    });

    group.finish();
}

criterion_group!(benches, fs_benchmark);
criterion_main!(benches);
