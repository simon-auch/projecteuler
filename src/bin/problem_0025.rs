use num::BigUint;

fn main() {
    dbg!(solve(3));
    dbg!(solve(1000));
}

fn solve(n: usize) -> usize {
    let min = BigUint::from(10usize).pow(n as u32 - 1);

    let mut i = 2;
    let mut a = BigUint::from(1usize);
    let mut b = BigUint::from(1usize);
    while b < min {
        i += 1;
        let mut c = a;
        c += &b;
        a = b;
        b = c;
    }
    i
}
