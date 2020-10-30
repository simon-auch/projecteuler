#[inline(always)]
pub fn time_it<F: Fn() -> ()>(f: F, loops: usize) -> f64 {
    use std::time::Instant;
    let now = Instant::now();
    for _ in 0..loops {
        f();
    }
    let total = now.elapsed().as_secs_f64();
    dbg!(total / loops as f64);
    total
}

pub fn check_bench<F: Fn() -> ()>(f: F) {
    let mut loops = 1;
    while loops < 100_000_000 && time_it(|| f(), loops) < 1.0 {
        loops *= 10;
    }
}
