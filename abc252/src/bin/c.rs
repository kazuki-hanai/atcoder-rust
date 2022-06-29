use proconio::input;
use std::cmp::min;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }

    let S: Vec<Vec<usize>> = (0..N)
        .map(|_| {
            input! {s: String};
            s.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();

    let mut counts = vec![vec! {0; 10}; 10];
    for i in 0..N {
        for j in 0..10 {
            counts[S[i][j]][j] += 1i32;
        }
    }

    let mut ans = 1000;
    for i in 0..10 {
        let mx = (0..10)
            .map(|j| 10 * (counts[i][j] - 1) + j as i32)
            .max()
            .unwrap();
        ans = min(ans, mx);
    }

    println!("{}", ans);
}
