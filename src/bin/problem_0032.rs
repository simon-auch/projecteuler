#[path = "../helper.rs"]
mod helper;

fn main() {
    //called multiple times with different loop counts in order to check if the compiler just removes it
    //if it optimized it away the measured time per loop would decrease with increased loop count
    helper::time_it(
        || {
            solve();
        },
        10,
    );
    helper::time_it(
        || {
            solve();
        },
        100,
    );
    helper::time_it(
        || {
            solve();
        },
        1000,
    );
    assert_eq!(solve(), 45228);
    dbg!(solve());
}

//the loops are just soo much more readable if not moved around
#[rustfmt::skip]
fn solve() -> usize {
    //let mut digits: Vec<usize> = digits.iter().cloned().collect();
    //digits.sort();
    //let mut a = Vec::with_capacity(10);
    //let mut b = Vec::with_capacity(10);
    
    //multiplying n digit number with m digit number results in either a n+m or n+m-1 digit number
    //consider we have d digits the following must hold:
    //(n+m = c || n+m-1 = c) && n+m+c = d
    //(n+m = c || n+m-1 = c) && c = d-n-m
    //n+m = d-n-m || n+m-1 = d-n-m
    //2n+2m = d || 2n+2m-1 = d
    //since d = 9 we get:
    //2n+2m = 9 || 2n+2m-1 = 9
    //false || n+m = 5
    //n+m = 5
    //c = 4
    //assuming n<m
    //0<n<3
    let mut acc = vec![];

    //a cannot be nine since 9*1234 is already bigger than 9999
    for a in (1..9).filter(|&x| x!=5){
        'b_start: 
        for b_1 in (1..10).filter(|&x| x!=a){
        for b_2 in (1..10).filter(|&x| x!=a && x!= b_1){
        for b_3 in (1..10).filter(|&x| x!=a && x!= b_1 && x!= b_2){
        for b_4 in (1..10).filter(|&x| x!=a && x!= b_1 && x!= b_2 && x!= b_3 && x!=5){
            let b = b_1*1000+b_2*100+b_3*10+b_4;
            let c = a*b;
            if c > 9999 {
                break 'b_start;
            }
            //dbg!(a,b,c);

            if 1000<=c {
                let mut digits: Vec<_> = helper::digits_iterator(c).collect();
                digits.sort();
                digits.dedup();
                if digits.len() == 4 && digits.into_iter().all(|d| d!=0 && d!=a && d!=b_1 && d!=b_2 && d!=b_3 && d!=b_4){
                    //dbg!(a,b,c);
                    acc.push(c);
                }
            }
        }
        }
        }
        }
    }
    //dbg!(&acc);
    for a_1 in (1..9){
    for a_2 in (1..10).filter(|&x| x!=a_1 && x != 5){
        let a = a_1*10 + a_2;
        'b_start: 
        for b_1 in (1..10).filter(|&x| x!=a_1 && x!= a_2){
        for b_2 in (1..10).filter(|&x| x!=a_1 && x!= a_2 && x!= b_1){
        for b_3 in (1..10).filter(|&x| x!=a_1 && x!= a_2 && x!= b_1 && x!= b_2 && x!=5){
            let b = b_1*100+b_2*10+b_3;
            let c = a*b;
            if c > 9999 {
                break 'b_start;
            }
            //dbg!(a,b,c);
            if 1000<=c {
                let mut digits: Vec<_> = helper::digits_iterator(c).collect();
                digits.sort();
                digits.dedup();
                if digits.len() == 4 && digits.into_iter().all(|d| d!= 0 && d!=a_1 && d!=a_2 && d!=b_1 && d!=b_2 && d!=b_3){
                    //dbg!(a,b,c);
                    acc.push(c);
                }
            }
        }
        }
        }
    }
    }
    //dbg!(&acc);
    acc.sort();
    acc.dedup();
    acc.iter().sum()
}
