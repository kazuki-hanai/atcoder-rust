use proconio::input;
use std::collections::HashMap;
use std::vec::Vec;

fn main() {
    input! {
        n: usize,
        st: [(String, u64); n]
    }

    let mut originals_i: HashMap<&String, usize> = HashMap::new();
    for i in 0..n {
        if !originals_i.contains_key(&st[i].0) {
            originals_i.insert(&st[i].0, i);
        }
    }

    let ans = originals_i.iter().fold(0, |ans, (_, i)| {
        if st[ans].1 < st[*i].1 {
            *i
        } else if st[ans].1 == st[*i].1 && *i < ans {
            *i
        } else {
            ans
        }
    });

    println!("{}", ans + 1);
}
