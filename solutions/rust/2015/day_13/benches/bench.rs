#[macro_use]
extern crate criterion;

use aoclib::Solvable;
use criterion::Criterion;

fn part_one_benchmark(c: &mut Criterion) {
    c.bench_function("2015 day 13 part one", |b| {
        let input = aoclib::reader(2015, 13, "input.txt").unwrap();
        b.iter(|| aoc_2015_13::PartOne::solve(&input).unwrap())
    });
}

fn part_two_benchmark(c: &mut Criterion) {
    c.bench_function("2015 day 13 part two", |b| {
        let input = aoclib::reader(2015, 13, "input.txt").unwrap();
        b.iter(|| aoc_2015_13::PartTwo::solve(&input).unwrap())
    });
}

criterion_group!(benches, part_one_benchmark, part_two_benchmark);
criterion_main!(benches);
