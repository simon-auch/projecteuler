use projecteuler::helper;

fn main() {
    helper::check_bench(|| {
        solve(10usize.pow(9));
    });
    assert_eq!(solve(10usize.pow(9)), 608720);
    dbg!(solve(10usize.pow(9)));
}

fn solve(n: usize) -> usize {
    (1..n)
        .filter(|i| i % 10 != 0)
        .map(|i| i + helper::reverse_digits(i))
        .filter(|&i| helper::digits_iterator(i).all(|d| d & 0b1 == 1))
        .count()
}
