use gcd::Gcd;
use num::traits::{AsPrimitive, FromPrimitive, PrimInt};

#[derive(Clone, Copy, Debug)]
pub struct Fraction<Int> {
    num: Int,
    den: Int,
}

impl<Int> Fraction<Int>
where
    Int: PrimInt,
{
    pub fn new(num: Int, den: Int) -> Self {
        Self { num: num, den: den }
    }
    pub fn num(&self) -> Int {
        self.num
    }
    pub fn den(&self) -> Int {
        self.den
    }
    pub fn floor(&self) -> Int {
        self.num / self.den
    }
    pub fn ceil(&self) -> Int {
        (self.num + self.den - Int::one()) / self.den
    }
    pub fn is_integer(&self) -> bool {
        (self.num % self.den) == Int::zero()
    }
}
impl<Int> Fraction<Int>
where
    Int: Gcd + PrimInt,
{
    pub fn gcd(&self) -> Int {
        gcd::Gcd::gcd(self.num, self.den)
    }
    pub fn is_reduced(&self) -> bool {
        self.gcd() == Int::one()
    }
    pub fn reduce(&mut self) {
        let gcd = self.gcd();
        self.num = self.num / gcd;
        self.den = self.den / gcd;
    }
}

impl<Int> core::cmp::Ord for Fraction<Int>
where
    Int: PrimInt,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        (self.num * other.den).cmp(&(self.den * other.num))
    }
}

impl<Int> core::cmp::PartialOrd for Fraction<Int>
where
    Int: PrimInt,
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<Int> core::cmp::Eq for Fraction<Int> where Int: PrimInt {}
impl<Int> core::cmp::PartialEq for Fraction<Int>
where
    Int: PrimInt,
{
    fn eq(&self, other: &Self) -> bool {
        self.num * other.den == self.den * other.num
    }
}
