use projecteuler::fibonacci;
use projecteuler::helper;
use projecteuler::modulo;

fn main() {
    helper::check_bench(|| {
        solve();
    });
    for i in 1..100 {
        dbg!(i, S(i));
    }
    assert_eq!(solve(), 922058210);
    dbg!(solve());
}

fn solve() -> usize {
    let m = 1_000_000_007;
    fibonacci::FibonacciIterator::new()
        .take(89)
        .map(|f| S_fast_mod(f, m))
        .fold(0, |acc, s| (acc + s) % m) as usize
}

fn s(mut n: usize) -> usize {
    let mut exp = 1;
    let mut acc = 0;
    for d in (1..=9).rev() {
        let count = n / d;
        n -= count * d;
        for _ in 0..count {
            acc += exp * d;
            exp *= 10;
        }
    }
    acc
}

#[allow(unused, non_snake_case)]
fn S(n: usize) -> usize {
    (1..=n).map(s).sum::<usize>()
}

fn s_fast(n: usize) -> usize {
    let mut nines = n / 9;
    let d = n % 9;

    let mut acc = 0;
    let mut exp = 1;
    let mut current = 9;
    while nines > 0 {
        if nines & 0b1 == 1 {
            acc = acc + exp * current;
            exp = acc + 1;
        }
        current += current * (current + 1);
        nines >>= 1;
    }
    d * exp + acc
}

#[allow(unused, non_snake_case)]
fn S_fast(mut n: usize) -> usize {
    /*
    001
    002
    003
    004
    005
    006
    007
    008
    009
    019
    029
    039
    049
    059
    069
    079
    089
    099
    199
    299
    399
    499
    599
    699
    799
    899
    999

    1-9   0  0  45
    10-18 0  45 81
    19-27 45 81 81

    00000000045
    00000000531
    00000005391
    00000053991
    00000539991
    00005399991
    00053999991
    00539999991
    05399999991
    53999999991

    \sum_{dec=1}^{n/10}

    */
    let mut acc = 0;

    let r = n % 9;
    let mut m = n / 9;

    const CORNER: usize = 45;
    const BLOCK: usize = 81;
    let mut total = CORNER;
    let mut line = BLOCK;

    let mut current_lines: usize = 1;
    let mut lines_left = m;
    let mut lines_removed: usize = 0;
    while m > 0 {
        if m & 0b1 == 1 {
            let inc = total + line * (lines_left - current_lines);
            acc += inc * 10usize.pow(lines_removed as u32);
            lines_removed += current_lines;
            lines_left -= current_lines;
        }
        total = total + total * 10usize.pow(current_lines as u32) + line * current_lines;

        line += line * 10usize.pow(current_lines as u32);
        current_lines *= 2;
        m >>= 1;
    }

    for i in n / 9 * 9 + 1..=n {
        acc += s_fast(i);
    }

    acc
}

fn s_fast_mod(n: u128, m: u128) -> u128 {
    let mut nines = n / 9;
    let d = n % 9;

    let mut acc = 0;
    let mut exp = 1;
    let mut current = 9;
    while nines > 0 {
        if nines & 0b1 == 1 {
            acc = acc + exp * current;
            acc %= m;
            exp = acc + 1;
            exp %= m;
        }
        current += current * (current + 1);
        current %= m;
        nines >>= 1;
    }
    (d * exp + acc) % m
}

#[allow(non_snake_case)]
fn S_fast_mod(n: u128, m: u128) -> u128 {
    let mut acc = 0;

    let mut bitmask = n / 9;

    const CORNER: u128 = 45;
    const BLOCK: u128 = 81;
    let mut total = CORNER;
    let mut line = BLOCK;

    let mut current_lines = 1;
    let mut lines_left = bitmask;
    let mut lines_removed = 0;
    while bitmask > 0 {
        if bitmask & 0b1 == 1 {
            let mut inc = total + line * (lines_left - current_lines);
            inc %= m;
            acc += inc * modulo::modulo_power(10, lines_removed, m);
            acc %= m;
            lines_removed += current_lines;
            lines_left -= current_lines;
        }
        total = total + total * modulo::modulo_power(10, current_lines, m) + line * current_lines;
        total %= m;

        line += line * modulo::modulo_power(10, current_lines, m);
        line %= m;
        current_lines *= 2;
        bitmask >>= 1;
    }

    for i in n / 9 * 9 + 1..=n {
        acc += s_fast_mod(i, m);
    }

    acc
}
