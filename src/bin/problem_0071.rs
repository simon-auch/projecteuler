#![feature(test)]

use fraction::Fraction;
use projecteuler::fraction;
use projecteuler::helper;

fn main() {
    dbg!(solve(8));
    dbg!(solve(1_000_000));
    helper::time_it(
        || {
            core::hint::black_box(solve(1_000_000));
        },
        100,
    );
}

fn solve(d: usize) -> Fraction {
    let mut low = Fraction::new(0, 1);
    let high = Fraction::new(3, 7);
    for i in 3..=d {
        //dbg!(i);
        //dbg!(low);
        /*
        low < a/i
        a > low*i

        a/i < high
        a < high*i
        */
        let max = Fraction::new(high.num() * i, high.den());
        //dbg!(max);
        let a = max.ceil() - 1;
        let new_low = Fraction::new(a, i);
        //dbg!(new_low);
        if new_low > low {
            low = new_low;
        }
    }
    low
}
