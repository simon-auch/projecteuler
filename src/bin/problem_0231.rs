use projecteuler::helper;
use projecteuler::primes;

fn main() {
    //dbg!(solve(10, 3));
    //dbg!(solve(20, 15));
    //dbg!(solve(20_000, 15_000));
    ////dbg!(solve(20_000_000, 15_000_000));
    //dbg!(solve_2(10, 3));
    //dbg!(solve_2(20, 15));
    //dbg!(solve_2(20_000, 15_000));
    //dbg!(solve_2(20_000_000, 15_000_000));
    //dbg!(solve_3(20_000_000, 15_000_000));
    //dbg!(solve_3(200_000_000, 150_000_000));
    for exp in 1..10 {
        let n = 2 * 10usize.pow(exp);
        let k = n * 3 / 4;
        dbg!(exp, n, k);
        helper::time_it(
            || {
                dbg!(solve_3(n, k));
            },
            1,
        );
    }
}

//should work but really slow because it factorizes quite a lot of numbers repeatedly
#[allow(dead_code)]
fn solve(n: usize, k: usize) -> usize {
    let mut sum = 0;
    for i in 1..=k {
        let v = n - k + i;
        let factorization = primes::factorize(v);
        //dbg!(&factorization);
        sum += factorization.iter().sum::<usize>();
        let v = i;
        let factorization = primes::factorize(v);
        //dbg!(&factorization);
        sum -= factorization.iter().sum::<usize>();
    }
    sum
}

//same as solve, but only factorizes numbers once
#[allow(dead_code)]
fn solve_2(n: usize, mut k: usize) -> usize {
    if k > n / 2 {
        k = n - k;
    }
    let sieve = primes::SieveDivisor::new(n + 1);
    let sum_of_prime_factors = |v: usize| {
        let mut sum = 0;
        sieve.for_each_divisor(v, |p, exp| sum += p * exp as usize);
        sum
    };
    let mut sum = 0;
    for i in 1..=k {
        sum += sum_of_prime_factors(n - k + i);
        sum -= sum_of_prime_factors(i);
    }
    sum
}

//faster approach
/// nCk = n!/(k!*(n-k)!)
/// Sei S(x) die Summe der Primfaktoren von x
/// S(nCk) = S(n!) - S(k!) - S((n-k)!)
///
/// How to compute S(n!)
/// S(n!) = \sum_{p <= n} E(n!,p) * p
/// where E(n,p) gives the number of times p divides n
///
/// E(n!,p) = \sum_{e=1} e * (floor(n/p^e) - floor(n/p^{e+1}))
/// E(n!,p) = \sum_{e=1} floor(n/p^e)
/// which can be further improved by dividing n through p
///
fn solve_3(n: usize, k: usize) -> usize {
    let primes = primes::primes(n + 1);
    let e = |mut n: usize, p: usize| {
        let mut sum = 0;
        while n >= p {
            n /= p;
            sum += n;
        }
        sum
    };
    let s = |n: usize| primes.iter().map(|p| e(n, *p) * p).sum::<usize>();
    s(n) - s(k) - s(n - k)
}
