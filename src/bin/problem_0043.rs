#[path = "../helper.rs"]
mod helper;

fn main() {
    //gets optimized into a nop I think
    helper::check_bench(|| {
        solve();
    });
    assert_eq!(solve(), 16695334890);
    dbg!(solve());
}
#[rustfmt::skip]
fn solve() -> usize {
    //einschränkungen:
    /*
    d_4 must be even
    d_3 + d_4 + d_5 mut be divisible by three
    d_6 must be either 5 or 0
    d_6 cannot be zero (d_6d_7d_8 should be divisible by 11, if d_6 is 0 d_7 must be equal to d_8 which cannot be)
    */
    let mut acc = 0;
    for d_6  in [5].iter(){
    for d_4  in [0,2,4,6,8].iter(){
    for d_2  in [0,1,2,3,4,6,7,8,9usize].iter(){
        let d = d_2;
        if d == d_4 {continue;}
    for d_3  in [0,1,2,3,4,6,7,8,9usize].iter(){
        //d_2*100 + d_3*10 + d_4 is always even
        let d = d_3;
        if d == d_2 || d == d_4 {continue;}
    for d_5  in [0,1,2,3,4,6,7,8,9usize].iter(){
        if (d_3 + d_4 + d_5)%3 != 0 {continue;}
        let d = d_5;
        if d == d_2 || d == d_3 || d == d_4 {continue;}
    for d_7  in [0,1,2,3,4,6,7,8,9usize].iter(){
        if (d_5*100 + d_6*10 + d_7) % 7 != 0 {continue;}
        let d = d_7;
        if d == d_2 || d == d_3 || d == d_4 || d == d_5 {continue;}
    for d_8  in [0,1,2,3,4,6,7,8,9usize].iter(){
        if (d_6*100 + d_7*10 + d_8) % 11 != 0 {continue;}
        let d = d_8;
        if d == d_2 || d == d_3 || d == d_4 || d == d_5 || d == d_7{continue;}
    for d_9  in [0,1,2,3,4,6,7,8,9usize].iter(){
        if (d_7*100 + d_8*10 + d_9) % 13 != 0 {continue;}
        let d = d_9;
        if d == d_2 || d == d_3 || d == d_4 || d == d_5 || d == d_7 || d == d_8{continue;}
    for d_10 in [0,1,2,3,4,6,7,8,9usize].iter(){
        if (d_8*100 + d_9*10 + d_10) % 17 != 0 {continue;}
        let d = d_10;
        if d == d_2 || d == d_3 || d == d_4 || d == d_5 || d == d_7 || d == d_8 || d == d_9{continue;}
    for d_1  in [0,1,2,3,4,5,6,7,8,9usize].iter(){
        let d = d_1;
        if d == d_2 || d == d_3 || d == d_4 || d == d_5 || d == d_7 || d == d_8 || d == d_9 || d == d_10{continue;}
        let temp = (((((((((d_1) * 10 + d_2) * 10 + d_3)*10 +d_4)*10+d_5)*10+d_6)*10+d_7)*10+d_8)*10+d_9)*10+d_10;
        acc += temp;
        
    }
    }
    }
    }
    }
    }
    }
    }
    }
    }
    acc
}
