pub fn get_continued_fraction_of(
    previous_sqrt: usize,
    non_square: usize,
) -> impl Iterator<Item = usize> {
    ContinuedFractionFrame::new(previous_sqrt, non_square)
}

/**
 * represents a fraction like:
 * nominator / (sqrt(non_square) - subtract_from_denominator)
*/
#[derive(PartialEq, Eq, Clone, Copy)]
struct ContinuedFractionFrame {
    previous_sqrt: usize,
    non_square: usize,
    nominator: usize,
    subtract_from_denominator: usize,
}

impl ContinuedFractionFrame {
    fn new(previous_sqrt: usize, non_square: usize) -> Self {
        Self {
            previous_sqrt,
            non_square,
            nominator: 1,
            subtract_from_denominator: previous_sqrt,
        }
    }
}

impl Iterator for ContinuedFractionFrame {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.nominator == 0 {
            return None;
        }

        let next_nominator = (self.non_square
            - self.subtract_from_denominator * self.subtract_from_denominator)
            / self.nominator;
        let next_extracted = (self.subtract_from_denominator + self.previous_sqrt) / next_nominator;
        let next_subtract_from_denominator =
            next_extracted * next_nominator - self.subtract_from_denominator;

        self.nominator = next_nominator;
        self.subtract_from_denominator = next_subtract_from_denominator;

        if self.nominator == 1 && self.subtract_from_denominator == self.previous_sqrt {
            self.nominator = 0;
        }

        return Some(next_extracted);
    }
}
