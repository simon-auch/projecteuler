#[path = "../primes.rs"]
mod primes;
#[path = "../helper.rs"]
mod helper;

fn main() {
    helper::check_bench(|| {solve(1_000_000);});
    assert_eq!(solve(1_000_000), 55);
    dbg!(solve(1_000_000));
}

fn solve(n: usize) -> usize {
    let primes = primes::sieve_bool(n);
    primes
    .iter()
    .enumerate()
    .skip(2)
    .filter(|&(i,p)| *p && is_circular_prime(&primes,i))
    .count()
}

fn is_circular_prime(sieve_bool: &[bool], mut p: usize) -> bool{
    let len = helper::digits_iterator(p).count();
    let mult = 10usize.pow((len-1) as u32);
    for _ in 0..len-1{
        let d = p%10;
        p /= 10;
        p += mult * d;
        if !sieve_bool[p] {
            return false
        }
    }
    true
}
