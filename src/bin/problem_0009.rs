/*
a²+b²=c²
a+b+c = s

a²+b²=(s-a-b)²
= s²+a²+b²-2sa-2sb+2ab

<=> 2sa + 2sb = s² + 2ab
<=> a = (s*(s-2b))/(2*(s-b))
*/

#[path = "../fraction.rs"]
mod fraction;
use fraction::Fraction;

fn main() {
    dbg!(solve(1_000));
}

fn solve(s: usize) -> Option<usize> {
    //b must be smaller than s/2
    for b in 1..s / 2 {
        let a = Fraction::new(s * (s - 2 * b), 2 * (s - b));
        if a.is_integer() {
            let a = a.floor();
            let c = s - a - b;
            return Some(a * b * c);
        }
    }
    None
}
