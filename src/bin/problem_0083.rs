use projecteuler::helper;

mod problem_0081;
use problem_0081::{solve_with_movements, MatrixCursor};

fn main() {
    helper::check_bench(|| {
        solve();
    });
    assert_eq!(solve(), 425185);
    dbg!(solve());
}

fn solve() -> usize {
    solve_with_movements(
        |cursor| {
            core::iter::once(cursor.down())
                .chain(core::iter::once(cursor.right()))
                .chain(core::iter::once(cursor.up()))
                .chain(core::iter::once(cursor.left()))
                .flatten()
        },
        |cursor| cursor.i == 79 && cursor.j == 79,
        core::iter::once(MatrixCursor::new(0, 0, 80, 80)),
    )
}
