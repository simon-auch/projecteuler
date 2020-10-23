/// Returns n.pow(exp) % m but tries to do the power efficiently and keeps the numbers small (best effort)
pub fn modulo_power(mut n: usize, mut exp: usize, m: usize) -> usize {
    //the basic idea is to convert n^exp%m into (n^(exp/2)%m)²%m
    let mut acc = 1;
    while exp != 0 {
        if exp & 0b1 == 1 {
            acc *= n;
            acc %= m;
        }
        n *= n;
        n %= m;
        exp >>= 1;
    }
    acc
}

pub fn modulo_power_u128(mut n: u128, mut exp: u128, m: u128) -> u128 {
    //the basic idea is to convert n^exp%m into (n^(exp/2)%m)²%m
    let mut acc = 1;
    while exp != 0 {
        if exp & 0b1 == 1 {
            acc *= n;
            acc %= m;
        }
        n *= n;
        n %= m;
        exp >>= 1;
    }
    acc
}
