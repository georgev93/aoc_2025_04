use aoc_2025_04::{load_file, solve, solve_part1, solve_part2};
use criterion::{Criterion, criterion_group, criterion_main};

fn bench_solve(c: &mut Criterion) {
    c.bench_function("solve input.txt", |b| b.iter(|| solve("data/input.txt")));
}

fn bench_load(c: &mut Criterion) {
    c.bench_function("input", |b| b.iter(|| load_file("data/input.txt")));
}

fn bench_solve_part1(c: &mut Criterion) {
    let input_file = load_file("data/input.txt");
    c.bench_function("input", move |b| b.iter(|| solve_part1(input_file.clone())));
}

fn bench_solve_part2(c: &mut Criterion) {
    let input_file = load_file("data/input.txt");
    c.bench_function("input", move |b| b.iter(|| solve_part2(input_file.clone())));
}

criterion_group! {name = benches; config= Criterion::default(); targets= bench_solve, bench_load, bench_solve_part1, bench_solve_part2}
criterion_main!(benches);
