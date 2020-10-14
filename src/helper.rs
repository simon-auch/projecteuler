//returns the digits of the number
pub fn digits(mut n: usize) -> Vec<usize> {
    let mut digits = vec![];
    while n != 0 {
        digits.push(n % 10);
        n = n / 10;
    }
    digits
}

pub fn digits_iterator(n: usize) -> impl Iterator<Item = usize> {
    DigitIter { n: n }
}

pub struct DigitIter {
    n: usize,
}

impl Iterator for DigitIter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n != 0 {
            let ret = self.n % 10;
            self.n /= 10;
            Some(ret)
        } else {
            None
        }
    }
}

pub fn from_digits(digits: &[usize]) -> usize {
    let mut acc = 0;
    for d in digits.iter().rev() {
        acc *= 10;
        acc += d;
    }
    acc
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
