fn main() {
    println!(
        "{}",
        sum_divisible_by(3, 999) + sum_divisible_by(5, 999) - sum_divisible_by(15, 999)
    );
}

///computes the sum of all numbers divisible by val and smaller or equal to max
fn sum_divisible_by(val: usize, max: usize) -> usize {
    //basically the same trick as counting all numbers from 0..n
    //sum of 0,val,2 val, 3 val, ... x val
    let numbers = max / val;
    let biggest = numbers * val;
    if numbers % 2 == 0 {
        (numbers / 2) * (biggest + val)
    } else {
        (numbers / 2) * (biggest + 2 * val) + val
    }
}
