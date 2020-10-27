use num::ToPrimitive;
use num::Zero;
use num::{BigUint, Integer};

use projecteuler::factorial;

fn main() {
    dbg!(solve(100));
    dbg!(solve(10));
}

fn solve(n: usize) -> usize {
    let mut sum = 0;
    let mut prod: BigUint = factorial::factorial(n);
    dbg!(&prod);
    while prod > BigUint::zero() {
        let (new_prod, rem) = prod.div_rem(&BigUint::from(10usize));
        sum += rem.to_usize().unwrap();
        prod = new_prod;
    }
    sum
}
