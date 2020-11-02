use num::BigUint;
use projecteuler::helper;

fn main() {
    helper::check_bench(|| {
        solve(100);
    });
    assert_eq!(solve(100), 272);
}

fn solve(mut n: usize) -> usize {
    //the problem starts counting with 1.
    n -= 1;
    //sadly even u128 overflows, so BigUint is needed
    let mut num = BigUint::from(0u8);
    let mut den = BigUint::from(1u8);
    for i in (0..n).rev() {
        let new_num = den.clone();
        let new_den = BigUint::from(get_continued_fraction_e(i)) * den + num;
        num = new_num;
        den = new_den;
    }
    num = num + den * BigUint::from(2u8);
    let num = format!("{}", num);
    num.as_bytes()
        .iter()
        .map(|b| (*b - 48) as usize)
        .sum::<usize>()
}

//1,2,1,1,4,1,...,1,2k,1
fn get_continued_fraction_e(i: usize) -> usize {
    let r = i % 3;
    if r == 0 || r == 2 {
        1
    } else {
        ((i / 3) + 1) * 2
    }
}
