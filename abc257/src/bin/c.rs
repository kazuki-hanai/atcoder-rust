use proconio::input;
use proconio::marker::Chars;
use std::cmp::max;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: Chars,
        mut W: [i64; N]
    }

    let mut WS: Vec<(&i64, char)> = W.iter().zip(S).collect();
    WS.sort_by(|a, b| a.0.cmp(b.0));

    let mut match_count = WS.iter().filter(|ws| ws.1 == '1').count();
    let mut max_count = match_count;
    for i in 0..N {
        if WS[i].1 == '0' {
            match_count += 1;
        } else {
            match_count -= 1;
        }

        if i < WS.len() - 1 && WS[i].0 != WS[i + 1].0 || i == WS.len() - 1 {
            max_count = max(max_count, match_count);
        }
    }

    println!("{}", max_count);
}
