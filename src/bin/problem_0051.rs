use projecteuler::digits;
use projecteuler::primes;

fn main() {
    dbg!(smallest_prime_family(8));
}

///returns the smallest prime that is part of a prime family of size n
fn smallest_prime_family(n: usize) -> usize {
    primes::primes_iterator()
        .filter(|x| match x {
            primes::PrimeOrFactor::Prime(_) => true,
            _ => false,
        })
        .map(|x| match x {
            primes::PrimeOrFactor::Prime(p) => p,
            _ => unreachable!(),
        })
        .map(|p| (p, prime_family_size(p)))
        .filter(|(_p, fs)| *fs == n)
        .nth(0)
        .unwrap()
        .0
}

//returns the size of the prime family of the prime p
fn prime_family_size(p: usize) -> usize {
    let digits = digits::digits(p);
    //save all digits that are 0, 1, 2 ,...
    let mut digit_groups = vec![vec![]; 10];
    for (i, d) in digits.iter().enumerate() {
        digit_groups[*d as usize].push(i);
    }
    //dbg!(&digit_groups);
    //now for each subset (except the empy one) for each digit group compute the prime family size and take the max
    let mut max_family_size = 0;
    //dbg!(&digits);
    //dbg!(&digit_groups);
    for d in 0..10 {
        //dbg!(d);
        for mask in 1..(2usize.pow(digit_groups[d].len() as u32)) {
            //dbg!(mask);
            let mut family_size = 0;
            let mut digits = digits.clone();
            //if the mask contains the most significant digit, we are not allowed to replace it with 0
            let deny_0 = digit_groups[d].last() == Some(&(digits.len() - 1));
            for r in if deny_0 { 1 } else { 0 }..10 {
                //dbg!(r);
                //replace the digits with r
                let mut mask = mask;
                let mut i = 0;
                while mask > 0 {
                    if (mask & 0b1) == 1 {
                        digits[digit_groups[d][i]] = r;
                    }
                    i += 1;
                    mask >>= 1;
                }

                if primes::is_prime(digits::from_digits(&digits)) {
                    //dbg!(helper::from_digits(&digits));
                    family_size += 1;
                }
            }

            if family_size > max_family_size {
                max_family_size = family_size;
            }
        }
    }
    max_family_size
}
