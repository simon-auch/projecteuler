use projecteuler::helper;

/*
Find the smallest positive integer, x, such that 2x, 3x, 4x, 5x, and 6x, contain the same digits

Note:
x must start with the digit 1. Suppose it starts with a digit >1 => 6x would contain more digits than x.


*/

fn main() {
    helper::check_bench(|| {
        solve();
    });
    dbg!(solve());
}

fn solve() -> usize {
    for exp in 5.. {
        let ten = 10usize.pow(exp as u32);
        for i in 0..ten {
            let i = ten + i;
            let mut digits = helper::digits(i);
            digits.sort();
            for jf in 2..=6 {
                let j = i * jf;
                let mut digits_j = helper::digits(j);
                digits_j.sort();
                if digits != digits_j {
                    break;
                } else if jf == 6 {
                    return i;
                }
            }
        }
    }
    unreachable!();
}
