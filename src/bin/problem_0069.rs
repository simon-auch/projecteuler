use projecteuler::fraction;
use projecteuler::helper;
use projecteuler::primes;

fn main() {
    helper::check_bench(|| {
        solve(1_000_000);
    });
    assert_eq!(solve(1_000_000), 510510);
    dbg!(solve(10));
    dbg!(solve(1_000_000));
}

fn solve(n: usize) -> usize {
    primes::euler_phi_list(n)
        .iter()
        .enumerate()
        .map(|(i, phi)| fraction::Fraction::new(i, *phi))
        .max()
        .unwrap()
        .num()
}
