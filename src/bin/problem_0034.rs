use projecteuler::digits;
use projecteuler::factorial;
use projecteuler::helper;

fn main() {
    helper::check_bench(|| {
        solve();
    });
    assert_eq!(solve(), 40730);
    dbg!(solve());
}

fn solve() -> usize {
    let factorials: [usize; 10] = [
        factorial::factorial(0),
        factorial::factorial(1),
        factorial::factorial(2),
        factorial::factorial(3),
        factorial::factorial(4),
        factorial::factorial(5),
        factorial::factorial(6),
        factorial::factorial(7),
        factorial::factorial(8),
        factorial::factorial(9),
    ];

    //9999999 => 9!*7 = 2_540_160
    //adding more digits will just make it worse since 9 is the biggest digit
    let max = 2_540_160usize;
    let mut ret = 0;
    for i in 3..=max {
        let sum = digits::digits_iterator(i)
            .map(|d| factorials[d as usize])
            .sum::<usize>();
        if i == sum {
            ret += i;
        }
    }
    ret
}
