pub fn factorial(mut n: usize) -> usize {
    let mut prod = 1;
    while n > 0 {
        prod *= n;
        n -= 1;
    }
    prod
}
