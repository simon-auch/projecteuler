use projecteuler::helper;

fn main() {
    helper::check_bench(|| {
        solve();
    });
    assert_eq!(solve(), 210);
    dbg!(solve());
}

fn solve() -> usize {
    [1_000_000, 100_000, 10_000, 1_000, 100, 10, 1usize]
        .iter()
        .map(|i| get_at_index(*i))
        .product::<usize>()
}

fn get_at_index(n: usize) -> usize {
    let n = n - 1;
    let mut current_index = 0usize;
    let mut current_digits = 1usize;
    loop {
        let current_pow = 10usize.pow(current_digits as u32 - 1);
        let numbers = current_pow * 10 - current_pow;
        let max_step = current_digits * numbers;
        if current_index + max_step < n {
            current_index += max_step;
            current_digits += 1;
            continue;
        }
        let i = current_pow + (n - current_index) / current_digits;
        let j = (n - current_index) % current_digits;
        let mut digits = helper::digits(i);
        digits.reverse();
        return digits[j] as usize;
    }
}
