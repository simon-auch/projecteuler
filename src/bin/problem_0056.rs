use num::BigUint;
use projecteuler::helper;

fn main() {
    helper::check_bench(|| {
        solve(100);
    });
    assert_eq!(solve(100), 972);
    dbg!(solve(100));
}

fn solve(n: usize) -> usize {
    let mut max = 0;
    for i in 0..n {
        //dbg!(i);
        let i = BigUint::from(i);
        for j in 0..n {
            let mut m = i.pow(j as u32);
            //println!("{}, {}, {}", i, j, m);
            let mut digitsum = 0usize;

            while m > BigUint::from(0u8) {
                digitsum += num::ToPrimitive::to_u8(&(&m % BigUint::from(10u8))).unwrap() as usize;
                m /= BigUint::from(10u8);
            }

            if digitsum > max {
                max = digitsum;
            }
        }
    }
    max
}
