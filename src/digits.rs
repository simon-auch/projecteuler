use num::traits::{AsPrimitive, FromPrimitive, PrimInt};

//returns the digits of the number
pub fn digits<Int>(mut n: Int) -> Vec<u8>
where
    Int: PrimInt + AsPrimitive<u8> + FromPrimitive,
{
    let mut digits = vec![];
    let ten = Int::from_usize(10usize).unwrap();
    while n != Int::zero() {
        digits.push((n % ten).as_());
        n = n / ten;
    }
    digits
}

pub fn digits_iterator<Int>(n: Int) -> impl Iterator<Item = u8>
where
    Int: PrimInt + AsPrimitive<u8> + FromPrimitive,
{
    DigitIter { n: n }
}

pub struct DigitIter<Int> {
    n: Int,
}

impl<Int> Iterator for DigitIter<Int>
where
    Int: PrimInt + AsPrimitive<u8> + FromPrimitive,
{
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let ten = Int::from_usize(10usize).unwrap();
        if self.n != Int::zero() {
            let ret = (self.n % ten).as_();
            self.n = self.n / ten;
            Some(ret)
        } else {
            None
        }
    }
}

pub fn from_digits<Int, I, B>(digits: I) -> Int
where
    Int: PrimInt + AsPrimitive<u8> + FromPrimitive,
    I: IntoIterator<Item = B>,
    B: core::borrow::Borrow<u8>,
{
    let ten = Int::from_usize(10usize).unwrap();
    digits.into_iter().fold(Int::zero(), |mut acc, d| {
        acc = acc * ten;
        acc + Int::from_u8(*(d.borrow())).unwrap()
    })
}

pub fn reverse_digits<Int>(n: Int) -> Int
where
    Int: PrimInt + AsPrimitive<u8> + FromPrimitive,
{
    from_digits(digits_iterator(n))
}

pub fn is_palindrome<Int>(n: Int) -> bool
where
    Int: PrimInt + AsPrimitive<u8> + FromPrimitive,
{
    digits_iterator(n)
        .zip(digits(n).iter().rev())
        .all(|(d1, &d2)| d1 == d2)
}

//concatenates two numbers n,m -> nm
pub fn concat_numbers(n: usize, m: usize) -> usize {
    //first find out how many digits m has
    let mut k = m;
    let mut i = 0;
    while k > 0 {
        k /= 10;
        i += 1;
    }
    n * (10usize.pow(i)) + m
}

pub fn digit_sum(mut n: usize) -> usize {
    let mut acc = 0;
    while n > 0 {
        let r = n % 10;
        acc += r;
        n /= 10;
    }
    acc
}
