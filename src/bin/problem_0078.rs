use projecteuler::partition;
use projecteuler::primes;

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
    //dbg!(solve(10_000));
    //dbg!(solve(100_000));
    //dbg!(solve(1_000_000));
    dbg!(solve2(1_000));
    dbg!(solve2(1_000_000));
}

fn solve(n: usize) -> usize {
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

//basically the same as solve, just changed the Iterator to use modulo
fn solve2(n: usize) -> usize {
    let mut vec: Vec<usize> = Vec::new();
    vec.push(1);
    while *(vec.last().unwrap()) != 0 {
        let mut next = 0isize;
        for i in 1.. {
            let j = if i % 2 == 1 {
                (i as isize + 1) / 2
            } else {
                -(i as isize + 1) / 2
            };
            let pentagonal = (j * (3 * j - 1) / 2) as usize;
            //dbg!(pentagonal);
            if pentagonal > vec.len() {
                break;
            }
            let val = vec[vec.len() - pentagonal];
            let exp = (i + 1) / 2;
            if exp & 0b1 == 0 {
                next -= val as isize;
            } else {
                next += val as isize;
            }
        }
        //dbg!(next);
        vec.push((next % n as isize) as usize);
    }
    vec.len() - 1
}
