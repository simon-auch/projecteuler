use projecteuler::digits;
use projecteuler::helper;

fn main() {
    helper::check_bench(|| {
        solve(10usize.pow(12));
    });
    assert_eq!(solve(10_000), 41333);
    assert_eq!(solve(10usize.pow(12)), 128088830547982);
    //dbg!(1_504_170_715_041_707u128 * 3_451_657_199_285_664u128 % 4_503_599_627_370_517u128);
    dbg!(solve(10 * 10 * 10 * 10));
    dbg!(solve(10usize.pow(12)));
}

fn solve(n: usize) -> usize {
    (4..)
        .map(|i| (i, i * i))
        .take_while(|(_, sq)| *sq <= n)
        .filter(is_s_number)
        .map(|(_, sq)| sq)
        .sum()
}

fn is_s_number((i, sq): &(usize, usize)) -> bool {
    let digits: Vec<u8> = digits::digits(*sq);
    recursion(&digits, *i)
}

fn recursion(digits: &[u8], total_left: usize) -> bool {
    if digits.len() == 0 {
        panic!();
    }
    let max: usize = digits::from_digits(digits.iter().rev());
    if max == total_left {
        return true;
    }
    if max < total_left {
        return false;
    }
    //max > total_left
    for split in 1..digits.len() {
        let (start, end) = digits.split_at(split);
        let inc: usize = digits::from_digits(start.iter().rev());
        if inc > total_left {
            break;
        } else {
            if recursion(end, total_left - inc) {
                return true;
            }
        }
    }
    false
}
