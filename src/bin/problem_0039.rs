#[path = "../helper.rs"]
mod helper;

fn main() {
    helper::check_bench(|| {
        solve(1_000);
    });
    assert_eq!(solve(1_000), 840);
    dbg!(solve(1_000));
}

//see https://mathworld.wolfram.com/PythagoreanTriple.html
fn solve(l: usize) -> usize {
    let mut count = Vec::with_capacity(l + 1);
    for _i in 0..=l {
        count.push(0);
    }
    recursion(3, 4, 5, &mut count);
    count
        .into_iter()
        .enumerate()
        .max_by_key(|(_, c)| *c)
        .unwrap()
        .0
}

fn recursion(a: usize, b: usize, c: usize, count: &mut [usize]) {
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
    recursion(
        1 * a + 2 * c - 2 * b,
        2 * a + 2 * c - 1 * b,
        2 * a + 3 * c - 2 * b,
        count,
    );
    recursion(
        1 * a + 2 * b + 2 * c,
        2 * a + 1 * b + 2 * c,
        2 * a + 2 * b + 3 * c,
        count,
    );
    recursion(
        2 * b + 2 * c - 1 * a,
        1 * b + 2 * c - 2 * a,
        2 * b + 3 * c - 2 * a,
        count,
    );
}
