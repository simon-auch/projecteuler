/*
1/8, 1/7, 1/6, 1/5, 1/4, 2/7, 1/3, 3/8, 2/5, 3/7, 1/2, 4/7, 3/5, 5/8, 2/3, 5/7, 3/4, 4/5, 5/6, 6/7, 7/8
1/8, 1/7, 1/6, 1/5, 2/8, 2/7, 2/6, 3/8, 2/5, 3/7, 4/8, 4/7, 3/5, 5/8, 4/6, 5/7, 6/8, 4/5, 5/6, 6/7, 7/8
 1    2    3    4    5    6    7    8    9    10   11   12   13   14   15   16   17   18   19   20   21


1,5,8,11,14,17,21

x/8 und x/6 teilen sich genau einen bruch, 8 und 6 haben auch genau einen gemeinsamen primfaktor. Zufall? Ich denke nicht :P
*/

/*
wie viele brüche teilen sich x/8 und x/12?

8 = 2*2*2
12 = 2*2*3

1,2,3,4,5,6,7 / 8

1/12,1/6,1/4,1/3,5/12,1/2,7/12,2/6,3/4,5/6,11/12
*/
#[path = "../helper.rs"]
mod helper;
#[path = "../primes.rs"]
mod primes;

fn main() {
    dbg!(solve(8));
    helper::time_it(
        || {
            solve(1_000_000);
        },
        100,
    );
}

fn solve(d: usize) -> usize {
    primes::euler_phi_list(d + 1).iter().skip(2).sum()
}
