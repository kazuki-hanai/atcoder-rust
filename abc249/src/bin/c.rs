use proconio::input;
use proconio::marker::Chars;
use std::cmp::max;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: i32,
    }

    let S: Vec<Vec<char>> = (0..N)
        .map(|_| {
            input! { s: Chars };
            s
        })
        .collect();

    let mut ans = 0;
    for i in 0..(1 << N) {
        let mut sum = vec![0; 26];
        for j in 0..N {
            if (i >> j) & 1 == 1 {
                for x in 0..S[j].len() {
                    sum[S[j][x] as usize - 0x61] += 1;
                }
            }
        }

        let mut tmp_ans = 0;
        for j in 0..26 {
            if sum[j] == K {
                tmp_ans += 1;
            }
        }

        ans = max(ans, tmp_ans);
    }

    println!("{}", ans);
}
