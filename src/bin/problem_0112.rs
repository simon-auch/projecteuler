use projecteuler::digits;
use projecteuler::helper;

fn main() {
    helper::check_bench(|| solve(99));
    assert_eq!(solve(99), 1587000);
    dbg!(solve(99));
}

fn solve(percent: usize) -> usize {
    let mut total = 1;
    let mut count = 0;
    while count * 100 != total * percent {
        total += 1;
        if is_bouncy(total) {
            count += 1;
        }
    }
    total
}

fn is_bouncy(n: usize) -> bool {
    let (inc, dec, _) =
        digits::digits_iterator(n).fold((true, true, None), |(inc, dec, last), d| {
            if let Some(last_d) = last {
                (inc && last_d <= d, dec && last_d >= d, Some(d))
            } else {
                (true, true, Some(d))
            }
        });
    !inc && !dec
}
