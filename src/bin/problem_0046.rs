use projecteuler::helper;
use projecteuler::primes;

fn main() {
    //gets optimized into a nop I think
    helper::check_bench(|| {
        solve();
    });
    assert_eq!(solve(), 5777);
    dbg!(solve());
}

fn solve() -> usize {
    let mut acc_p = Vec::new();
    let prime_iter = primes::primes_iterator().enumerate();
    for (i, p_or_f) in prime_iter {
        let i = i + 2;
        //i is even
        if i & 0b1 == 0 {
            continue;
        }
        match p_or_f {
            primes::PrimeOrFactor::Prime(p) => {
                acc_p.push(p);
                continue;
            }
            primes::PrimeOrFactor::Factor(_) => {
                if !sum_of_prime_and_doubled_square(i, &acc_p) {
                    dbg!(i);
                }
            }
        }
    }
    unreachable!();
}

fn sum_of_prime_and_doubled_square(n: usize, primes: &[usize]) -> bool {
    let mut j = primes.len();
    let mut i = 0;
    while j > 0 {
        let temp = i * i * 2 + primes[j - 1];
        if temp > n {
            j -= 1;
        } else if temp < n {
            i += 1;
        } else {
            return true;
        }
    }
    false
}
