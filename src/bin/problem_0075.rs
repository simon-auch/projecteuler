#[path = "../fraction.rs"]
mod fraction;
use fraction::Fraction;

fn main() {
    //dbg!(solve(120));
    //dbg!(solve(1_000));
    //dbg!(solve(1_000_000));
    //dbg!(solve(1_500_000)); // = 161667
    dbg!(solve2(1_500_000)); // = 161667
    let now = std::time::Instant::now();
    dbg!(solve2(500_000_000)); // = 52957114
    dbg!(now.elapsed().as_secs_f64());
}

fn solve(l: usize) -> usize {
    let mut sum = 0;
    //s must be even
    for s in 1..=l / 2 {
        let s = s * 2;
        let mut triangles = 0;
        if s % 1000 == 0 {
            dbg!(s);
        }
        //b must be smaller than s/2
        //b must be even
        for b in 1..s / 4 {
            let b = b * 2;
            let a = Fraction::new(s * (s - 2 * b), 2 * (s - b));
            if a.is_integer() {
                let a = a.floor();
                if a <= b || (a & 1) == 1 {
                    //dbg!(s, b.num());
                    triangles += 1;
                    if triangles > 1 {
                        break;
                    }
                }
            }
        }
        //a and b can always be swapped
        if triangles == 1 {
            sum += 1;
        }
    }
    sum
}

//see https://mathworld.wolfram.com/PythagoreanTriple.html
fn solve2(l: usize) -> usize {
    let mut count = Vec::with_capacity(l + 1);
    for _i in 0..=l {
        count.push(0);
    }
    recursion2(3, 4, 5, &mut count);
    count.iter().filter(|c| **c == 1).count()
}
fn recursion2(a: usize, b: usize, c: usize, count: &mut [usize]) {
    let s = a + b + c;
    if s > count.len() - 1 {
        return;
    }
    {
        let mut cs = s;
        while cs < count.len() {
            count[cs] += 1;
            cs += s;
        }
    }
    recursion2(
        1 * a + 2 * c - 2 * b,
        2 * a + 2 * c - 1 * b,
        2 * a + 3 * c - 2 * b,
        count,
    );
    recursion2(
        1 * a + 2 * b + 2 * c,
        2 * a + 1 * b + 2 * c,
        2 * a + 2 * b + 3 * c,
        count,
    );
    recursion2(
        2 * b + 2 * c - 1 * a,
        1 * b + 2 * c - 2 * a,
        2 * b + 3 * c - 2 * a,
        count,
    );
}
