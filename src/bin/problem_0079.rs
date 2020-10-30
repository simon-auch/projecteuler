use projecteuler::digits;
use projecteuler::helper;

static KEYLOG: &[u32] = &[
    319, 680, 180, 690, 129, 620, 762, 689, 762, 318, 368, 710, 720, 710, 629, 168, 160, 689, 716,
    731, 736, 729, 316, 729, 729, 710, 769, 290, 719, 680, 318, 389, 162, 289, 162, 718, 729, 319,
    790, 680, 890, 362, 319, 760, 316, 729, 380, 319, 728, 716,
];

fn main() {
    helper::check_bench(|| {
        solve_2();
    });
    assert_eq!(solve_2(), 73162890);
    //assert_eq!(solve(), 73162890);
    dbg!(solve_2());
    //dbg!(solve());
}

//much much faster than solve_2 but only works if acyclic (the passcode contains no digit two times)
fn solve_2() -> usize {
    //(used, #incomming, [#outgoing to x])
    let mut graph = vec![
        (false, 0, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        (false, 0, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        (false, 0, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        (false, 0, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        (false, 0, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        (false, 0, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        (false, 0, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        (false, 0, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        (false, 0, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        (false, 0, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    ];
    for key in KEYLOG {
        let k1 = (key / 100) as usize;
        let k2 = ((key / 10) % 10) as usize;
        let k3 = (key % 10) as usize;

        graph[k1].0 = true;
        graph[k2].0 = true;
        graph[k3].0 = true;

        graph[k1].2[k2] += 1;
        graph[k2].1 += 1;

        graph[k1].2[k3] += 1;
        graph[k3].1 += 1;

        graph[k2].2[k3] += 1;
        graph[k3].1 += 1;
    }

    //now we can bouild the passcode by always removing the node without incoming edges
    let mut passcode = 0;
    'outer: while graph.len() > 0 {
        //dbg!(&graph);
        for i in 0..graph.len() {
            if graph[i].1 > 0 || !graph[i].0 {
                continue;
            }
            passcode = passcode * 10 + i;
            for j in 0..=9 {
                graph[j].1 -= graph[i].2[j];
                graph[i].2[j] = 0;
            }
            graph[i].0 = false;
            continue 'outer;
        }
        break;
    }
    passcode
}

#[allow(dead_code)]
fn solve() -> usize {
    //reverse each keylog entry and add a one at the beginning. The one serve the purpose of not "forgetting" trailing zeroes.
    let mut keylog: Vec<_> = KEYLOG
        .iter()
        .map(|k| digits::reverse_digits(*k * 10 + 1))
        .collect();
    keylog.sort();
    keylog.dedup();
    let mut best_passcode = u128::MAX;
    recursion(keylog, 0, &mut best_passcode);
    best_passcode as usize
}

fn recursion(keylog: Vec<u32>, passcode: u128, best_passcode: &mut u128) {
    if keylog.len() == 0 {
        if passcode < *best_passcode {
            *best_passcode = passcode;
        }
        return;
    }
    if passcode > *best_passcode {
        return;
    }

    let mut digits: Vec<u8> = keylog.iter().map(|k| (k % 10) as u8).collect();
    digits.sort();
    digits.dedup();

    for d in digits {
        let sub_keylog = keylog
            .iter()
            .map(|k| if (k % 10) as u8 == d { k / 10 } else { *k })
            .filter(|k| *k != 1)
            .collect();
        //dbg!(&sub_keylog);
        //dbg!(passcode);
        recursion(sub_keylog, passcode * 10 + d as u128, best_passcode)
    }
}
