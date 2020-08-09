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

pub struct PartitionIterator<Sum> {
    partitions: Vec<Sum>,
}
impl<Sum: NumAssignRef> PartitionIterator<Sum> {
    pub fn new() -> Self {
        let mut partitions = Vec::new();
        partitions.push(Sum::one());
        Self {
            partitions: partitions,
        }
    }
}

//using the recurrance (20) from https://mathworld.wolfram.com/PartitionFunctionP.html
impl<Sum: NumAssignRef + Clone> Iterator for PartitionIterator<Sum> {
    type Item = Sum;
    fn next(&mut self) -> Option<Self::Item> {
        let mut next = Sum::zero();
        for i in 1.. {
            let j = if i % 2 == 1 {
                (i as isize + 1) / 2
            } else {
                -(i as isize + 1) / 2
            };
            let pentagonal = (j * (3 * j - 1) / 2) as usize;
            //dbg!(pentagonal);
            if pentagonal > self.partitions.len() {
                break;
            }
            let val = &self.partitions[self.partitions.len() - pentagonal];
            let exp = (i + 1) / 2;
            if exp & 0b1 == 0 {
                next -= val;
            } else {
                next += val;
            }
        }

        //dbg!(next);
        self.partitions.push(next.clone());
        Some(next)
    }
}
