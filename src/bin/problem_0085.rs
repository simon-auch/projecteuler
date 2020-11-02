use projecteuler::helper;

fn main() {
    helper::check_bench(|| {
        solve(2_000_000);
    });
    assert_eq!(solve(2_000_000), 2772);
    dbg!(solve(2_000_000));
}

//solved using a boundary walk
//for the given problem the cursor is moved a total of 2210 times.
fn solve(goal: usize) -> usize {
    let mut cursor = Cursor::new();
    while cursor.count < goal {
        cursor.inc_n();
        cursor.inc_m();
    }
    let mut opt = cursor;
    while cursor.n > 0 {
        while cursor.count < goal {
            cursor.inc_m();
        }
        while cursor.count > goal {
            cursor.dec_m();
        }
        opt = best_cursor(goal, opt, cursor);
        cursor.inc_m();
        opt = best_cursor(goal, opt, cursor);
        cursor.dec_n();
    }
    opt.n * opt.m
}

fn best_cursor(goal: usize, c1: Cursor, c2: Cursor) -> Cursor {
    let dist1 = if goal < c1.count {
        c1.count - goal
    } else {
        goal - c1.count
    };
    let dist2 = if goal < c2.count {
        c2.count - goal
    } else {
        goal - c2.count
    };
    if dist1 < dist2 {
        c1
    } else {
        c2
    }
}

#[derive(Debug, Clone, Copy)]
struct Cursor {
    n: usize,
    m: usize,
    count: usize,
}

impl Cursor {
    fn new() -> Self {
        Self {
            n: 1,
            m: 1,
            count: 1,
        }
    }
    fn inc_n(&mut self) {
        self.n += 1;
        self.count += self.n * ((self.m * (self.m + 1)) / 2);
    }
    fn inc_m(&mut self) {
        self.m += 1;
        self.count += self.m * ((self.n * (self.n + 1)) / 2);
    }
    fn dec_n(&mut self) {
        self.count -= self.n * ((self.m * (self.m + 1)) / 2);
        self.n -= 1;
    }
    fn dec_m(&mut self) {
        self.count -= self.m * ((self.n * (self.n + 1)) / 2);
        self.m -= 1;
    }
}
