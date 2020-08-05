use num::traits::NumAssignRef;

///returns the number of ways to split n into integers in ascending order:
/// for example partition(5) = 7
/// because:
/// 5
/// 4+1
/// 3+1+1
/// 3+2
/// 2+2+1
/// 2+1+1+1
/// 1+1+1+1+1+1

pub fn partition(n: usize) -> usize {
    partition_into::<_, _, usize>(n, 1..=n)
}
///returns the number of ways to split n into integers from set in ascending order
/// set must generate the items in ascending order
pub fn partition_into<
    Set: IntoIterator<Item = T>,
    T: core::borrow::Borrow<usize>,
    Sum: NumAssignRef,
>(
    n: usize,
    set: Set,
) -> Sum {
    let mut count: Vec<_> = (0..=n).map(|_| Sum::zero()).collect();
    count[0] = Sum::one();
    for s in set.into_iter() {
        let s = *(s.borrow());
        for i in s..=n {
            let (start, end) = count.split_at_mut(i);
            //count[i] += count[i-s]
            end[0] += &start[i - s];
        }
    }
    count.pop().unwrap()
}

//the following compute the same but require more memory (n² instead of n).
//they can however be adapted to only require O(n) additional time to add the next value

pub struct PartitionIterator<Sum> {
    counts: Vec<Vec<Sum>>,
}
impl<Sum: NumAssignRef> PartitionIterator<Sum> {
    pub fn new() -> Self {
        let mut counts = Vec::new();
        counts.push(Vec::new());
        counts[0].push(Sum::one());
        Self { counts: counts }
    }
}

impl<Sum: NumAssignRef + Clone> Iterator for PartitionIterator<Sum> {
    type Item = Sum;
    fn next(&mut self) -> Option<Self::Item> {
        let n = self.counts.len();
        let mut new_count: Vec<Sum> = (0..=n).map(|_| Sum::zero()).collect();
        //self.counts.push(count);
        for j in 1..=n {
            if self.counts[n - j].len() > 1 {
                new_count[n - j] += self.counts[n - j].pop().unwrap();
                self.counts[n - j].shrink_to_fit();
            } else {
                new_count[n - j] += &self.counts[n - j][0];
            }
            //dbg!(i, j);
            //new_count[n-j] += &new_count[n-j + 1];
            let (start, end) = new_count.split_at_mut(n - j + 1);
            start[n - j] += &end[0];
        }
        new_count.pop();
        self.counts.push(new_count);
        Some(self.counts[n][0].clone())
    }
}
