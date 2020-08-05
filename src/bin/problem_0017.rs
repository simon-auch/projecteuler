fn main() {
    dbg!(count_letters(342, &special_values()));
    dbg!(solve(1000));
}

fn special_values() -> std::collections::BTreeMap<usize, usize> {
    let mut special_values = std::collections::BTreeMap::new();
    &[
        (1usize, 3usize),
        (2, 3),
        (3, 5),
        (4, 4),
        (5, 4),
        (6, 3),
        (7, 5),
        (8, 5),
        (9, 4),
        (10, 3),
        (11, 6),
        (12, 6),
        (13, 8),
        (14, 8),
        (15, 7),
        (16, 7),
        (17, 9),
        (18, 8),
        (19, 8),
        (20, 6),
        (30, 6),
        (40, 5),
        (50, 5),
        (60, 5),
        (70, 7),
        (80, 6),
        (90, 6),
    ]
    .iter()
    .for_each(|(i, j)| {
        special_values.insert(*i, *j);
    });
    special_values
}

fn solve(n: usize) -> usize {
    (1..=n)
        .map(|i| {
            //dbg!(i);
            count_letters(i, &special_values())
        })
        .sum::<usize>()
}

fn count_letters(n: usize, special_values: &std::collections::BTreeMap<usize, usize>) -> usize {
    if n == 0 {
        panic!();
    } else if n < 20 {
        *(special_values.get(&n).unwrap())
    } else if n < 100 {
        let mut ret = *(special_values.get(&(n - (n % 10))).unwrap());
        if n % 10 != 0 {
            ret += count_letters(n % 10, special_values);
        }
        ret
    } else if n < 1000 {
        let mut ret = *(special_values.get(&(n / 100)).unwrap()) + 7; //hundred ~= 7
        if n % 100 != 0 {
            ret += count_letters(n % 100, special_values) + 3; //and ~= 3
        }
        ret
    } else if n == 1000 {
        11
    } else {
        panic!();
    }
}
