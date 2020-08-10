#[path = "../primes.rs"]
mod primes;

fn main() {
    dbg!(solve(42));
    dbg!(solve(1000));
}

fn solve(max: usize) -> isize {
    //b must be positive
    //b must be prime
    //a+b+1 must be ge 2 -> a+b+1 >= 2 <=> a >= -b+1
    let mut max_count = 0;
    let mut max_prod: isize = 0;
    let p = primes::primes(max + 1);
    for b in p {
        let b = b as isize;
        for a in 1 - b..max as isize {
            //zero is always correct
            let mut count = 1;
            //dbg!((a, b, count));
            while {
                let v = count * (count + a) + b;
                v >= 2 && primes::is_prime(v as usize)
            } {
                count += 1;
                //dbg!((a, b, count));
            }
            if count > max_count {
                dbg!((a, b, count));
                max_count = count;
                max_prod = a * b;
            }
        }
    }
    max_prod
}
