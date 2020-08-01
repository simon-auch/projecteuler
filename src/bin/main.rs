#[path = "../primes.rs"]
mod primes;

fn main() {
    for (i, p) in (2..100).zip(primes::primes_iterator()) {
        println!("{}: {:?}", i, p);
    }
    println!("{:?}", primes::primes(100));
    println!("{:?}", primes::sieve_bool(10));
    println!("{:?}", primes::sieve_prime_biggest(10));
    println!("Hello, world!");
}
