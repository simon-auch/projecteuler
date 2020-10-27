use projecteuler::primes;

fn main() {
    dbg!(solve(20));
    dbg!(solve(28123));
}

fn solve(n: usize) -> usize {
    let sieve = primes::sieve_prime_biggest(n + 1);
    let mut abundant_numbers = Vec::new();
    for i in 1..=n {
        if i < primes::sum_of_divisors(i, &sieve) - i {
            abundant_numbers.push(i);
        }
    }
    dbg!(abundant_numbers.len());
    let mut combinations = Vec::with_capacity(n + 1);
    combinations.resize_with(n + 1, || false);
    for i in 0..abundant_numbers.len() {
        for j in i..abundant_numbers.len() {
            let ind = abundant_numbers[i] + abundant_numbers[j];
            if ind < combinations.len() {
                combinations[ind] = true;
            }
        }
    }
    combinations
        .iter()
        .enumerate()
        .filter(|(_i, b)| !**b)
        .map(|(i, _)| i)
        .sum::<usize>()
}
