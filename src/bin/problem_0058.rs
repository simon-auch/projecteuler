use projecteuler::helper;
use projecteuler::primes;

fn main() {
    helper::check_bench(|| {
        solve();
    });
    assert_eq!(solve(), 26241);
    dbg!(solve());
}

// see problem 28
fn solve() -> usize {
    let mut total = 5;
    let mut acc = 3;

    let mut last = 9;
    let mut stepsize = 4;
    while acc * 100 >= total * 10 {
        //the last corner is a cube and therefore not prime
        for _i in 1..=3 {
            last += stepsize;
            if primes::is_prime(last) {
                //dbg!(last);
                acc += 1;
            }
        }
        last += stepsize;
        total += 4;
        stepsize += 2;
    }
    stepsize -= 2;
    stepsize + 1
}
