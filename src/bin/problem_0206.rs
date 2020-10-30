use projecteuler::helper;
use projecteuler::primes;

fn main() {
    helper::check_bench(|| {
        solve();
    });
    assert_eq!(solve(), 1389019170);
    dbg!(solve());
}

fn solve() -> usize {
    let (mut low, _) = primes::sqrt(1020304050607080900);
    let (_, high) = primes::sqrt(1929394959697989990);
    low = (low / 10) * 10;
    (low..=high).step_by(10).filter(is_answer).next().unwrap()
}

fn is_answer(n: &usize) -> bool {
    let square = n * n;
    if square < 1020304050607080900 {
        return false;
    }
    //dbg!(n, square);
    helper::digits_iterator(square)
        .step_by(2)
        .zip([1, 2, 3, 4, 5, 6, 7, 8, 9, 0].iter().rev())
        .all(|(a, b)| a == *b)
}
