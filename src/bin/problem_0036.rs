#[path = "../helper.rs"]
mod helper;
#[path = "../primes.rs"]
mod primes;

fn main() {
    //gets optimized into a nop I think
    //helper::check_bench(|| {solve(1_000_000);});
    assert_eq!(solve(1_000_000), 872187);
    dbg!(solve(1_000_000));
}

fn solve(n: usize) -> usize {
    (1..n)
        .step_by(2)
        .filter(|&i| is_palindrome_b2(i) && is_palindrome_b10(i))
        .sum()
}

fn is_palindrome_b10(n: usize) -> bool {
    helper::digits_iterator(n).fold(0, |mut acc, d| {
        acc *= 10;
        acc + d
    }) == n
}

fn is_palindrome_b2(n: usize) -> bool {
    (n.reverse_bits() >> n.leading_zeros()) == n
}
