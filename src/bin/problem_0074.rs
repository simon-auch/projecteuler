fn main(){
    let mut factorials = &mut [1usize;10];
    for i in 1..=9 {
        factorials[i] = factorials[i-1] * i
    }
    let factorials: &[usize; 10] = &factorials;

    //no chain has length 0, therefore we can use 0 as "not computed" value
    let max_value_possible = 2_540_160;
    let mut chains = Vec::with_capacity(max_value_possible+1);
    for i in 0..=max_value_possible{
        chains.push(0u8);
    }

    let mut stack = vec![];
    for i in 0..1_000_000 {
        if chains[i] == 0 {
            stack.push(i);
        }
        while stack.len() > 0 {
            let next = sum_factorial_of_digits(*stack.last().unwrap(), factorials);
            if chains[next] != 0 {
                let mut length = chains[next];
                while let Some(c) = stack.pop() {
                    length += 1;
                    chains[c] = length;
                }
            } else {
                if stack.contains(&next) {
                    let mut length = 0;
                    while let Some(c) = stack.pop() {
                        length += 1;
                        chains[c] = length;
                    }
                }else {
                    stack.push(next);
                }
            }
        }
    }
    dbg!(chains.iter().filter(|c| **c == 60).count());
}

fn sum_factorial_of_digits(mut n: usize, prec: &[usize;10]) -> usize {
    let mut sum = 0;
    while {
        sum += prec[n%10];
        n /= 10;
        ; n> 0} {
    }
    sum
}