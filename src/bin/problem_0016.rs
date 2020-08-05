fn main() {
    dbg!(solve(15));
    dbg!(solve(1000));
}

fn solve(n: usize) -> usize {
    use num::BigUint;
    use num::Integer;
    use num::ToPrimitive;
    let mut n = BigUint::from(2usize).pow(n as u32);
    let mut sum = 0;
    while n >= BigUint::from(1usize) {
        let (new_n, rem) = n.div_rem(&BigUint::from(10usize));
        sum += rem.to_usize().unwrap();
        n = new_n;
    }
    sum
}
