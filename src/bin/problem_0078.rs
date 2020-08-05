#[path = "../partition.rs"]
mod partition;
#[path = "../primes.rs"]
mod primes;

use num::BigUint;

fn main() {
    //dbg!(binomial::binomial_coefficient(100, 50));
    dbg!(primes::factorize(1_000_000));
    for i in 1..10 {
        dbg!(i, partition::partition(i));
    }
    dbg!(solve(5));
    dbg!(solve(2));
    dbg!(solve(100));
    dbg!(solve(1_000));
    dbg!(solve(10_000));
    dbg!(solve(100_000));
    dbg!(solve(1_000_000));
}

fn solve(n: usize) -> usize {
    let mut i = 0;
    partition::PartitionIterator::<BigUint>::new()
        .enumerate()
        .filter(|(i, part)| {
            if i % 100 == 0 {
                dbg!(i);
            }
            part % BigUint::from(n) == BigUint::from(0usize)
        })
        .nth(0)
        .unwrap()
        .0
        + 1
}
