use num::traits::NumAssignRef;

pub fn factorial<T: NumAssignRef>(mut n: usize) -> T {
    let mut prod = T::one();
    let mut mult = T::one();
    while n > 0 {
        prod *= &mult;
        n -= 1;
        mult += T::one();
    }
    prod
}
