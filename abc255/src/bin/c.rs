use proconio::input;
use std::cmp;

#[allow(non_snake_case)]
fn main() {
    input! {
        X: i64,
        mut A: i64,
        mut D: i64,
        N: i64,
    }

    if D < 0 {
        A = A + D * (N - 1);
        D *= -1;
    }

    let mut l = 0;
    let mut r = N - 1;
    while l < r {
        let i = (l + r) / 2;
        if A + D * i < X {
            l = i + 1;
        } else {
            r = i - 1;
        }
    }

    let mut res = (A + D * l - X).abs();
    for i in cmp::max(0, l - 5)..cmp::min(l + 5, N - 1) {
        if (A + D * i - X).abs() < res {
            res = (A + D * i - X).abs();
        }
    }

    println!("{}", res);
}
