use projecteuler::helper;

fn main() {
    dbg!(solve(4));
    dbg!(solve(5));
}
fn solve(power: usize) -> usize {
    let mut sum = 0;
    //first find the upper limit of digits
    let powers: Vec<usize> = [0usize, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        .iter()
        .map(|d| (*d).pow(power as u32))
        .collect();
    let max = {
        let mut a = 0;
        let mut b = 0;
        while a >= b {
            a += powers[9];
            b *= 10;
            b += 9;
        }
        a
    };
    //dbg!(max);
    let min = 2;
    //dbg!(min);
    for i in min..=max {
        let dsum = helper::digits_iterator(i).map(|d| powers[d]).sum::<usize>();
        //dbg!(i,dsum);
        if dsum == i {
            sum += i;
        }
    }
    sum
}
