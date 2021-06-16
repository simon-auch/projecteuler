use projecteuler::helper;

fn main() {
    helper::check_bench(|| {
        solve();
    });
    assert_eq!(solve(), 6531031914842725);
}

#[rustfmt::skip]
fn solve() -> usize {
    // 10 must be on the outside, otherwise the string would have 17 digits
    let mut maximum = 0isize;
    for i0 in 1..10 {
    for i1 in 1..10 {
        if i1 == i0 {continue}
        let s = 10 + i0 + i1;
    for i2 in 1..10 {
        if i2 == i0 || 
           i2 == i1 {continue}
    let i3 = s - i2 - i1;
        if i3 <= 0  ||
           i3 >= 10 ||
           i3 == i0 || 
           i3 == i1 ||
           i3 == i2 {continue}
    for i4 in 1..10 {
        if i4 == i0 || 
           i4 == i1 || 
           i4 == i2 || 
           i4 == i3 {continue}
    let i5 = s - i4 - i3;
        if i5 <= 0  ||
           i5 >= 10 ||
           i5 == i0 || 
           i5 == i1 || 
           i5 == i2 || 
           i5 == i3 || 
           i5 == i4 {continue}
    for i6 in 1..10 {
        if i6 == i0 ||
           i6 == i1 || 
           i6 == i2 || 
           i6 == i3 || 
           i6 == i4 || 
           i6 == i5 {continue}
    let i7 = s - i6 - i5;
        if i7 <= 0  ||
           i7 >= 10 ||
           i7 == i0 || 
           i7 == i1 || 
           i7 == i2 || 
           i7 == i3 || 
           i7 == i4 || 
           i7 == i5 || 
           i7 == i6 {continue}
    let i8 = s - i7 - i0;
        if i8 <= 0  ||
           i8 >= 10 ||
           i8 == i0 || 
           i8 == i1 || 
           i8 == i2 || 
           i8 == i3 || 
           i8 == i4 || 
           i8 == i5 || 
           i8 == i6 || 
           i8 == i7 {continue}
        let groups = [[10, i0, i1], [i2, i1, i3], [i4, i3, i5], [i6, i5, i7], [i8, i7, i0]];
        let start_group = groups.iter().enumerate().min_by_key(|(_g, group)| group[0]).unwrap().0;
        let (groups_end, groups_start) = groups.split_at(start_group);
        let value = groups_start.iter().chain(groups_end.iter())
          .flatten()
          .fold(0, |acc, i| 
            if *i == 10 {
                acc * 100 + i
            } else {
                acc * 10 + i
            }
        );
        if value > maximum {
            maximum = value;
        }
    }}}}}
    return maximum as usize;
}
