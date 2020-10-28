use projecteuler::helper;

fn main() {
    helper::check_bench(|| {
        solve();
    });
    assert_eq!(solve(), 49);
    dbg!(solve());
}

fn solve() -> usize {
    /*
    a.pow(b)
    if a is greater equal 10, a.pow(b) will always have more digits than b

    when does a**b has the correct number of digits?
    10^(b-1) <= a**b < 10^b

    10^(b-1) = a**b
    0.1 * 10^b = a**b
    log(0.1) + log(10)*b = log(a)*b
    log(0.1) = b*(log(a) - log(10))
    b = log(0.1)/(log(a) - log(10))

    */
    let mut acc = 0;
    for a in 1u128..10 {
        //computes the same value as the following loop, just without the loop
        acc += (0.1f64.log2() / ((a as f64).log2() - (10.0f64.log2()))).floor() as usize;
        /*
        for b in 1u128.. {
            let x = a.pow(b as u32);

            let digits = format!("{}", x).len() as u128;
            if digits == b {
                acc += 1;
            }
            if digits < b {
                break;
            }
        }
        */
    }
    acc
}
