use num::traits::{AsPrimitive, FromPrimitive, PrimInt};

/// Returns n.pow(exp) % m but tries to do the power efficiently and keeps the numbers small (best effort)
pub fn modulo_power<Int>(mut n: Int, mut exp: Int, m: Int) -> Int
where
    Int: PrimInt + AsPrimitive<u8> + FromPrimitive,
{
    //the basic idea is to convert n^exp%m into (n^(exp/2)%m)²%m
    let mut acc = Int::one();
    while exp != Int::zero() {
        if exp & Int::one() == Int::one() {
            acc = acc * n;
            acc = acc % m;
        }
        n = n * n;
        n = n % m;
        exp = exp >> 1;
    }
    acc
}
