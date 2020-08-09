fn main(){
    dbg!(solve(2,6));
}

fn solve(low: usize, high: usize) -> usize{
    let mut values = Vec::new();
    for i in low..=high{
        values.push(min_product_sum(i));
    }
    values.sort();
    values.dedup();
    values.into_iter().sum::<usize>()
}

fn min_product_sum(n: usize) -> usize{
    if n == 0{
        panic!();
    }
    if n == 1 {
        return 1
    }
    let mut numbers = vec![1;n];
    numbers[0] = 2;
    numbers[1] = 2;
}