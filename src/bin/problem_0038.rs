use projecteuler::helper;

fn main() {
    //gets optimized into a nop I think
    //helper::check_bench(|| {solve(1_000_000);});
    //assert_eq!(solve(), 0);
    dbg!(solve());
}

fn solve() -> usize {
    let mut max = 0usize;
    //since j must be atleast 2, the biggest number 987654321 would consist of atleast 2 parts: 9876 5 4321
    for i in 1..9999 {
        let mut acc = 0usize;
        let mut digits_seen = [false; 9];
        let mut digits_total = 0usize;
        'loop_j: for j in 1..=9 {
            let prod = i * j;
            acc = helper::concat_numbers(acc, prod);
            let digits = helper::digits(prod);
            digits_total += digits.len();
            if digits_total > 9 {
                break;
            }
            for d in digits {
                if d == 0 {
                    break 'loop_j;
                }
                if digits_seen[d - 1] {
                    break 'loop_j;
                } else {
                    digits_seen[d - 1] = true;
                }
            }
            if digits_total == 9 && acc > max && j > 1 {
                max = acc;
                dbg!(max);
            }
        }
    }
    max
}
