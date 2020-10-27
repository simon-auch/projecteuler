use projecteuler::permutation;

fn main() {
    assert_eq!(
        permutation::permutation_rank_mod(solve(3), 1_000_000_007),
        2294
    );
    assert_eq!(
        permutation::permutation_rank_mod(solve(5), 1_000_000_007),
        641839204
    );
    assert_eq!(solve_2(3, 1_000_000_007), 2294);
    assert_eq!(solve_2(5, 1_000_000_007), 641839204);
    dbg!(solve_2(25, 1_000_000_007));
    //dbg!(permutation::permutation_rank_mod(solve(25), 1_000_000_007));
}

fn solve(exp: usize) -> Vec<u32> {
    //idea: construct the permutation and then compute back
    let n = 2usize.pow(exp as u32);
    let mut acc = Vec::new();
    acc.resize_with(n, || 0);
    acc[0] = 1;

    for exp in 0..exp {
        let elements = 2usize.pow(exp as u32);
        for i in 0..elements {
            let j = i + elements;
            acc[j] = acc[i] * 2;
            acc[i] = acc[i] * 2 - 1;
        }
        if exp >= 2 {
            let m = elements;
            acc.swap(m - 1, m);
        }
    }
    acc
}

//the same as solve but keeps track of smaller elements to the left and right of each value in order to not having to "normalize" the permutation.
//reduces runtime from 5s to around 1.5s
fn solve_2(exp: usize, m: usize) -> usize {
    //idea: construct the permutation and then compute back
    let n = 2usize.pow(exp as u32);
    let mut perm = Vec::new();
    perm.resize_with(n, || (0, 0, 0));
    perm[0] = (1, 0, 0);

    for exp in 0..exp {
        let elements = 2usize.pow(exp as u32);
        for i in 0..elements {
            let j = i + elements;
            perm[j].0 = perm[i].0 * 2;
            perm[i].0 = perm[i].0 * 2 - 1;

            //
            let left = perm[i].1;
            let right = perm[i].2;
            let middle = left + right;
            perm[i].1 = left;
            perm[i].2 = middle + right;

            perm[j].1 = middle + 1 + left;
            perm[j].2 = right;
        }
        if exp >= 2 {
            let m = elements;
            perm.swap(m - 1, m);
            if perm[m - 1].0 < perm[m].0 {
                perm[m].1 += 1;
                perm[m].2 -= 1;
            } else {
                perm[m - 1].1 -= 1;
                perm[m - 1].2 += 1;
            }
        }
    }

    //compute rank of the permutation
    let mut acc = 0u128;
    let mut fact = 1;
    for i in 0..perm.len() {
        let d = perm[perm.len() - i - 1];
        acc += fact * (d.0 - d.1 - 1) as u128;
        acc %= m as u128;
        fact *= (i + 1) as u128;
        fact %= m as u128;
    }
    acc as usize
}
