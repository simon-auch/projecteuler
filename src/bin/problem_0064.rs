use projecteuler::{helper, square_roots};

fn main() {
    helper::check_bench(|| {
        solve(10_000);
    });
    assert_eq!(solve(10_000), 1322);
}

fn solve(n: usize) -> usize {
    return (1..)
        .map(|x| (x, x * x, (x + 1) * (x + 1)))
        .flat_map(|(x, previous_square, next_square)| {
            (previous_square + 1..next_square).map(move |non_square| (x, non_square))
        })
        .take_while(|(_x, non_square)| *non_square <= n)
        .map(|(previous_sqrt, non_square)| {
            square_roots::get_continued_fraction_of(previous_sqrt, non_square).count()
        })
        .filter(|period_length| period_length & 0b1 == 1)
        .count();
}
