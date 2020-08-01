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
        .nth(10001 - 1));
}
