#[path = "../primes.rs"]
mod primes;

use core::ops::Range;

fn main() {
    dbg!(solve(2..6, 2..6));
    dbg!(solve(2..101, 2..101));
}

fn solve(a: Range<usize>, b: Range<usize>) -> usize {
    let mut acc: Vec<Vec<(usize, usize)>> = Vec::new();
    for a in a {
        let factorization = primes::factorize_count(a);
        for b in b.clone() {
            acc.push(factorization.iter().map(|x| (x.0, x.1 * b)).collect());
        }
    }
    acc.sort();
    acc.dedup();
    acc.len()
}
