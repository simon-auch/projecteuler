use crate::factorial;

pub struct PermutationIterator {
    next: u32,
    elements: u32,
    max: u32,
}

impl PermutationIterator {
    pub fn new(elements: u32) -> Self {
        Self {
            next: 0,
            elements: elements,
            max: factorial::factorial(elements as usize),
        }
    }
}

impl Iterator for PermutationIterator {
    type Item = Vec<u32>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next == self.max {
            return None;
        }
        self.next += 1;
        Some(permutation_from_rank(self.elements, self.next - 1))
    }
}

//not exactly what I would call a fast implementation
pub fn permutation_from_rank(elements: u32, mut n: u32) -> Vec<u32> {
    let mut digits: Vec<_> = (1..=elements).collect();
    let mut ret = Vec::with_capacity(elements as usize);
    for i in 0..elements {
        let f: u32 = factorial::factorial((elements - i - 1) as usize);
        let d = n / f;
        n %= f;
        ret.push(digits.remove(d as usize));
    }
    ret
}

//basically reverts nth_permutation
//only works for small permutations due to overflows and stuff
//also has quadratic runtime
pub fn permutation_rank(mut perm: Vec<u32>) -> u32 {
    let mut acc = 0;
    let mut f: u32 = factorial::factorial(perm.len());
    for i in 0..perm.len() {
        let d = perm[i];
        f /= (perm.len() - i) as u32;
        acc += f * (d - 1);
        for j in i + 1..perm.len() {
            if perm[j] > d {
                perm[j] -= 1;
            }
        }
    }
    acc
}

//basically the same as permutation_rank but with modular arithmetic and in much faster
//runtime should be O(n log n), dominated by normalizing the permutation
pub fn permutation_rank_mod(mut perm: Vec<u32>, m: usize) -> usize {
    dbg!("start");
    let mut acc = 0u128;
    let mut fact = 1;
    dbg!("normalizing permutation");
    normalize_permutation(&mut perm);
    dbg!("computing rank");
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
//makes it much easier to compute the permutation rank
pub fn normalize_permutation(perm: &mut [u32]) {
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
