use std::iter;

use num::{BigUint, One, Zero};
use projecteuler::{helper, square_roots};

fn main() {
    helper::check_bench(|| {
        solve(1000);
    });
    dbg!(find_minimum_solution_in_x_4(&7, &61));
    dbg!(solve(7));
    assert_eq!(solve(1000), 661);
    assert_eq!(solve(7), 5);
    //dbg!(find_minimum_solution_in_x_1(&2));
    //dbg!(find_minimum_solution_in_x_1(&3));
    //dbg!(find_minimum_solution_in_x_1(&5));
    //dbg!(find_minimum_solution_in_x_1(&6));
    //dbg!(find_minimum_solution_in_x_1(&7));
}

fn solve(n: usize) -> usize {
    (1..)
        .map(|x| (x, x * x, (x + 1) * (x + 1)))
        .flat_map(|(previous_sqrt, previous_square, next_square)| {
            (previous_square + 1..next_square).map(move |non_square| (previous_sqrt, non_square))
        })
        .take_while(|(_previous_sqrt, non_square)| *non_square <= n)
        .max_by_key(|(previous_sqrt, non_square)| {
            find_minimum_solution_in_x_4(previous_sqrt, non_square)
        })
        .unwrap()
        .1
}

fn find_minimum_solution_in_x_4(previous_sqrt: &usize, d: &usize) -> BigUint {
    /*
    Basic observation: For each solution x/y approximate sqrt(d), maybe the approximation of one of the continued fractions is always a solution?
    */
    let continued_fraction = square_roots::get_continued_fraction_of(*previous_sqrt, *d)
        .map(|x| BigUint::from(x))
        .collect::<Vec<_>>();
    let previous_sqrt = BigUint::from(*previous_sqrt);

    let ret = (1..)
        .map(|i| {
            (0..i)
                .rev()
                .map(|j| j % continued_fraction.len())
                .map(|j| &continued_fraction[j])
                .chain(iter::once(&previous_sqrt))
                .fold((BigUint::zero(), BigUint::one()), |(nom, den), next| {
                    (den.clone(), nom + den * next)
                })
        })
        .find(|(den, nom)| {
            // these are the approximations of sqrt(d)
            //println!("{}, {}", nom, den);
            nom * nom == BigUint::one() + *d as u128 * den * den
        })
        .unwrap();

    //println!("{}: {}/{}", d, ret.1, ret.0);

    ret.1
}

#[allow(unused)]
fn find_minimum_solution_in_x_1(d: &u128) -> u128 {
    let mut x: u128 = 1;
    let mut y: u128 = 1;

    loop {
        let rhs = 1 + d * y * y;
        let lhs = x * x;
        match lhs.cmp(&(rhs)) {
            std::cmp::Ordering::Less => {
                //always makes either floor or ceil (sqrt(d)) steps
                x += 1;
            }
            std::cmp::Ordering::Equal => {
                println!("{}: {}, {}", d, x, y);
                //x += 1;
                return x;
            }
            std::cmp::Ordering::Greater => {
                y += 1;
            }
        }
    }
    return 0;
}
/*
x² - dy² = 1
x² + 2dx + d² - dy² = 1 - 2dx + d²
(x+d)² - dy² = 1 - 2dx + d²

x² - dy² = 1
x² + 1 = dy²
(x-1)² +2x = dy²

x² - dy² = 1
=> x² - (sqrt(d)y)² = 1
=> (x + sqrt(d)y)(x - sqrt(d)y) = 1


*/

/*

d : x, y
60: 1, 0
60: 31, 4
60: 1921, 248
60: 119071, 15372
60: 7380481, 952816
60: 457470751, 59059220

248      = 4      * 31 * 2 - 0
15372    = 248    * 31 * 2 - 4
952816   = 15372  * 31 * 2 - 248
59059220 = 952816 * 31 * 2 - 15372

y_{i+1}  = y_{i} * x_{1} * 2 - y_{i-1}

1  * 4   - 0 * 4    = 4
31 * 248 - 4 * 1921 = 4
1921 * 15372 - 248 * 119071 = 4

x_{i} * y_{i+1} - y_{i} * x_{i+1} = y_{1}

=>
x_{i} * (y_{i} * x_{1} * 2 - y_{i-1}) - y_{i} * x_{i+1} = y_{1}
x_{i} * (y_{i} * x_{1} * 2 - y_{i-1}) - y_{1} = y_{i} * x_{i+1}
(x_{i} * (y_{i} * x_{1} * 2 - y_{i-1}) - y_{1}) / y_{i} = x_{i+1}
*/
