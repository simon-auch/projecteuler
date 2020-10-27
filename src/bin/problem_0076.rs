use projecteuler::partition;

fn main() {
    //dbg!(binomial::binomial_coefficient(100, 50));
    dbg!(solve(4));
    dbg!(solve(5));
    dbg!(solve(6));
    dbg!(solve(100));
}

fn solve(n: usize) -> usize {
    partition::partition(n) - 1
}
