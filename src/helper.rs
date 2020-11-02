#[inline(always)]
pub fn time_it<F: Fn() -> R, R>(f: F, loops: usize) -> f64 {
    use std::time::Instant;
    let now = Instant::now();
    for _ in 0..loops {
        f();
    }
    let total = now.elapsed().as_secs_f64();
    dbg!(total / loops as f64);
    total
}

pub fn get_usize() -> usize {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("IO Error while reading from stdin.");
    buffer
        .trim()
        .parse::<usize>()
        .expect("Could not parse input as usize")
}

pub fn check_bench<F: Fn() -> R, R>(f: F) {
    let mut loops = 1;
    while loops < 100_000_000 && time_it(|| black_box(&f)(), loops) < 1.0 {
        loops *= 10;
    }
}

pub fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = core::ptr::read_volatile(&dummy);
        core::mem::forget(dummy);
        ret
    }
}
