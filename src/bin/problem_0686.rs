use projecteuler::helper;

fn main() {
    helper::check_bench(|| {
        solve(678910);
    });
    assert_eq!(solve(678910), 193060223);
    dbg!(solve(678910));
}

fn solve(n: usize) -> usize {
    /*
    For some reasons after the first exponent, subsequent exponents always have a distance of 196, 289, or 485.
    */
    let mut last_found = 90;
    let mut found = 1;
    let mut current_power = 90;
    let mut current = 1u128 << current_power;

    while current > u64::MAX as u128 {
        current /= 10;
    }

    let steps = &[196, 289 - 196, 485 - 289];
    // truncated decimals of 2**steps[i]
    let steps_fac = &[
        1004336277661868922,
        9903520314283042199,
        1004336277661868922,
    ];
    let steps_div = 1000000000000000000u128;

    'outer: while found < n {
        //dbg!(total_steps);

        for step_i in 0..3 {
            let step = steps[step_i];
            let step_fac = steps_fac[step_i];

            current *= step_fac;
            current /= steps_div;
            current_power += step;
            while current > u64::MAX as u128 {
                current /= 10;
            }

            let mut current = current.clone();
            current /= 10u128.pow((128 - current.leading_zeros()) / 4);
            while current > 999 {
                current /= 10;
            }

            if current == 123 {
                found += 1;
                last_found = current_power;
                continue 'outer;
            }
        }
        dbg!(last_found, found, current_power);
        panic!();
    }
    current_power as usize
}
