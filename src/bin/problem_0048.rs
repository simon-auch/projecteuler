#[path = "../helper.rs"]
mod helper;
#[path = "../modulo.rs"]
mod modulo;

fn main() {
    helper::check_bench(|| {
        solve(1000);
    });
    assert_eq!(solve(1000), 9110846700);
    dbg!(solve(1000));
}

//sadly this overflows for the question if only using usize (on a 64 bit machine)
//thats why I also implemented a version using 128 bit numbers :)
fn solve(n: usize) -> usize {
    let m = 10_000_000_000;
    let mut acc = 0;
    for i in 1..=n {
        acc += modulo::modulo_power_u128(i as u128, i as u128, m as u128) as usize;
        acc %= m;
    }
    acc
}
