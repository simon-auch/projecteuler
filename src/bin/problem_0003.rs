#[path = "../primes.rs"]
mod primes;

fn main() {
    println!("{:?}", primes::factorize(600_851_475_143));
}
