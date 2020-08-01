fn main() {
    dbg!(sum_square_difference(100));
    dbg!(sum_of_numbers(10));
    dbg!(sum_of_squares(10));
}

fn sum_square_difference(n: usize) -> usize {
    sum_of_numbers(n).pow(2) - sum_of_squares(n)
}

fn sum_of_numbers(n: usize) -> usize {
    (n.pow(2) + n.pow(1)) / 2
}

fn sum_of_squares(n: usize) -> usize {
    (n.pow(3) + n.pow(2) + sum_of_numbers(n)) / 3
}
