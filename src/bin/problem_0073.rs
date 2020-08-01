#![feature(test)]
#![feature(map_first_last)]

#[path = "../fraction.rs"]
mod fraction;
use fraction::Fraction;

//the fastest one is definitely solve5.

fn main() {
    for i in &[8, 10, 12, 20, 12_000] {
        let i = *i;
        dbg!(i);
        dbg!(solve(i));
        dbg!(solve2(i));
        dbg!(solve3(i));
        dbg!(solve4(i));
        dbg!(solve5(i));
    }
    //dbg!(solve5(12_000));
    //dbg!(solve5(200_000_000));
    //dbg!(solve5(12_000));
    //dbg!(solve5(2_000_000_000));
    //dbg!(solve3(100_000));
    //dbg!(solve3(1_000_000));
}

//based on Farey series
fn solve2(d: usize) -> usize {
    let low = 3;
    let high = 2;
    recursion(d, low, high)
}
fn recursion(d: usize, x: usize, y: usize) -> usize {
    let s = x + y;
    if s > d {
        0
    } else {
        1 + recursion(d, x, s) + recursion(d, s, y)
    }
}

//basically solve2 but  not recursive
fn solve3(d: usize) -> usize {
    let mut sum = 0;
    let mut stack = Vec::with_capacity(d);
    stack.push(3);
    stack.push(2);
    while stack.len() > 1 {
        let s = stack[stack.len() - 2] + stack[stack.len() - 1];
        if s <= d {
            let y = stack.pop().unwrap();
            stack.push(s);
            stack.push(y);
            sum += 1
        } else {
            stack.pop();
        }
    }
    sum
}

fn solve4(d: usize) -> usize {
    /*
    As its a farey series all fractions must have the form:
    (k*1+m*1)/(k*3+m*2)
    with gcd(k,m) = 1. (otherwise the fraction would not be reduced)

    So the question is how many pairs (k,m) exist that meet the conditions:
        k*3 + m*2 <= d
        gcd(k,m) = 1 //this ensures the fraction is reduced
    if we only think about the first condition there are ... many pairs (k,m):
        //upper limit for m (k=1)
        pairs = 0
        max_m <= (d-3)/2
        for m in 1..=max_m {
            max_k = floor((d-2*m)/3)
            pairs += max_k
        }

        split the loop for m mod 3
        m mod 3 = 0
            floor((d-2*m)/3)
            = floor(d/3 - 2*m/3)
            = floor(d/3) - 2*m/3
        which is corresponds to:
        sum_{m=3,6..}^{max_m} floor(d/3) - 2*m/3
        = sum_{i=1}^{floor(max_m/3)} floor(d/3) - 2*i
        = floor(max_m/3)*floor(d/3) - 2*(floor(max_m/3)*(floor(max_m/3)+1)/2)
        = floor(max_m/3) * ( floor(d/3) - floor(max_m/3)-1 )

        m mod 3 = 1
            floor((d-2*m)/3)
            = floor(d/3 - 2*m/3)
            = floor(d/3 - 2/3) - 2*(m-1)/3
            = floor((d - 2)/3) - 2*(m-1)/3
        which is corresponds to:
        sum_{m=1,4..}^{max_m} floor((d - 2)/3) - 2*(m-1)/3
        = sum_{i=1}^{floor((max_m+2)/3)} floor((d - 2)/3) - 2*((i*3-2)-1)/3
        = sum_{i=1}^{floor((max_m+2)/3)} floor((d - 2)/3) - 2*(i-1)
        = floor((max_m+2)/3)*floor((d - 2)/3) + 2 floor((max_m+2)/3) - 2 * sum_{i=1}^{floor((max_m+2)/3)} i
        = floor((max_m+2)/3)*floor((d - 2)/3) + 2 floor((max_m+2)/3) - 2 * (floor((max_m+2)/3)*(floor((max_m+2)/3)+1))/2
        = floor((max_m+2)/3) * ( floor((d - 2)/3) + 1 - floor((max_m+2)/3) )

        m mod 3 = 2
            floor((d-2*m)/3)
            = floor(d/3 - 2*m/3)
            = floor(d/3 - 4/3) - 2*(m-2)/3
            = floor((d - 4)/3) - 2*(m-2)/3
        sum_{m=2,5..}^{max_m} floor((d - 4)/3) - 2*(m-2)/3
        = sum_{i=1}^{floor((max_m+1)/3)} floor((d - 4)/3) - 2*((i*3-1)-2)/3
        = sum_{i=1}^{floor((max_m+1)/3)} floor((d - 4)/3) - 2*(i-1)
        = floor((max_m+1)/3) * floor((d - 4)/3) - 2 * sum_{i=1}^{floor((max_m+1)/3)} (i-1)
        = floor((max_m+1)/3) * floor((d - 4)/3) - 2 * ( ((floor((max_m+1)/3)*(floor((max_m+1)/3)+1))/2) - floor((max_m+1)/3))
        = floor((max_m+1)/3) * floor((d - 4)/3) + floor((max_m+1)/3) * ( 1 - floor((max_m+1)/3) )
        = floor((max_m+1)/3) * (floor((d - 4)/3) +  1 - floor((max_m+1)/3) )




        //in other words:
        sum_{m=1}^{max_m} floor((d-2*m)/3)
        //which is the same as:
        1/2 * sum_{m=1}^{max_m} (d-2*m)/3 + (d-2*(max_m-m+1))/3
        1/6 * sum_{m=1}^{max_m} (d-2*m) + (d-2*(max_m-m+1))
        1/6 * sum_{m=1}^{max_m} 2d -2*m -2*(max_m-m+1)
        1/3 * sum_{m=1}^{max_m} d -m -(max_m-m+1)
        1/3 * (d*max_m + sum_{m=1}^{max_m} -m -max_m+m-1)
        1/3 * (d*max_m + sum_{m=1}^{max_m} -max_m-1)
        1/3 * (d*max_m - max_m*max_m - max_m)
        max_m/3 * (d - max_m - 1)

    now lets think how many pairs (k,m) have gcd(k,m)=x
        let g be the denominator of such a fraction.
        as x divides g and g is smaller than d, the denominator of the reduced fraction must be smaller than d/x
        it follows that the number of pairs (k,m) with gcd(k,m)=x = solve4(d/x)
    */
    if d <= 4 {
        return 0;
    }
    let max_m = Fraction::new(d - 3, 2).floor();
    let max_k = Fraction::new(d - 2, 3).floor();
    let floor = |a, b| Fraction::new(a, b).floor();
    let pairs = floor(max_m, 3) * (floor(d, 3) - floor(max_m, 3) - 1)
        + floor(max_m + 2, 3) * (floor(d - 2, 3) + 1 - floor(max_m + 2, 3))
        + floor(max_m + 1, 3) * (floor(d - 4, 3) + 1 - floor(max_m + 1, 3));

    let mut doubles = 0;
    for g in 2..=max_k {
        doubles += solve4(d / g);
    }
    pairs - doubles
}

//same idea as solve4, but only evaluating solve5(x) once for each x at most
fn solve5(d: usize) -> usize {
    if d <= 4 {
        return 0;
    }
    let mut sum = 0;
    let mut stack: std::collections::BTreeMap<usize, (usize, usize)> = Default::default();

    stack.insert(d, (1, 0));

    while let Some((d, (p_mult, n_mult))) = stack.pop_last() {
        //dbg!((d, (p_mult, n_mult)));
        let max_m = Fraction::new(d - 3, 2).floor();
        let max_k = Fraction::new(d - 2, 3).floor();
        let floor = |a, b| Fraction::new(a, b).floor();
        let pairs = floor(max_m, 3) * (floor(d, 3) - floor(max_m, 3) - 1)
            + floor(max_m + 2, 3) * (floor(d - 2, 3) + 1 - floor(max_m + 2, 3))
            + floor(max_m + 1, 3) * (floor(d - 4, 3) + 1 - floor(max_m + 1, 3));

        sum += p_mult * pairs;
        sum -= n_mult * pairs;

        let mut g = 2;
        while g <= max_k {
            if d / g <= 4 {
                break;
            }
            let mut mult = 1;
            while d / g == d / (g + 1) {
                mult += 1;
                g += 1;
            }
            stack
                .entry(d / g)
                .and_modify(|(other_p_mult, other_n_mult)| {
                    *other_p_mult += mult * n_mult;
                    *other_n_mult += mult * p_mult;
                })
                .or_insert((mult * n_mult, mult * p_mult));
            g += 1;
        }
    }
    sum
}

fn solve(d: usize) -> usize {
    let mut sum = 0;
    let low = Fraction::new(1, 3);
    let high = Fraction::new(1, 2);
    for i in 3..=d {
        //dbg!(i);
        //dbg!(low);
        /*
        low < a/i
        a > low*i

        a/i < high
        a < high*i
        */
        let min = Fraction::new(low.num() * i, low.den()).floor() + 1;
        let max = Fraction::new(high.num() * i, high.den()).ceil() - 1;
        for a in min..=max {
            if Fraction::new(a, i).is_reduced() {
                sum += 1;
            }
        }
    }
    sum
}
