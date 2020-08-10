fn main() {
    dbg!(recurring_cycle_digits(1, 7));
    dbg!(solve(11));
    dbg!(solve(1000));
}

fn solve(d: usize) -> usize {
    let mut max = 0;
    let mut max_ind = 0;
    for den in 2..d {
        let rec = recurring_cycle_digits(1, den);
        if rec.len() > max {
            max = rec.len();
            max_ind = den;
        }
    }
    max_ind
}

fn recurring_cycle_digits(mut num: usize, den: usize) -> Vec<usize> {
    let mut cycle = Vec::new();
    loop {
        if num == 0 {
            break;
        }
        num = (num * 10);
        let i = cycle.iter().enumerate().filter(|(_i, x)| **x == num).nth(0);
        if let Some((i, _x)) = i {
            cycle.drain(0..i);
            break;
        } else {
            cycle.push(num);
        }
        num %= den;
    }
    cycle.iter_mut().for_each(|x| *x = *x / den);
    return cycle;
}
