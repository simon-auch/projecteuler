use projecteuler::helper;

fn main() {
    helper::check_bench(|| {
        solve(100, 1_000_000);
    });
    helper::check_bench(|| {
        solve(100_000_000, 1_000_000);
    });
    helper::check_bench(|| {
        solve(100_000_000, 100_000_000);
    });
    helper::check_bench(|| {
        solve(1_000_000_000, 1_000_000_000);
    });
    assert_eq!(solve(7, 12), 7);
    assert_eq!(solve(7, 16), 5);
    assert_eq!(solve(100, 1_000_000), 4075);
    assert_eq!(solve(100_000_000, 1_000_000), 4_999_999_947_996_535);
    dbg!(solve(100, 1_000_000));
    dbg!(solve(100_000_000, 1_000_000));
    dbg!(solve(1_000_000_000, 1_000_000_000));
}

fn solve(n: usize, bound: usize) -> usize {
    //we could construct the whole pascals triangle and just count all values bigger than one million, but we can do better
    let mut count = 0;
    let mut line: Vec<_> = Vec::new();
    line.push(1);
    for i in 2..=n {
        if count != 0 && line.len() <= 3 {
            break;
        }
        let last = if count == 0 && i & 0b1 == 0 {
            let last = line.len() - 1;
            let new = line[last] * 2;
            line.push(new);

            line.len() - 1
        } else {
            line.len()
        };
        let mut prev = 0;
        for j in 0..line.len() {
            if j < last {
                let temp = line[j];
                line[j] += prev;
                prev = temp;
            }

            if line[j] >= bound {
                //elements to the right including j itself
                let elements_right = line.len() - j;
                //missing rows including the current one
                let missing_rows = n - i + 1;

                //dbg!(i, j, elements_right, missing_rows, count);

                if count == 0 {
                    //each row in the triangle has i+1 elements
                    let elements_total = if i & 0b1 == 0 {
                        elements_right * 2 - 1
                    } else {
                        elements_right * 2
                    };
                    //dbg!(elements_total);
                    let a = euler_range(elements_total, elements_total + missing_rows - 1);
                    count += a;
                } else {
                    count += elements_right * missing_rows * 2;
                }
                //dbg!(count);
                line.truncate(j);
                break;
            }
        }
    }
    //special case the linear increasing second value
    if line.len() >= 2 {
        //last line thats smaller than bound
        //i<bound
        if bound < n {
            count += (n - bound + 1) * 2;
        }
    }
    //dbg!(&line);
    if line.len() >= 3 {
        //(i-1)*(i)/2 < bound
        //i < (\sqrt(8*bound + 1) + 1)/2
        //i*2 -1 < \sqrt(8*bound + 1)
        //i*2 -1 < ceil(\sqrt(8*bound + 1))
        let (sqrt_floor, _sqrt_ceil) = projecteuler::primes::sqrt(8 * bound + 1);
        let mut i = (sqrt_floor + 1) / 2;
        while i * (i + 1) / 2 < bound {
            dbg!("dingding");
            i += 1;
        }
        if i < n {
            count += (n - i) * 2;
        }
    }

    count
}

//returns the sum 1+2+...+n
fn euler_sum(n: usize) -> usize {
    n * (n + 1) / 2
}
//returns the sum n+ n+1 + n+2 + .. + m
fn euler_range(n: usize, m: usize) -> usize {
    euler_sum(m) - euler_sum(n - 1)
}
