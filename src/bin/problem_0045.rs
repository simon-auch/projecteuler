#[path = "../helper.rs"]
mod helper;

fn main() {
    //gets optimized into a nop I think
    //helper::check_bench(|| {
    //    solve();
    //});
    assert_eq!(solve(), 1533776805);
    dbg!(solve());
}
//What is the distance from T_n to T_{n+1}?
/*
Well, obviously:
T_{n+1}-T_n = n+1
*/
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
//What is the distance from H_n to H_{n-1}
/*
By guessing
H_{n+1}-H_n = 4n+1
*/
fn solve() -> usize {
    //since evry triangle number is a hexagonal number, we can ignore the triangle numbers
    let mut p_i = 165;
    let mut h_i = 143;
    let mut p = 40755usize;
    let mut h = p;

    p += 3 * p_i + 1;
    p_i += 1;

    while !(p == h) {
        if p <= h {
            p += 3 * p_i + 1;
            p_i += 1;
        } else {
            h += 4 * h_i + 1;
            h_i += 1;
        }
    }
    p
}
