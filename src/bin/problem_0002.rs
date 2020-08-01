fn main() {
    let mut sum = 0;
    let mut a = 1;
    let mut b = 1;
    while b <= 4_000_000 {
        let c = a + b;
        a = b;
        b = c;

        if b % 2 == 0 {
            sum += b;
        }
    }
    if b % 2 == 0 {
        sum -= b;
    }
    println!("{}", sum);
}
