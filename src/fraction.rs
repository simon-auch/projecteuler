#[derive(Clone, Copy, Debug)]
pub struct Fraction {
    num: usize,
    den: usize,
}

impl Fraction {
    pub fn new(num: usize, den: usize) -> Self {
        Self { num: num, den: den }
    }
    pub fn num(&self) -> usize {
        self.num
    }
    pub fn den(&self) -> usize {
        self.den
    }
    pub fn floor(&self) -> usize {
        self.num / self.den
    }
    pub fn ceil(&self) -> usize {
        (self.num + self.den - 1) / self.den
    }
    pub fn is_integer(&self) -> bool {
        (self.num % self.den) == 0
    }
    pub fn gcd(&self) -> usize {
        gcd::Gcd::gcd(self.num, self.den)
    }
    pub fn is_reduced(&self) -> bool {
        self.gcd() == 1
    }
    pub fn reduce(&mut self) {
        let gcd = self.gcd();
        self.num /= gcd;
        self.den /= gcd;
    }
}

impl core::cmp::Ord for Fraction {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        (self.num * other.den).cmp(&(self.den * other.num))
    }
}

impl core::cmp::PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl core::cmp::Eq for Fraction {}
impl core::cmp::PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        self.num * other.den == self.den * other.num
    }
}
