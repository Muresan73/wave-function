#[path = "../src/node.rs"]
mod node;
use criterion::{criterion_group, criterion_main, Criterion};
use node::bench;
fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("wave funcion search", |b| b.iter(bench));
}

criterion_group!(
  name = benches;
  config = Criterion::default().significance_level(0.1).sample_size(30);
  targets = criterion_benchmark
);
criterion_main!(benches);
