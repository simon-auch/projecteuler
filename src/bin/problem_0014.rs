fn main() {
    dbg!(longest_collatz_chain(1_000_000));
    dbg!(collatz_chain(13).len());
    dbg!(collatz_chain(837799).len());
}

//returns starting number and length of the longest sequenz
fn longest_collatz_chain(n: usize) -> (usize, usize) {
    let mut chains: Vec<usize> = vec![0; n + 1];
    chains[1] = 1;

    let mut stack = vec![];

    for i in 2..=n {
        let mut i = i;
        while i > n || chains[i] == 0 {
            stack.push(i);
            i = collatz(i);
        }
        let mut chain_end_length = chains[i];
        while let Some(j) = stack.pop() {
            if j <= n {
                chains[j] = chain_end_length + 1;
            }
            chain_end_length += 1;
        }
    }

    //dbg!(&chains);

    (0usize..)
        .zip(chains.iter().cloned())
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
}

fn collatz_chain(mut n: usize) -> Vec<usize> {
    let mut acc = vec![];
    while n != 1 {
        acc.push(n);
        n = collatz(n);
    }
    acc.push(1);
    acc
}

fn collatz(n: usize) -> usize {
    if (n & 0b1) == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}

//returns the possible previous values
fn rev_collatz(n: usize) -> (usize, Option<usize>) {
    (
        2 * n,
        if (n - 1) % 3 == 0 {
            Some((n - 1) / 3)
        } else {
            None
        },
    )
}
