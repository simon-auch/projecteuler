#[path = "../binomial.rs"]
mod binomial;

fn main() {
    dbg!(solve(2));
    dbg!(solve(20));
}

fn solve(n: usize) -> usize {
    binomial::binomial_coefficient(2 * n, n)
}
