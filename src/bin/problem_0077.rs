use projecteuler::partition;
use projecteuler::primes;

fn main() {
    //dbg!(binomial::binomial_coefficient(100, 50));
    dbg!(solve(4));
    dbg!(solve(5));
    dbg!(solve(6));
    dbg!(solve(100));
    dbg!(solve(5_000));
}

fn solve(n: usize) -> usize {
    let primes: Vec<usize> = primes::primes_iterator()
        .filter(|p| p.is_prime())
        .map(|p| p.get_prime())
        //since partition_into overflows for values higher than 416, we dont have to bother with them.
        .take_while(|p| *p <= 416)
        .collect();
    let mut i = 0;
    loop {
        i += 1;
        if partition::partition_into::<_, _, usize>(i, &primes) > n {
            return i;
        }
    }
}
