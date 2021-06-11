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
    helper::check_bench(|| {
        solve(100_000, 10usize.pow(16));
    });
    assert_eq!(solve(7, 12), 7);
    assert_eq!(solve(7, 16), 5);
    assert_eq!(solve(100, 1_000_000), 4075);
    assert_eq!(solve(100_000_000, 1_000_000), 4_999_999_947_996_535);
    assert_eq!(solve(100_000, 10usize.pow(16)), 4_999_290_674);
    dbg!(solve(100, 1_000_000));
    dbg!(solve(100_000_000, 1_000_000));
    dbg!(solve(1_000_000_000, 1_000_000_000));
    dbg!(solve(100_000, 10usize.pow(16)));
}

fn solve(n: usize, bound: usize) -> usize {
    //we could construct the whole pascals triangle and just count all values bigger than one million, but we can do better
    let mut count = 0;
    let mut line: Vec<_> = Vec::new();
    line.push(1);
    let mut i = 1;
    while i < n {
        i += 1;

        if count != 0 && line.len() <= 3 {
            //dbg!(&line, count);
            {
                let total_elements = i + 1;
                let missing = total_elements - line.len() * 2;
                count += euler_range(missing, missing + n - i);
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
                line.truncate(j);
                break;
            }
        }
        //dbg!(&line);
        if line.len() < i / 2 + 1 {
            //how many elements the row should have:
            let total_elements = i + 1;
            //how many elements the line has
            count += total_elements - line.len() * 2;
            //dbg!(i,total_elements,line.len(),count);
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
