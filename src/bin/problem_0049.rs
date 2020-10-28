use projecteuler::helper;
use projecteuler::primes;

fn main() {
    helper::check_bench(|| {
        solve(4, 3);
    });
    //helper::check_bench(|| {
    //    solve(6, 4);
    //});
    assert_eq!(solve(4, 3)[1], [2969, 6299, 9629]);
    dbg!(solve(6, 4));
    //dbg!(solve(10, 5));
}

//n represents the number of digits
//m represents the length of the sequence to find
fn solve(n: usize, m: usize) -> Vec<Vec<usize>> {
    let primes = primes::primes(10usize.pow(n as u32));
    let barrier = primes
        .binary_search(&10usize.pow((n - 1) as u32))
        .expect_err("1000 is not prime");
    let primes: Vec<usize> = primes[barrier..].iter().cloned().collect();

    let mut prime_digits: Vec<(usize, usize)> = primes
        .iter()
        .map(|p| {
            (*p, {
                let mut d = helper::digits(*p);
                d.sort();
                helper::from_digits(&d)
            })
        })
        .collect();

    prime_digits.sort_by(|(_, d1), (_, d2)| d1.cmp(d2));

    //dbg!(&prime_digits);
    let mut candidates = Vec::new();
    {
        let mut i = 0;
        for j in 1..prime_digits.len() {
            if prime_digits[j].1 == prime_digits[i].1 {
            } else {
                if j - i >= m {
                    candidates.push(&prime_digits[i..j]);
                }
                i = j;
            }
        }
    }

    //dbg!(&candidates);

    let results: Vec<_> = candidates
        .iter()
        .flat_map(|c| check_contains_sequence(c, m))
        .collect();

    //dbg!(&results);
    results
}

fn check_contains_sequence(prime_digits: &[(usize, usize)], m: usize) -> Vec<Vec<usize>> {
    let mut acc = Vec::new();
    for i in 0..prime_digits.len() {
        'j: for j in i + 1..prime_digits.len() {
            let d = prime_digits[j].0 - prime_digits[i].0;
            for k in 1..=m - 2 {
                match prime_digits.binary_search(&(prime_digits[j].0 + d * k, prime_digits[i].1)) {
                    Ok(_) => {}
                    _ => {
                        continue 'j;
                    }
                }
            }
            acc.push((0..m).map(|k| prime_digits[i].0 + d * k).collect());
        }
    }
    acc
}
