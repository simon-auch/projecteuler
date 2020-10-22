#[path = "../fraction.rs"]
mod fraction;
#[path = "../helper.rs"]
mod helper;

fn main() {
    //called multiple times with different loop counts in order to check if the compiler just removes it
    //if it optimized it away the measured time per loop would decrease with increased loop count
    helper::time_it(
        || {
            solve();
        },
        100,
    );
    helper::time_it(
        || {
            solve();
        },
        1000,
    );
    helper::time_it(
        || {
            solve();
        },
        10000,
    );
    helper::time_it(
        || {
            solve();
        },
        100000,
    );
    assert_eq!(solve(), 100);
    dbg!(solve());
}

fn solve() -> usize {
    //the correct reduced fraction must be one of:
    //1/2, 1/3,..,1/9
    //2/3, 2/4,..,2/9
    //...
    //7/8,7/9
    //8/9
    let mut acc = fraction::Fraction::new(1, 1);
    for num in 1..=8 {
        for den in num + 1..=9 {
            let mut frac = fraction::Fraction::new(num, den);
            frac.reduce();
            let num_red = frac.num();
            let den_red = frac.den();

            let mut i = 2;
            while den_red * i <= 99 {
                let num_i = num_red * i;
                let den_i = den_red * i;
                if num_i <= 10 || den_i <= 10 || i % 10 == 0 {
                    i += 1;
                    continue;
                } else {
                    i += 1;
                }
                let n_1 = num_i % 10;
                let n_2 = num_i / 10;
                let d_1 = den_i % 10;
                let d_2 = den_i / 10;

                if n_1 == d_2 && n_2 == num && d_1 == den
                    || n_2 == d_1 && n_1 == num && d_2 == den
                {
                    //dbg!(num,den,num_i,den_i,num_red,den_red);
                    acc = fraction::Fraction::new(acc.num() * num_red, acc.den() * den_red);
                }
            }
        }
    }
    acc.reduce();
    acc.den()
}
