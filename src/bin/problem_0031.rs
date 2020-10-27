use projecteuler::partition;

static COINS: &[usize] = &[1, 2, 5, 10, 20, 50, 100, 200];

fn main() {
    //dbg!(binomial::binomial_coefficient(100, 50));
    dbg!(solve(20));
    dbg!(solve(200));
    //dbg!(solve(200));
}

fn solve(n: usize) -> usize {
    partition::partition_into(n, COINS)
}
