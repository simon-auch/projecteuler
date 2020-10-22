fn main() {
    dbg!(solve(5));
    dbg!(solve(1001));
}

fn solve(n: usize) -> usize {
    let mut last = 1;
    let mut sum = 1;
    let mut stepsize = 2;
    for _ in 0..n / 2 {
        for _i in 1..=4 {
            last = last + stepsize;
            sum += last;
        }
        stepsize += 2;
    }
    sum
}
