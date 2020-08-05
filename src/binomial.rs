mod factorial;
use factorial::factorial;

pub fn binomial_coefficient(n: usize, mut k: usize) -> usize {
    if k > n / 2 {
        k = n - k;
    }
    let mut prod = 1;
    for i in 1..=k {
        prod *= n - k + i;
        prod /= i;
    }
    prod
    //factorial(n) / (factorial(k) * factorial(n - k))
}
