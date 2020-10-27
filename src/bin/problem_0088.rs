use fraction::Fraction;
use projecteuler::fraction;

fn main() {
    dbg!(min_product_sum(2));
    dbg!(min_product_sum(64));
    dbg!(solve(2, 6));
    dbg!(solve(2, 12000));
}

fn solve(low: usize, high: usize) -> usize {
    let mut values = Vec::new();
    for i in low..=high {
        let min = min_product_sum(i);
        values.push(min);
        //dbg!((i, min));
    }
    values.sort();
    values.dedup();
    values.into_iter().sum::<usize>()
}

fn min_product_sum(n: usize) -> usize {
    if n == 0 {
        panic!();
    }
    if n == 1 {
        return 1;
    }
    let mut min = None;
    let mut numbers = vec![1; n];
    numbers[1] = 2;
    let mut prod = 2;
    let mut sum = n + 1;
    loop {
        //dbg!(&numbers);
        //dbg!(sum);
        //dbg!(prod);
        //fix numbers[0] so that prod = sum
        let sum_tail = sum - numbers[0];
        let prod_tail = prod / numbers[0];
        let new_value = Fraction::new(sum_tail, prod_tail - 1);
        if new_value.is_integer() {
            //
            let new_value = new_value.floor();
            sum = sum_tail + new_value;
            prod = prod_tail * new_value;
            numbers[0] = new_value;
            //found a solution, remember it if its smaller than the last one
            if min.is_none() || min.unwrap() > sum {
                min = Some(sum);
            }
        }
        //increment numbers[i] if the minimum prod of numbers stays below the minimum sum
        let mut sum_tail = sum - numbers[0];
        let mut prod_tail = prod / numbers[0];
        for i in 1..n {
            sum_tail -= numbers[i];
            prod_tail /= numbers[i];

            let next_value = numbers[i] + 1;

            let min_sum = sum_tail + next_value * (i + 1);
            let min_prod = next_value
                .checked_pow((i + 1) as u32)
                .and_then(|x| x.checked_mul(prod_tail));
            //let min_prod = prod_tail * next_value.pow((i + 1) as u32);
            if min_prod.is_some() && min_prod.unwrap() <= min_sum {
                //there might be a solution
                for j in 0..=i {
                    numbers[j] = next_value;
                }
                sum = min_sum;
                prod = min_prod.unwrap();
                break;
            }
            if i == n - 1 {
                //if the product is always going to be bigger than the sum
                //there is nothing left we can do
                return min.unwrap();
            }
        }
    }
}
