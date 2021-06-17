use projecteuler::helper;
use projecteuler::square_roots;

fn main() {
    //gets optimized into a nop I think
    helper::check_bench(|| {
        solve();
    });
    assert_eq!(solve(), 5482660);
    dbg!(solve());
}
//what is the distance from P_n to P_{n+1}?
/*
P_{n+1}-P_n = (n+1)(3n+3-1)/2 - n(3n-1)/2
P_{n+1}-P_n = ((n+1)(3n+3-1) - n(3n-1))/2

(n+1)(3n+3-1) - n(3n-1)
(n+1)(3n+2) - n(3n-1)
3n²+2n+3n+2 - 3n²+n
2n+3n+2+n
6n+2

P_{n+1}-P_n = (6n+2)/2
P_{n+1}-P_n = 3n+1
*/

//how to go back from number to index
/*
P_n = n(3n-1)/2
n = 1/6 * (1 +- \sqrt{ 24*P_n + 1 })
*/
fn solve() -> usize {
    let mut min = usize::MAX;
    for i in 1.. {
        let p_i = get_pentagonal(i);
        for j in (1..i).rev() {
            let p_j = get_pentagonal(j);
            let diff = p_i - p_j;
            let sum = p_i + p_j;
            if diff > min {
                if j == i - 1 {
                    dbg!(i);
                    return min;
                }
                break;
            }
            if is_pentagonal(diff) && is_pentagonal(sum) {
                dbg!(i, diff);
                min = diff;
            }
        }
    }

    0
}

fn get_pentagonal(n: usize) -> usize {
    (n * (3 * n - 1)) / 2
}

fn is_pentagonal(n: usize) -> bool {
    let (floor, ceil) = square_roots::sqrt(24 * n + 1);
    if floor != ceil {
        return false;
    }
    let temp = floor + 1;
    temp % 6 == 0
}
