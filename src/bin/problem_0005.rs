#[path = "../primes.rs"]
mod primes;

fn main() {
    let mut total_counters = vec![0; 20];
    for i in 2..=20 {
        let mut counters = vec![0; 20];
        for p in primes::factorize(i) {
            counters[p] += 1;
        }
        //dbg!(&counters);
        for i in 0..20 {
            if total_counters[i] < counters[i] {
                total_counters[i] = counters[i];
            }
        }
    }
    dbg!(&total_counters);
    let mut res = 1;
    for (i, c) in (0usize..).zip(total_counters) {
        res *= i.pow(c);
    }
    dbg!(res);
}
