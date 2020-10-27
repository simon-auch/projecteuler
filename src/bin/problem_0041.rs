use projecteuler::helper;
use projecteuler::primes;

fn main() {
    //gets optimized into a nop I think
    //helper::check_bench(|| {solve(1_000_000);});
    //assert_eq!(solve(), 0);
    //dbg!(is_pandigital(2143));
    dbg!(solve());
}

fn solve() -> usize {
    //since 1+2+3+4+5+6+7+8+9 divisible by 3 cannot be 9 pandigital
    //same goes for 8
    let primes = primes::sieve_bool(7654321);
    for (p, _) in primes.iter().enumerate().filter(|(_p, b)| **b).rev() {
        //dbg!(p);
        if is_pandigital(p) > 0 {
            return p;
        }
    }
    0
}

fn is_pandigital(n: usize) -> usize {
    let mut digits = [false; 9];
    let mut max_digit = 0;
    for d in helper::digits_iterator(n) {
        let d = d as usize;
        if d == 0 {
            return 0;
        }
        if d > max_digit {
            max_digit = d;
        }
        let d = d - 1;
        if digits[d] {
            return 0;
        }
        digits[d] = true;
    }
    if digits[..max_digit].iter().all(|b| *b) {
        max_digit
    } else {
        0
    }
}
