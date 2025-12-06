use aoc_2025_04::{load_file, solve, solve_part1, solve_part2};
use criterion::{Criterion, criterion_group, criterion_main};

fn bench_solve(c: &mut Criterion) {
    c.bench_function("solve input.txt", |b| b.iter(|| solve("data/input.txt")));
}

fn bench_load(c: &mut Criterion) {
    c.bench_function("input", |b| b.iter(|| load_file("data/input.txt")));
}

fn bench_solve_part1(c: &mut Criterion) {
    c.bench_function("Part 1 only", |b| {
        b.iter(|| {
            assert_eq!(solve_part1(&mut load_file("data/input.txt")), 1384);
        })
    });
}

fn bench_solve_part2(c: &mut Criterion) {
    c.bench_function("Part 2 only", |b| {
        b.iter(|| assert_eq!(solve_part2(&mut load_file("data/input.txt")), 8013))
    });
}

criterion_group! {
name = benches;
config= Criterion::default();
targets= bench_solve, bench_load, bench_solve_part1, bench_solve_part2}

criterion_main!(benches);
