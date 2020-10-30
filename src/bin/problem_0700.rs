use projecteuler::helper;

fn main() {
    helper::check_bench(|| {
        solve_2();
    });
    assert_eq!(solve_2(), 1517926517777556);
    //dbg!(1_504_170_715_041_707u128 * 3_451_657_199_285_664u128 % 4_503_599_627_370_517u128);
    dbg!(solve());
}
//multiplactive inverse element of m is:
// 3_451_657_199_285_664

fn solve() -> usize {
    let m = 4_503_599_627_370_517u128;
    let fact = 1_504_170_715_041_707u128;
    let inv = 3_451_657_199_285_664u128;
    let mut current = fact;

    let mut count = 1;
    let mut min = current;
    let mut acc = min;

    //first we decrease the amount of potential euler numbers by brute forcing the first few
    while count < 16 {
        current += fact;
        current %= m;
        if current < min {
            count += 1;
            min = current;
            acc += min;
            //dbg!(min);
        }
    }
    //dbg!(current);
    //dbg!(min);
    //now for every number below our minimum we compute when they would appear
    let mut rest: Vec<_> = (1..min).map(|i| (i * inv % m, i)).collect();
    rest.sort();
    //and sum the ones up that are the smallest up to that point
    for (_, i) in rest.iter() {
        if *i < min {
            count += 1;
            min = *i;
            acc += min;
            //dbg!(min);
        }
    }
    acc as usize
}

fn solve_2() -> usize {
    let m = 4_503_599_627_370_517u128;
    let fact = 1_504_170_715_041_707u128;

    let mut low = fact;
    let mut high = fact;

    let mut acc = low;

    while low != 0 {
        let next = (low + high) % m;
        if next < low {
            low = next;
            acc += low;
        } else {
            high = next;
        }
    }

    acc as usize
}
