use projecteuler::helper;
use projecteuler::modulo;

fn main() {
    helper::check_bench(|| {
        solve();
    });
    assert_eq!(solve(), 8739992577);
    dbg!(solve());
}

fn solve() -> usize {
    let m = 10_000_000_000;
    let exp = 7830457;
    let mut t = modulo::modulo_power(2u128, exp, m);
    t *= 28433;
    t += 1;
    (t % m) as usize
}
