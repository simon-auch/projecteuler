use projecteuler::helper;

mod problem_0081;
use problem_0081::{solve_with_movements, MatrixCursor};

fn main() {
    helper::check_bench(|| {
        solve();
    });
    assert_eq!(solve(), 260324);
    dbg!(solve());
}

fn solve() -> usize {
    solve_with_movements(
        |cursor| {
            core::iter::once(cursor.down())
                .chain(core::iter::once(cursor.right()))
                .chain(core::iter::once(cursor.up()))
                .flatten()
        },
        |cursor| cursor.j == 79,
        (0..80).map(|i| MatrixCursor::new(i, 0, 80, 80)),
    )
}
