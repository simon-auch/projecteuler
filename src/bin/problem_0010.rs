#[path = "../primes.rs"]
mod primes;

fn main() {
    dbg!(primes::primes_iterator()
        .filter(|x| {
            match x {
                primes::PrimeOrFactor::Prime(_) => true,
                _ => false,
            }
        })
        .map(|x| match x {
            primes::PrimeOrFactor::Prime(p) => p,
            _ => unreachable!(),
        })
        .take_while(|x| *x < 2_000_000usize)
        .sum::<usize>());
}
