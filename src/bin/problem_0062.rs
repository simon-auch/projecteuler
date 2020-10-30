use projecteuler::helper;

fn main() {
    helper::check_bench(|| {
        solve(5);
    });
    assert_eq!(solve(5), 127035954683);
    dbg!(solve(5));
}

fn solve(perms: usize) -> usize {
    let mut counts = std::collections::BTreeMap::new();
    for i in 1.. {
        let cube = i * i * i;
        let mut digits = helper::digits(cube);
        digits.sort();
        let key: usize = helper::from_digits(digits.iter().rev());
        //dbg!(i, cube, key);
        let entry = counts.entry(key).or_insert((0, cube));
        entry.0 += 1;
        if entry.0 == perms {
            return entry.1;
        }
    }
    unreachable!();
}
