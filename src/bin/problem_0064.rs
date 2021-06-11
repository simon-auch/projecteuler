use projecteuler::helper;

fn main() {
    helper::check_bench(|| {
        solve(10_000);
    });
    dbg!(solve(10_000));
    assert_eq!(solve(13), 4);
}

fn solve(n: usize) -> usize {
    return (1..)
        .map(|x| (x, x * x, (x + 1) * (x + 1)))
        .flat_map(|(x, previous_square, next_square)| {
            (previous_square + 1..next_square).map(move |non_square| (x, non_square))
        })
        .take_while(|(_x, non_square)| *non_square <= n)
        .map(|(previous_sqrt, non_square)| compute_period_length_of(previous_sqrt, non_square))
        .filter(|period_length| period_length & 0b1 == 1)
        .count();
}

fn compute_period_length_of(previous_sqrt: usize, non_square: usize) -> usize {
    let first_frame = Frame {
        nominator: 1,
        subtract_from_denominator: previous_sqrt,
    };
    let mut i = 0;
    let mut prev_frame = first_frame;
    loop {
        i += 1;
        prev_frame = prev_frame.next(non_square, previous_sqrt);
        if prev_frame == first_frame {
            return i;
        }
    }
}

/**
 * represents a fraction like:
 * nominator / (sqrt(x) - subtract_from_denominator)
 * where x is a constant we dont need to save in each frame
*/
#[derive(PartialEq, Eq, Clone, Copy)]
struct Frame {
    nominator: usize,
    subtract_from_denominator: usize,
}

impl Frame {
    fn next(&self, non_square: usize, previous_sqrt: usize) -> Self {
        let next_nominator = (non_square
            - self.subtract_from_denominator * self.subtract_from_denominator)
            / self.nominator;
        let next_extracted = (self.subtract_from_denominator + previous_sqrt) / next_nominator;
        let next_subtract_from_denominator =
            next_extracted * next_nominator - self.subtract_from_denominator;
        Frame {
            nominator: next_nominator,
            subtract_from_denominator: next_subtract_from_denominator,
        }
    }
}
