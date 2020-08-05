const NUMBERS: &[usize] = &[
    75, 95, 64, 17, 47, 82, 18, 35, 87, 10, 20, 04, 82, 47, 65, 19, 01, 23, 75, 03, 34, 88, 02, 77,
    73, 07, 63, 67, 99, 65, 04, 28, 06, 16, 70, 92, 41, 41, 26, 56, 83, 40, 80, 70, 33, 41, 48, 72,
    33, 47, 32, 37, 16, 94, 29, 53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14, 70, 11, 33, 28, 77, 73,
    17, 78, 39, 68, 17, 57, 91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48, 63, 66, 04, 68, 89,
    53, 67, 30, 73, 16, 69, 87, 40, 31, 04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23,
];

fn main() {
    dbg!(solve(2, &NUMBERS));
    dbg!(solve(15, &NUMBERS));
}
//public because its the same as for problem 67
pub fn solve(n: usize, numbers: &[usize]) -> usize {
    assert!(n * (n + 1) / 2 <= numbers.len());

    let index = |row: usize, i: usize| (row - 1) * row / 2 + i;
    let mut counts = vec![0; n];
    for i in (1..=n).rev() {
        for j in 0..i {
            counts[j] += numbers[index(i, j)];
        }
        for j in 0..counts.len() - 1 {
            if counts[j] < counts[j + 1] {
                counts[j] = counts[j + 1];
            }
        }
        let last = counts.pop();
        if i == 1 {
            return last.unwrap();
        }
    }
    panic!();
}
