#[path = "../helper.rs"]
mod helper;
#[path = "../primes.rs"]
mod primes;

fn main() {
    helper::check_bench(|| {
        solve(1_000_000);
    });
    assert_eq!(solve(1_000_000), 997651);
    dbg!(solve(1_000_000));
}

//n represents the number of digits
//m represents the length of the sequence to find
fn solve(n: usize) -> usize {
    let primes = primes::primes(n);
    let mut primes_bool = Vec::new();
    primes_bool.resize_with(n, || false);
    for &p in primes.iter() {
        primes_bool[p] = true;
    }

    let mut terms = 0;
    let mut acc = 0;
    while acc + primes[terms] < n {
        acc += primes[terms];
        terms += 1;
    }

    if primes_bool[acc] {
        return acc;
    }

    let mut acc_start = acc;
    let mut offset = 0;
    while terms > 1 {
        let next_acc = acc + primes[offset + terms] - primes[offset];
        if next_acc >= n {
            offset = 0;
            terms -= 1;
            acc = acc_start - primes[terms];
            acc_start = acc;
            if primes_bool[acc] {
                return acc;
            }
        } else {
            acc = next_acc;
            offset += 1;
            if primes_bool[acc] {
                return acc;
            }
        }
    }

    0
}
