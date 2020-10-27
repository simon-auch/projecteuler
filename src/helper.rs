//returns the digits of the number
pub fn digits(mut n: usize) -> Vec<u8> {
    let mut digits = vec![];
    while n != 0 {
        digits.push((n % 10) as u8);
        n = n / 10;
    }
    digits
}

pub fn digits_iterator(n: usize) -> impl Iterator<Item = u8> {
    DigitIter { n: n }
}

pub struct DigitIter {
    n: usize,
}

impl Iterator for DigitIter {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n != 0 {
            let ret = (self.n % 10) as u8;
            self.n /= 10;
            Some(ret)
        } else {
            None
        }
    }
}

pub fn from_digits(digits: &[u8]) -> usize {
    digits.iter().fold(0, |mut acc, d| {
        acc *= 10usize;
        acc + *d as usize
    })
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

#[inline(always)]
pub fn time_it<F: Fn() -> ()>(f: F, loops: usize) {
    use std::time::Instant;
    let now = Instant::now();
    for _ in 0..loops {
        f();
    }
    dbg!(now.elapsed().as_secs_f64() / loops as f64);
}

pub fn check_bench<F: Fn() -> ()>(f: F) {
    time_it(|| f(), 1);
    time_it(|| f(), 10);
    time_it(|| f(), 100);
    time_it(|| f(), 1000);
}
