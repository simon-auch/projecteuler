#[path = "../helper.rs"]
mod helper;

fn main() {
    //dbg!(solve(17, 6).len());
    helper::time_it(
        || {
            solve(8, 4);
        },
        10_000,
    );
}

//returns the solutions for s-gonal and d digits
fn solve(s: usize, d: usize) -> Vec<Vec<(u16, u16)>> {
    let numbers: Vec<_> = (3..s + 1)
        .rev()
        .map(|s| create_numbers(|n| Psn(s, n), d))
        .collect();
    //dbg!(&numbers);
    //for i in 0..numbers.len() {
    //    dbg!(numbers[i].len());
    //}

    let mut solutions = vec![];
    let mut current = vec![];
    let mut used = vec![false; numbers.len()];
    recursion(&mut current, &mut used, &numbers, &mut solutions);
    solutions
}

fn recursion(
    current: &mut Vec<(u16, u16)>,
    used: &mut Vec<bool>,
    numbers: &[Vec<(u16, u16)>],
    solutions: &mut Vec<Vec<(u16, u16)>>,
) {
    for i in 0..if current.len() == 0 { 1 } else { numbers.len() } {
        if used[i] {
            continue;
        }
        //make a binary search for the next possible starting number
        let start = {
            if current.len() == 0 {
                0
            } else {
                match numbers[i].binary_search_by_key(&current[current.len() - 1].1, |(a, b)| *a) {
                    Ok(mut some_start) => {
                        while some_start > 0
                            && current[current.len() - 1].1 == numbers[i][some_start - 1].0
                        {
                            some_start -= 1;
                        }
                        some_start
                    }
                    Err(_) => continue,
                }
            }
        };
        used[i] = true;
        for j in start..numbers[i].len() {
            let new = numbers[i][j];
            //from the binary search we know that once we encounter an invalid one, the rest will be invalid too.
            if current.len() > 0 && current[current.len() - 1].1 != new.0 {
                break;
            }
            if current.len() == 0
                || current[current.len() - 1].1 == new.0
                    && (current.len() < numbers.len() - 1 || current[0].0 == new.1)
            {
                current.push(new);
                if current.len() == numbers.len() {
                    solutions.push(current.clone());
                } else {
                    recursion(current, used, numbers, solutions);
                }
                current.pop();
            }
        }
        used[i] = false;
    }
}

fn create_numbers<F: Fn(usize) -> usize>(f: F, d: usize) -> Vec<(u16, u16)> {
    let min = 10usize.pow(d as u32 - 1) - 1;
    let max = 10usize.pow(d as u32) - 1;
    let div = 10usize.pow(d as u32 / 2);
    let min_b = 10u16.pow(d as u32 / 2 - 1); //the second part cant start with a zero
    (1..)
        .map(|n| f(n))
        .filter(|n| *n > min)
        .take_while(|n| *n <= max)
        .map(|n| ((n / div) as u16, (n % div) as u16))
        .filter(|(a, b)| *b >= min_b)
        .collect()
}

fn Psn(s: usize, n: usize) -> usize {
    ((s - 2) * (n * n) + 4 * n - s * n) / 2
}
