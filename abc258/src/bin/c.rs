use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Q: usize,
        S: Chars,
        TX: [(u32, usize); Q]
    }

    let mut shift_i = 0;
    for (t, x) in TX.iter() {
        if *t == 1 {
            shift_i = (shift_i + N - x) % N;
        } else {
            println!("{}", S[(x - 1 + shift_i) % N]);
        }
    }
}
