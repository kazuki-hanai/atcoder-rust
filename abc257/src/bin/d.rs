use proconio::input;
use std::cmp::min;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        XYP: [(i64, i64, i64); N]
    }

    let mut min_s = 4_000_000_000;
    for i in 0..N {
        let mut left_s = 1;
        let mut right_s = 4_000_000_000;

        while left_s < right_s {
            let s = (left_s + right_s) / 2;
            let mut reached = vec![false; N];
            reached[i] = true;
            let mut next_p = vec![i];
            while !next_p.is_empty() {
                let p = next_p.pop().unwrap();
                for j in 0..N {
                    let distance = (XYP[p].0 - XYP[j].0).abs() + (XYP[p].1 - XYP[j].1).abs();
                    if !reached[j] && distance <= XYP[p].2 * s {
                        reached[j] = true;
                        next_p.push(j);
                    }
                }
            }

            let can_reach_all = reached.iter().all(|b| *b);

            if can_reach_all {
                right_s = s;
            } else {
                left_s = s + 1;
            }
        }

        min_s = min(min_s, left_s);
    }

    println!("{}", min_s);
}
