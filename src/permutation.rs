use crate::factorial;

pub struct PermutationIterator {
    next: usize,
    elements: usize,
    max: usize,
}

impl PermutationIterator {
    pub fn new(elements: usize) -> Self {
        Self {
            next: 0,
            elements: elements,
            max: factorial::factorial(elements),
        }
    }
}

impl Iterator for PermutationIterator {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next == self.max {
            return None;
        }
        self.next += 1;
        Some(nth_permutation(self.elements, self.next - 1))
    }
}

pub fn nth_permutation(elements: usize, mut n: usize) -> Vec<usize> {
    let mut digits: Vec<_> = (1..=elements).collect();
    let mut ret = Vec::with_capacity(elements);
    for i in 0..elements {
        let f: usize = factorial::factorial(elements - i - 1);
        let d = n / f;
        n %= f;
        ret.push(digits.remove(d));
    }
    ret
}

//basically reverts nth_permutation
pub fn permutation_index(mut perm: Vec<usize>) -> usize {
    let mut acc = 0;
    let mut f: usize = factorial::factorial(perm.len());
    for i in 0..perm.len() {
        let d = perm[i];
        f /= perm.len() - i;
        acc += f * (d - 1);
        for j in i + 1..perm.len() {
            if perm[j] > d {
                perm[j] -= 1;
            }
        }
    }
    acc
}
