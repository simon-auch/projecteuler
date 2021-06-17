use num::{traits::NumAssignRef, BigUint, CheckedMul};

// see Square roots by subtraction Frazer Jarvis
// http://www.afjarvis.staff.shef.ac.uk/maths/jarvisspec02.pdf
pub fn get_approximation(n: usize, precision: u32) -> BigUint {
    let five = BigUint::from(5usize);
    let ten = BigUint::from(10usize);
    let fortyfive = BigUint::from(45usize);
    let hundred = BigUint::from(100usize);

    let mut a = five.clone() * n;
    let mut b = five.clone();

    for _ in 0..precision {
        while &a >= &b {
            a -= &b;
            b += &ten;
        }
        a *= &hundred;
        b *= &ten;
        b -= &fortyfive;
    }
    b /= &ten;
    return b;
}

// TODO PERF: could this also be implemented using the approximation method?
///returns (floor(sqrt(n)), ceil(sqrt(n)))
pub fn sqrt<N: CheckedMul + NumAssignRef + Ord + Clone>(n: N) -> (N, N) {
    sqrt_with_lower_bound_hint(n, N::zero())
}

///returns (floor(sqrt(n)), ceil(sqrt(n)))
pub fn sqrt_with_lower_bound_hint<N: CheckedMul + NumAssignRef + Ord + Clone>(
    n: N,
    lower_bound_hint: N,
) -> (N, N) {
    //first we do a really shitty sqrt approximation
    use core::cmp::Ordering;
    let mut sqrt_range = (lower_bound_hint, n.clone());
    while sqrt_range.1 > N::one() {
        let mut mid = sqrt_range.1.clone() / (N::one() + N::one());
        mid += &sqrt_range.0;
        match mid.checked_mul(&mid) {
            Some(product) => match product.cmp(&n) {
                Ordering::Equal => {
                    sqrt_range = (mid, N::one());
                    break;
                }
                Ordering::Greater => {
                    sqrt_range.1 = sqrt_range.1 / (N::one() + N::one());
                }
                Ordering::Less => {
                    sqrt_range.1 = sqrt_range.0 + sqrt_range.1;
                    sqrt_range.1 -= &mid;
                    sqrt_range.0 = mid;
                }
            },
            None => {
                sqrt_range.1 = sqrt_range.1 / (N::one() + N::one());
            }
        }
    }

    let sqrt = sqrt_range.0;
    let mut squared = sqrt.clone();
    squared *= &sqrt;
    let ret = match n.cmp(&squared) {
        Ordering::Less => (sqrt.clone() - N::one(), sqrt),
        Ordering::Equal => (sqrt.clone(), sqrt),
        Ordering::Greater => (sqrt.clone(), sqrt + N::one()),
    };

    debug_assert!(ret.0.clone() * ret.0.clone() <= n);
    debug_assert!(ret.1.clone() * ret.1.clone() >= n);

    ret
}

pub fn get_continued_fraction_of(
    previous_sqrt: usize,
    non_square: usize,
) -> impl Iterator<Item = usize> {
    ContinuedFractionFrame::new(previous_sqrt, non_square)
}

/**
 * represents a fraction like:
 * nominator / (sqrt(non_square) - subtract_from_denominator)
*/
#[derive(PartialEq, Eq, Clone, Copy)]
struct ContinuedFractionFrame {
    previous_sqrt: usize,
    non_square: usize,
    nominator: usize,
    subtract_from_denominator: usize,
}

impl ContinuedFractionFrame {
    fn new(previous_sqrt: usize, non_square: usize) -> Self {
        Self {
            previous_sqrt,
            non_square,
            nominator: 1,
            subtract_from_denominator: previous_sqrt,
        }
    }
}

impl Iterator for ContinuedFractionFrame {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.nominator == 0 {
            return None;
        }

        let next_nominator = (self.non_square
            - self.subtract_from_denominator * self.subtract_from_denominator)
            / self.nominator;
        let next_extracted = (self.subtract_from_denominator + self.previous_sqrt) / next_nominator;
        let next_subtract_from_denominator =
            next_extracted * next_nominator - self.subtract_from_denominator;

        self.nominator = next_nominator;
        self.subtract_from_denominator = next_subtract_from_denominator;

        if self.nominator == 1 && self.subtract_from_denominator == self.previous_sqrt {
            self.nominator = 0;
        }

        return Some(next_extracted);
    }
}
