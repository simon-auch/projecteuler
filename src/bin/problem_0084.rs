use std::ops::{Add, AddAssign, Mul, Sub};

use num::Zero;
use projecteuler::helper;

const JAIL_PASS_THROUGH_INDEX: usize = 10;
const JAIL_INTO_INDEX: usize = 40;

type FTYPE = f64;

fn main() {
    helper::check_bench(|| {
        solve(100, 4);
    });
    dbg!(solve(100, 6));
    assert_eq!(solve(100, 4), "101524");
}

fn solve(iterations: usize, dice_sides: usize) -> String {
    let mut probabilities = [0.0; 41];
    probabilities[0] = 1.0;
    let move_count: FTYPE = (dice_sides * dice_sides) as FTYPE;

    let cc_and_ch = matrix_scale_to_float_with_scale(COMMUNITY_CHEST_AND_CHANCE_MOVE, 1.0 / 16.0);

    let roll_1_no_pair = construct_dice_matrix(dice_sides, true, false, false);
    let roll_1_with_pair = construct_dice_matrix(dice_sides, true, true, false);
    let roll_1_only_pair = matrix_subtraction(roll_1_with_pair, roll_1_no_pair);

    let roll_2_no_pair = construct_dice_matrix(dice_sides, false, false, false);
    let roll_2_with_pair = construct_dice_matrix(dice_sides, false, true, false);
    let roll_2_only_pair = matrix_subtraction(roll_2_with_pair, roll_2_no_pair);

    let roll_3 = construct_dice_matrix(dice_sides, false, true, true);

    let roll_1_no_pair = matrix_scale_to_float_with_scale(roll_1_no_pair, 1.0 / move_count);
    let roll_1_only_pair = matrix_scale_to_float_with_scale(roll_1_only_pair, 1.0 / move_count);
    let roll_2_no_pair = matrix_scale_to_float_with_scale(roll_2_no_pair, 1.0 / move_count);
    let roll_2_only_pair = matrix_scale_to_float_with_scale(roll_2_only_pair, 1.0 / move_count);
    let roll_3 = matrix_scale_to_float_with_scale(roll_3, 1.0 / move_count);

    let roll_1_no_pair = matrix_multiply(roll_1_no_pair, cc_and_ch);
    let roll_1_only_pair = matrix_multiply(roll_1_only_pair, cc_and_ch);
    let roll_2_no_pair = matrix_multiply(roll_2_no_pair, cc_and_ch);
    let roll_2_only_pair = matrix_multiply(roll_2_only_pair, cc_and_ch);
    let roll_3 = matrix_multiply(roll_3, cc_and_ch);

    let mut full_move = matrix_addition(
        roll_1_no_pair,
        matrix_multiply(
            roll_1_only_pair,
            matrix_addition(roll_2_no_pair, matrix_multiply(roll_2_only_pair, roll_3)),
        ),
    );

    // exponentially many steps at once
    for _ in 0..iterations {
        full_move = matrix_multiply(full_move, full_move);
        for moves in full_move.iter_mut() {
            let s = moves.iter().sum::<FTYPE>();
            moves.iter_mut().for_each(|p| *p /= s);
        }
    }

    probabilities = vector_matrix_multiply(probabilities, full_move);

    probabilities[JAIL_PASS_THROUGH_INDEX] += probabilities[JAIL_INTO_INDEX];
    probabilities[JAIL_INTO_INDEX] = 0.0;
    let mut vec = probabilities.iter().enumerate().collect::<Vec<_>>();
    vec.sort_by(|(_, p1), (_, p2)| p1.partial_cmp(p2).unwrap().reverse());

    return format!("{:02}{:02}{:02}", vec[0].0, vec[1].0, vec[2].0);
}

#[rustfmt::skip]
const COMMUNITY_CHEST_AND_CHANCE_MOVE: [[usize;41];41] = [
//   00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40
    [16, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 16, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [01, 00, 14, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 01],
    [00, 00, 00, 16, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 16, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 16, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 16, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [01, 00, 00, 00, 01, 01, 00, 06, 00, 00, 00, 01, 01, 00, 00, 02, 00, 00, 00, 00, 00, 00, 00, 00, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 01, 01],
    [00, 00, 00, 00, 00, 00, 00, 00, 16, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 16, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 16, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 16, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 16, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 16, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 16, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 16, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 16, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 14, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 01],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 16, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 16, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 16, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 16, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [01, 00, 00, 00, 00, 01, 00, 00, 00, 00, 00, 01, 00, 00, 00, 00, 00, 00, 00, 01, 00, 00, 06, 00, 01, 02, 00, 00, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 01, 01],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 16, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 16, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 16, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 16, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 16, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 16, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 16, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 16, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 16, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 16, 00, 00, 00, 00, 00, 00, 00, 00],
    [01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 14, 00, 00, 00, 00, 00, 00, 01],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 16, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 16, 00, 00, 00, 00, 00],
    [01, 00, 00, 00, 00, 03, 00, 00, 00, 00, 00, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 01, 00, 00, 00, 00, 00, 00, 00, 00, 01, 00, 00, 06, 00, 00, 01, 01],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 16, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 16, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 16, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 16],
];

fn construct_dice_matrix(
    dice_sides: usize,
    move_out_of_jail: bool,
    move_on_pair: bool,
    move_into_jail_on_pair: bool,
) -> [[usize; 41]; 41] {
    let total_moves = dice_sides * dice_sides - if move_on_pair { 0 } else { dice_sides };
    //dice distribution
    let mut moves = Vec::with_capacity(dice_sides * 2 + 1);
    moves.push(0);
    for i in 0..=dice_sides {
        moves.push(i);
    }
    for i in 1..dice_sides {
        moves.push(dice_sides - i);
    }

    debug_assert_eq!(moves.iter().sum::<usize>(), dice_sides * dice_sides);

    if move_into_jail_on_pair || !move_on_pair {
        for (distance_moved, possibilities) in moves.iter_mut().enumerate() {
            if distance_moved % 2 == 0 && *possibilities > 0 {
                *possibilities -= 1;
            }
        }
    }

    let mut ret = [[0; 41]; 41];
    for i in 0..41 {
        ret[i][i] = total_moves;
    }

    let mut move_from_to = |from: usize, to: usize, times: usize| {
        ret[from][from] -= times;
        if to == 30 {
            ret[from][JAIL_INTO_INDEX] += times;
        } else {
            ret[from][to] += times;
        }
    };

    for n in 0..40 {
        for (distance_moved, possibilities) in moves.iter().enumerate() {
            move_from_to(n, (n + distance_moved) % 40, *possibilities);
        }
    }

    if move_out_of_jail {
        for (distance_moved, possibilities) in moves.iter().enumerate() {
            move_from_to(
                JAIL_INTO_INDEX,
                (JAIL_PASS_THROUGH_INDEX + distance_moved) % 40,
                *possibilities,
            );
        }
    }

    if move_into_jail_on_pair {
        for n in 0..40 {
            move_from_to(n, JAIL_INTO_INDEX, dice_sides);
        }
    }

    for moves in ret.iter() {
        debug_assert_eq!(moves.iter().sum::<usize>(), total_moves);
    }
    debug_assert_eq!(ret.iter().map(|moves| moves[30]).sum::<usize>(), 0);

    return ret;
}

fn vector_matrix_multiply<
    U: Copy + Mul<Output = U> + AddAssign + Zero,
    const N: usize,
    const M: usize,
>(
    a: [U; N],
    b: [[U; M]; N],
) -> [U; M] {
    matrix_multiply([a], b)[0]
}

fn matrix_multiply<
    U: Copy + Mul<Output = U> + AddAssign + Zero,
    const N: usize,
    const T: usize,
    const M: usize,
>(
    a: [[U; T]; N],
    b: [[U; M]; T],
) -> [[U; M]; N] {
    let mut ret = [[Zero::zero(); M]; N];

    for n in 0..N {
        for m in 0..M {
            for t in 0..T {
                ret[n][m] += a[n][t] * b[t][m];
            }
        }
    }

    return ret;
}

fn matrix_subtraction<U: Copy + Sub<Output = U> + Zero, const N: usize, const M: usize>(
    a: [[U; M]; N],
    b: [[U; M]; N],
) -> [[U; M]; N] {
    let mut ret = [[Zero::zero(); M]; N];

    for n in 0..N {
        for m in 0..M {
            ret[n][m] = a[n][m] - b[n][m];
        }
    }

    return ret;
}

fn matrix_addition<U: Copy + Add<Output = U> + Zero, const N: usize, const M: usize>(
    a: [[U; M]; N],
    b: [[U; M]; N],
) -> [[U; M]; N] {
    let mut ret = [[Zero::zero(); M]; N];

    for n in 0..N {
        for m in 0..M {
            ret[n][m] = a[n][m] + b[n][m];
        }
    }

    return ret;
}

fn matrix_scale_to_float_with_scale<const N: usize, const M: usize>(
    a: [[usize; M]; N],
    scale: FTYPE,
) -> [[FTYPE; M]; N] {
    let mut ret = [[0.0; M]; N];

    for n in 0..N {
        for m in 0..M {
            ret[n][m] = a[n][m] as FTYPE * scale;
        }
    }

    return ret;
}
