use projecteuler::permutation;

fn main() {
    assert_eq!(custom_permutation_index(solve(3), 1_000_000_007), 2294);
    assert_eq!(custom_permutation_index(solve(5), 1_000_000_007), 641839204);
    assert_eq!(solve_2(3, 1_000_000_007), 2294);
    assert_eq!(solve_2(5, 1_000_000_007), 641839204);
    dbg!(solve_2(25, 1_000_000_007));
    //dbg!(custom_permutation_index(solve(25), 1_000_000_007));
}

//basically the same as permutation::permutation_index as the time of writing but with modular arithmetic and in much faster
fn custom_permutation_index(mut perm: Vec<u32>, m: usize) -> usize {
    dbg!("start");
    let mut acc = 0u128;
    let mut fact = 1;
    dbg!("normalizing permutation");
    normalize_permutation(&mut perm);
    dbg!("computing index");
    for i in 0..perm.len() {
        let d = perm[perm.len() - i - 1];
        acc += fact * (d - 1) as u128;
        acc %= m as u128;
        fact *= (i + 1) as u128;
        fact %= m as u128;
    }
    acc as usize
}

//normalizes a permutation:
//(1,2,3,4) -> (1,1,1,1)
//(1,3,4,2) -> (1,2,2,1)
//(4,3,2,1) -> (4,3,2,1)
//basically it replaces every number with 1 + the amount of smaller numbers that follow
//makes it much easier to compute the permutation index
fn normalize_permutation(perm: &mut [u32]) {
    let mut vec: Vec<_> = perm.iter().cloned().enumerate().collect();
    let mut out: Vec<_> = Vec::new();
    out.resize_with(vec.len(), || (0usize, 0u32));

    let mut exp = 0;
    while 2usize.pow(exp) < vec.len() {
        //elements the merge step would consider from each slice
        let m = 2usize.pow(exp);
        for (chunk, chunk_out) in vec.chunks_mut(2 * m).zip(out.chunks_mut(2 * m)) {
            if chunk.len() <= m {
                continue;
            }
            let left = &chunk[0..m];
            let right = &chunk[m..];

            let mut i_left = 0;
            let mut i_right = 0;

            for out in chunk_out.iter_mut() {
                if i_right == right.len()
                    || i_left < left.len() && left[i_left].1 < right[i_right].1
                {
                    *out = left[i_left];
                    i_left += 1;
                } else {
                    *out = right[i_right];
                    perm[right[i_right].0] -= i_left as u32;
                    i_right += 1;
                }
            }
        }
        core::mem::swap(&mut vec, &mut out);
        exp += 1;
    }
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

    //compute index of the permutation
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
