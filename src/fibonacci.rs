use num::traits::PrimInt;
pub struct FibonacciIterator<Int> {
    a: Int,
    b: Int,
}

impl<Int> FibonacciIterator<Int>
where
    Int: PrimInt,
{
    pub fn new() -> Self {
        Self {
            a: Int::zero(),
            b: Int::one(),
        }
    }
}

impl<Int> Iterator for FibonacciIterator<Int>
where
    Int: PrimInt,
{
    type Item = Int;

    fn next(&mut self) -> Option<Self::Item> {
        let t = self.a + self.b;
        self.a = self.b;
        self.b = t;
        Some(t)
    }
}

pub struct FibonacciIteratorMod<Int> {
    a: Int,
    b: Int,
    m: Int,
}

impl<Int> FibonacciIteratorMod<Int>
where
    Int: PrimInt,
{
    pub fn new(m: Int) -> Self {
        Self {
            a: Int::zero(),
            b: Int::one(),
            m: m,
        }
    }
}

impl<Int> Iterator for FibonacciIteratorMod<Int>
where
    Int: PrimInt,
{
    type Item = Int;

    fn next(&mut self) -> Option<Self::Item> {
        let t = (self.a + self.b) % self.m;
        self.a = self.b;
        self.b = t;
        Some(t)
    }
}
