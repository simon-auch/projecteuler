#[path = "../helper.rs"]
mod helper;

fn main() {
    dbg!(is_palindrom(9009));
    dbg!(is_palindrom(9029));

    let mut palindrome = 0;
    for i in (100..1000) {
        for j in (i..1000) {
            if i * j > palindrome && is_palindrom(i * j) {
                palindrome = i * j;
            }
        }
    }
    dbg!(palindrome);
}

///returns true if n is a palindrom
fn is_palindrom(mut n: usize) -> bool {
    let digits = helper::digits(n);
    digits.iter().zip(digits.iter().rev()).all(|(a, b)| a == b)
}
