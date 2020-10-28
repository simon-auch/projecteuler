use projecteuler::helper;

fn main() {
    helper::check_bench(|| {
        solve(1000);
    });
    assert_eq!(solve(1000), 153);
    dbg!(solve(1000));
}

fn solve(n: usize) -> usize {
    //how does the numerator and denominator change each step?
    /*
    denominator:
    2,5,12,29,70,169,408

    D(i+1) = D(i)*2 + D(i-1)
    seems to work

    nominator:
    3,7,17,41,99,239,577

    N(i+1) = N(i)*2 + N(i-1)
    seems to work too
    */
    let mut p_nom = num::BigUint::from(1u8);
    let mut nom = num::BigUint::from(3u8);
    let mut p_den = num::BigUint::from(1u8);
    let mut den = num::BigUint::from(2u8);

    let mut acc = 0;

    for _i in 2..=n {
        let temp = nom.clone();
        nom = nom * num::BigUint::from(2u8) + p_nom;
        p_nom = temp;

        let temp = den.clone();
        den = den * num::BigUint::from(2u8) + p_den;
        p_den = temp;

        if format!("{}", nom).len() > format!("{}", den).len() {
            acc += 1;
        }
    }

    acc
}
