use projecteuler::primes;

fn main() {
    dbg!(solve_1(10));
    //dbg!(solve_1(10_000));
    dbg!(solve_2(10_000));
}

fn solve_1(n: usize) -> usize {
    let mut sum = 0;
    for a in 1..n {
        let b = sum_divisors_1(a);
        if a < b && a == sum_divisors_1(b) {
            sum += a + b;
        }
    }
    sum
}

fn sum_divisors_1(n: usize) -> usize {
    let mut sum = 0;
    for i in 1..n {
        if n % i == 0 {
            sum += i;
        }
    }
    sum
}

fn solve_2(n: usize) -> usize {
    let sieve = primes::sieve_prime_biggest(n);
    let mut sum = 0;
    for a in 1..n {
        let b = primes::sum_of_divisors(a, &sieve);
        if b < a && a == primes::sum_of_divisors(b, &sieve) {
            sum += a + b;
        }
    }
    sum
}
