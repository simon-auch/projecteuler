use projecteuler::primes;

fn main() {
    //gets optimized into a nop I think
    //helper::check_bench(|| {solve(1_000_000);});
    assert_eq!(solve(), 748317);
    dbg!(solve());
}

fn solve() -> usize {
    let single_digit_primes = [1, 2, 3, 5, 7, 9];
    let mut pow = 1;
    let mut left_inc: Vec<_> = [2, 3, 5, 7].iter().cloned().collect();
    let mut right_inc: Vec<_> = [2, 3, 5, 7].iter().cloned().collect();

    let mut acc = Vec::new();

    while !left_inc.is_empty() && !right_inc.is_empty() {
        //dbg!(&left_inc);
        //dbg!(&right_inc);
        pow *= 10;
        let new_left_inc: Vec<_> = left_inc
            .iter()
            .flat_map(|&p| single_digit_primes.iter().map(move |&d| p + d * pow))
            //.filter(|p| right_inc.contains(p))
            .filter(|&p| primes::is_prime(p))
            .collect();
        let new_right_inc: Vec<_> = right_inc
            .iter()
            .flat_map(|&p| single_digit_primes.iter().map(move |&d| p * 10 + d))
            //.filter(|p| left_inc.contains(p))
            .filter(|&p| primes::is_prime(p))
            .collect();

        for left_inc_p in &new_left_inc {
            let p = left_inc_p / 10;
            if right_inc.contains(&p) {
                dbg!(left_inc_p);
                acc.push(*left_inc_p);
            }
        }

        left_inc = new_left_inc;
        right_inc = new_right_inc;
    }

    acc.into_iter().sum()
}
