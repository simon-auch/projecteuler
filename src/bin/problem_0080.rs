use projecteuler::{helper, square_roots};

fn main() {
    helper::check_bench(|| {
        square_roots::get_approximation(2, 1000);
    });
    //dbg!(find_minimum_solution_in_x_4(&7, &61));
    assert_eq!(solve(100, 100), 40886);
}

fn solve(n: usize, precision: u32) -> usize {
    return (1..)
        .map(|x| (x * x, (x + 1) * (x + 1)))
        .flat_map(|(previous_square, next_square)| (previous_square + 1..next_square))
        .take_while(|non_square| *non_square <= n)
        .map(|n| sum_first_precision_digits(n, precision))
        .sum();
}

fn sum_first_precision_digits(n: usize, precision: u32) -> usize {
    square_roots::get_approximation(n, precision)
        .to_radix_be(10)
        .iter()
        .take(precision as usize)
        .map(|n| *n as usize)
        .sum::<usize>()
}
