use num::traits::{AsPrimitive, FromPrimitive, PrimInt};

///Returns a vector with all prime numbers from 2..n
pub fn primes(n: usize) -> Vec<usize> {
    if n <= 2 {
        return vec![];
    }
    if n == 3 {
        return vec![2];
    }
    //if n == 4{
    //  return vec![3]
    //}

    let mut ret = vec![2];
    let mut sieve = Vec::new();
    sieve.resize_with(n / 2, || true);

    for i in 1..n / 2 {
        if sieve[i] {
            ret.push(i * 2 + 1);
            let mut j = 3 * i + 1;
            while j < n / 2 {
                sieve[j] = false;
                j += 2 * i + 1;
            }
        }
    }
    ret
}

///Returns a vector of booleans, sieve_bool(n)[x] == true iff. x is prime or x == 0 or x == 1
pub fn sieve_bool(n: usize) -> Vec<bool> {
    let mut sieve = Vec::new();
    sieve.resize_with(n, || true);
    for i in 2..n {
        if sieve[i] {
            let mut j = i + i;
            while j < n {
                sieve[j] = false;
                j += i;
            }
        }
    }
    sieve
}

pub struct SieveDivisor<Int> {
    sieve: Vec<Int>,
}

impl<Int> SieveDivisor<Int>
where
    Int: AsPrimitive<usize> + FromPrimitive + PrimInt,
{
    pub fn new(n: Int) -> Self {
        let mut sieve = Vec::new();
        sieve.resize_with(n.as_(), || Int::zero());
        sieve[1] = Int::one();
        //normally this code could be written with some ranges, but the Int Traits are kind of limited
        let mut i = Int::one() + Int::one();
        while i < n {
            if sieve[i.as_()] == Int::zero() {
                sieve[i.as_()] = i;
                let mut j = i;
                while j < n {
                    sieve[j.as_()] = i;
                    j = j + i;
                }
            }
            i = i + Int::one();
        }
        Self { sieve: sieve }
    }
    pub fn len(&self) -> usize {
        self.sieve.len()
    }
    pub fn get_divisor(&self, n: Int) -> Int {
        self.sieve[n.as_()]
    }
    pub fn is_prime(&self, n: Int) -> bool {
        self.sieve[n.as_()] == n
    }
    // this is the core ability exposed by this Struct.
    // most other functions rely on it.
    // given a number n, it calls the closure f with p, exp, where p.pow(exp) is a divisor of n (and maximal in terms of exp)
    pub fn for_each_divisor<F>(&self, mut n: Int, mut f: F)
    where
        F: FnMut(Int, u32),
    {
        while n > Int::one() {
            let p = self.sieve[n.as_()];
            let mut exp: u32 = 0;
            while self.sieve[n.as_()] == p {
                exp += 1;
                n = n / p;
            }
            f(p, exp);
        }
    }
    pub fn sum_of_divisors<Sum: From<Int> + PrimInt>(&self, n: Int) -> Sum {
        let mut prod = Sum::one();
        self.for_each_divisor(n, |p, exp| {
            let p: Sum = std::convert::From::from(p);
            let exp = exp;
            prod = prod * (p.pow(exp + 1) - Sum::one()) / (p - Sum::one());
        });
        prod
    }
}

struct PrimeIterator {
    primes: std::collections::binary_heap::BinaryHeap<std::cmp::Reverse<(usize, usize)>>,
    next: usize,
}

impl PrimeIterator {
    fn new() -> Self {
        Self {
            primes: std::collections::binary_heap::BinaryHeap::new(),
            next: 1,
        }
    }
}

#[derive(Debug)]
pub enum PrimeOrFactor {
    Prime(usize),
    Factor(usize),
}

impl PrimeOrFactor {
    pub fn is_prime(&self) -> bool {
        match self {
            PrimeOrFactor::Prime(_) => true,
            _ => false,
        }
    }
    pub fn get_prime(&self) -> usize {
        match self {
            PrimeOrFactor::Prime(p) => *p,
            _ => panic!(),
        }
    }
}

impl Iterator for PrimeIterator {
    type Item = PrimeOrFactor;

    fn next(&mut self) -> Option<Self::Item> {
        self.next += 1;
        //largest divisor
        let mut div = 0;
        while let Some(mut p) = self.primes.peek_mut() {
            let p = &mut p.0; //unpack the reverse
            if p.0 == self.next {
                p.0 += p.1;
                //as the primes are sorted in ascending order, the biggest one is last
                div = p.1;
            } else if p.0 < self.next {
                p.0 += p.1;
            } else {
                if div == 0 {
                    //we found no divisor
                    break;
                } else {
                    return Some(PrimeOrFactor::Factor(div));
                }
            }
        }
        self.primes
            .push(std::cmp::Reverse((self.next * 2, self.next)));
        Some(PrimeOrFactor::Prime(self.next))
    }
}

///Returns an Iterator that generates primes
pub fn primes_iterator() -> impl Iterator<Item = PrimeOrFactor> {
    PrimeIterator::new()
}

///returns (floor(sqrt(n)), ceil(sqrt(n)))
pub fn sqrt(n: usize) -> (usize, usize) {
    //first we do a really shitty sqrt approximation
    use core::cmp::Ordering;
    let mut sqrt_range = (0, n);
    while sqrt_range.1 > 1 {
        let mid = sqrt_range.0 + sqrt_range.1 / 2;
        match mid.saturating_mul(mid).cmp(&n) {
            Ordering::Equal => {
                sqrt_range = (mid, 1);
                break;
            }
            Ordering::Greater => {
                sqrt_range.1 = sqrt_range.1 / 2;
            }
            Ordering::Less => {
                sqrt_range.1 = sqrt_range.0 + sqrt_range.1 - mid;
                sqrt_range.0 = mid;
            }
        }
    }

    let sqrt = sqrt_range.0;
    let ret = if sqrt * sqrt == n {
        (sqrt, sqrt)
    } else if sqrt * sqrt > n {
        (sqrt - 1, sqrt)
    } else {
        (sqrt, sqrt + 1)
    };

    assert!(ret.0 * ret.0 <= n);
    assert!(ret.1 * ret.1 >= n);

    ret
}

pub fn factorize(n: usize) -> Vec<usize> {
    if n == 1 {
        return vec![];
    }
    if n <= 3 {
        return vec![n];
    }

    let (_sqrt_floor, sqrt_ceil) = sqrt(n);
    //only one factor can be bigger than the sqrt.
    //if no factor is smaller or equal the sqrt, the number is prime (as it cannot have factors)

    //go through all primes below or equal sqrt until you find a factor
    for p in primes_iterator()
        .filter(|p| match p {
            PrimeOrFactor::Prime(_) => true,
            _ => false,
        })
        .map(|p| match p {
            PrimeOrFactor::Prime(p) => p,
            _ => unreachable!(),
        })
    {
        if n % p == 0 {
            let mut ret = factorize(n / p);
            ret.push(p);
            return ret;
        }
        if p > sqrt_ceil {
            break;
        }
    }
    //if we did not find a prime that was smaller than the sqrt we now the number is a prime
    vec![n]
}

//returns the factorization as pairs of prime and exponent
pub fn factorize_count(n: usize) -> Vec<(usize, usize)> {
    let factorization = factorize(n);
    let mut acc: Vec<(usize, usize)> = Vec::new();
    for p in factorization {
        if let Some(ref mut x) = acc.last_mut() {
            if x.0 == p {
                x.1 += 1;
            } else {
                acc.push((p, 1));
            }
        } else {
            acc.push((p, 1));
        }
    }
    acc
}

pub fn is_prime(n: usize) -> bool {
    //factorize(n).len() == 1
    miller(n)
}

//see https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test
pub fn miller(n: usize) -> bool {
    if n == 0 || n & 0b1 == 0 {
        return false;
    }
    let m = n - 1;
    let r = m.trailing_zeros() as usize;
    let d = m / (1 << r);
    //dbg!(n, m, r, d);

    let witnesses: &[usize] = if n < 2_047 {
        &[2]
    } else if n < 1_373_653 {
        &[2, 3]
    } else if n < 9_080_191 {
        &[31, 73]
    } else if n < 25_326_001 {
        &[2, 3, 5]
    } else if n < 3_215_031_751 {
        &[2, 3, 5, 7]
    } else if n < 4_759_123_141 {
        &[2, 7, 61]
    } else if n < 1_122_004_669_633 {
        &[2, 13, 23, 1662803]
    } else if n < 2_152_302_898_747 {
        &[2, 3, 5, 7, 11]
    } else if n < 3_474_749_660_383 {
        &[2, 3, 5, 7, 11, 13]
    } else if n < 341_550_071_728_321 {
        &[2, 3, 5, 7, 11, 13, 17]
    } else {
        panic!("To big for miller");
    };
    'witness: for &a in witnesses {
        let mut x = crate::modulo::modulo_power(a, d, n);
        if x == 1 || x == n - 1 {
            continue 'witness;
        }
        for _ in 0..r - 1 {
            x = (x * x) % n;
            if x == n - 1 {
                continue 'witness;
            }
        }
        return false;
    }
    return true;
}

//returns the number of numbers that are smaller and coprime to n
pub fn euler_phi(n: usize) -> usize {
    let factors = factorize(n);
    let mut z = n * (factors[0] - 1);
    let mut n = factors[0];
    for i in 1..factors.len() {
        if factors[i] != factors[i - 1] {
            z *= factors[i] - 1;
            n *= factors[i];
        }
    }
    //n must divide z as Phi(n) is always an integer
    z / n
}

//gleiche wie euler_phi, aber für alle zahlen von 0..n
pub fn euler_phi_list(n: usize) -> Vec<usize> {
    let mut phi: Vec<_> = (0..n).map(|i| (i, 1)).collect();
    for p in 2..phi.len() {
        if phi[p].1 != 1 {
            continue;
        }
        let mut j = p;
        while j < phi.len() {
            phi[j].0 *= p - 1;
            phi[j].1 *= p;
            j += p;
        }
    }
    phi.iter().map(|(z, n)| z / n).collect()
}
