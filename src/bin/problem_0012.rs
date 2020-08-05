#[path = "../primes.rs"]
mod primes;

fn main() {
    dbg!(solve(5));
    dbg!(solve(500));
}

//this could be improved by using current = n(n+1)/2
// and then factorizing n, n+1, join the factors and remove a factor 2.

fn solve(n: usize) -> usize {
    let mut current = 1;
    for i in 2.. {
        current += i;
        let factors = primes::factorize(current);
        let mut prod = 1;
        let mut p_exp = 0;
        for j in 0..factors.len() {
            p_exp += 1;
            if j + 1 < factors.len() && factors[j] == factors[j + 1] {
            } else {
                prod *= p_exp + 1;
                p_exp = 0;
            }
        }
        if prod > n {
            return current;
        }
    }
    panic!();
}
