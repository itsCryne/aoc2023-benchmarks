use aoc_macros::benchmark;
use criterion::{criterion_group, criterion_main, Criterion};

benchmark!();
criterion_group!(aoc_benchmark, benchmark);
criterion_main!(aoc_benchmark);
