#[path = "../factorial.rs"]
mod factorial;

fn main() {
    dbg!(solve(1, vec![0, 1, 2]));
    dbg!(solve(1_000_000, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));
}

fn solve(mut n: usize, mut digits: Vec<usize>) -> usize {
    n -= 1;
    let mut ret = 0;
    while digits.len() > 0 {
        let m: usize = factorial::factorial(digits.len() - 1);
        let i = n / m;
        n -= i * m;
        let d = digits.remove(i);
        ret *= 10;
        ret += d;
    }
    ret
}
