use projecteuler::digits;

fn main() {
    assert_eq!(solve(10_000), 249);
    dbg!(solve(10_000));
}

fn solve(n: usize) -> usize {
    let mut acc = 0;
    'i: for mut i in 0..n as u128 {
        for _ in 0..50 {
            //dbg!(i);
            i = i + digits::reverse_digits(i);
            if digits::is_palindrome(i) {
                continue 'i;
            }
        }
        acc += 1;
    }
    acc
}
