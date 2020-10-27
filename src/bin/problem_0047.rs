use projecteuler::helper;
use projecteuler::primes;

fn main() {
    //gets optimized into a nop I think
    helper::check_bench(|| {
        solve(4);
    });
    assert_eq!(solve(4), 134043);
    dbg!(solve(2));
    dbg!(solve(3));
    dbg!(solve(4));
}

fn solve(n: usize) -> usize {
    let mut primes = primes::sieve_prime_biggest(1_000);
    let mut i = 1;
    let mut acc = 0;
    while acc != n {
        i += 1;
        if i >= primes.len() {
            primes = primes::sieve_prime_biggest(primes.len() * 2);
        }
        if primes[i] == i {
            acc = 0;
        } else {
            //count distinct prime factors
            let mut i = i;
            let mut j = 1;
            while primes[i] != i {
                if primes[i] != primes[i / primes[i]] {
                    j += 1;
                }
                i /= primes[i];
            }
            if j >= n {
                acc += 1;
            } else {
                acc = 0;
            }
        }
    }
    i - n + 1
}
