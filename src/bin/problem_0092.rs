use projecteuler::digits;
use projecteuler::helper;

fn main() {
    helper::check_bench(|| {
        solve();
    });
    assert_eq!(solve(), 8581146);
    dbg!(solve());
}

fn solve() -> usize {
    /*
    maximum number below ten million:
    9_999_999

    square of 9: 81
    7*9^2 = 576
    */
    let mut count = 0;
    let mut cache = Vec::with_capacity(577);
    cache.push(false);

    for i in 1..10_000_000 {
        let i = if i <= 576 {
            cache.push(ends_in_89(i));
            i
        } else {
            next(i)
        };
        if cache[i] {
            count += 1;
        }
    }

    count
}

fn next(n: usize) -> usize {
    digits::digits_iterator(n)
        .map(|d| (d * d) as usize)
        .sum::<usize>()
}

fn ends_in_89(mut n: usize) -> bool {
    while n != 1 && n != 89 {
        n = next(n);
    }
    n == 89
}
