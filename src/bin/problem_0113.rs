use projecteuler::binomial;
use projecteuler::helper;

fn main() {
    helper::check_bench(|| solve(100));
    assert_eq!(solve(100), 51161058134250);
    helper::check_bench(|| solve_2(100));
    assert_eq!(solve_2(100), 51161058134250);
    dbg!(solve(100));
}

//also works, but is much slower (about factor 70 if one is to believe the benchmarks)
fn solve_2(digits: usize) -> usize {
    let mut total = 0;
    for i in 1..=digits {
        total += binomial::binomial_coefficient(8 + i, i);
        total += binomial::binomial_coefficient(9 + i, i);
        total -= 10;
    }
    total
}

fn solve(digits: usize) -> usize {
    //idea: recursion on number of digits
    let inc = count_increasing(digits);
    let dec = count_decreasing(digits);
    inc + dec - 9 * digits
}

fn count_increasing(mut digits: usize) -> usize {
    let mut total = 0;
    let mut current = [1; 9];
    while digits > 1 {
        let mut s = 0;
        for i in 0..current.len() {
            s += current[i];
            current[i] = s;
        }
        total += s;
        digits -= 1;
    }
    total + current.iter().cloned().sum::<usize>()
}

fn count_decreasing(mut digits: usize) -> usize {
    let mut total = 0;
    let mut current = [1; 10];
    current[0] = 0;
    while digits > 1 {
        let mut s = 0;
        for i in (0..current.len()).rev() {
            s += current[i];
            current[i] = s;
        }
        total += s;
        digits -= 1;
    }
    total + current.iter().cloned().sum::<usize>()
}
