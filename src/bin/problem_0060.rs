//works but still a bit slow (around 40s)

use projecteuler::helper;
use projecteuler::primes;

const SET_SIZE: usize = 5;
fn main() {
    dbg!(helper::concat_numbers(1234, 5678));
    let mut prime_generator = primes::primes_iterator()
        .filter(|x| match x {
            primes::PrimeOrFactor::Prime(_) => true,
            _ => false,
        })
        .map(|x| match x {
            primes::PrimeOrFactor::Prime(p) => p,
            _ => unreachable!(),
        });

    //drop the 2,3, and 5
    //2 and 5 cannot be in a bigger prime pair set.
    prime_generator.next();
    prime_generator.next();
    prime_generator.next();

    //remeber the 3
    let mut prime_sets = (PrimePairTree::new(), PrimePairTree::new());
    prime_sets.0.add(3);
    prime_sets.1.add(3);

    let mut smallest_sum = None;

    for p in prime_generator {
        if smallest_sum.is_some() && smallest_sum.unwrap() < p {
            break;
        }
        //dbg!(&prime_sets);
        let prime_sets = if helper::digit_sum(p) % 3 == 1 {
            &mut prime_sets.0
        } else {
            &mut prime_sets.1
        };
        if let Some(sum) = prime_sets.add(p) {
            dbg!(sum);
            if smallest_sum.is_none() || smallest_sum.unwrap() > sum {
                smallest_sum = Some(sum);
            }
        }
    }
    dbg!(smallest_sum);
}

fn is_prime_pair(n: usize, m: usize) -> bool {
    primes::is_prime(helper::concat_numbers(n, m)) && primes::is_prime(helper::concat_numbers(m, n))
}

struct PrimePairTree {
    primes: Vec<usize>,
    children: Vec<PrimePairTreeNode>,
}

struct PrimePairTreeNode {
    //index into prime_tree.primes
    prime_ind: usize,
    children: Vec<PrimePairTreeNode>,
}

impl PrimePairTree {
    fn new() -> Self {
        Self {
            primes: vec![],
            children: vec![],
        }
    }
    fn add(&mut self, p: usize) -> Option<usize> {
        //compute with which primes the new one forms a pair
        let pairs: Vec<bool> = self
            .primes
            .iter()
            .map(|old_p| is_prime_pair(*old_p, p))
            .collect();
        //special case the root node
        self.primes.push(p);
        let mut stack = vec![];
        let mut ret = 0;
        for child in &mut self.children {
            if let Some(sum) = child.add(p, &pairs, &self.primes, &mut stack) {
                if ret == 0 || ret > sum {
                    ret = sum;
                }
            }
        }
        self.children
            .push(PrimePairTreeNode::new(self.primes.len() - 1));

        if ret != 0 {
            Some(ret)
        } else {
            None
        }
    }
}

impl PrimePairTreeNode {
    fn new(prime_ind: usize) -> Self {
        Self {
            prime_ind: prime_ind,
            children: vec![],
        }
    }
    fn add(
        &mut self,
        p: usize,
        pairs: &[bool],
        primes: &[usize],
        stack: &mut Vec<usize>,
    ) -> Option<usize> {
        if !pairs[self.prime_ind] {
            return None;
        }
        stack.push(primes[self.prime_ind]);
        //its a pair
        let mut ret = 0;
        for child in &mut self.children {
            if let Some(sum) = child.add(p, pairs, primes, stack) {
                if ret == 0 || ret > sum {
                    ret = sum;
                }
            }
        }
        if stack.len() == SET_SIZE - 1 {
            let sum = stack.iter().sum::<usize>() + p;
            if ret == 0 || ret > sum {
                ret = sum;
            }
        }
        stack.pop();
        self.children.push(PrimePairTreeNode::new(primes.len() - 1));
        if ret != 0 {
            Some(ret)
        } else {
            None
        }
    }
}
