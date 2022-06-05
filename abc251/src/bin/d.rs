use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        A: [u32; n]
    }

    let mut ans = 0u64;

    let mut num_counts: HashMap<u32, Vec<usize>> = HashMap::new();
    for i in 0..n {
        if let Some(v) = num_counts.get_mut(&A[i]) {
            v.push(i);
        } else {
            let v = vec![i];
            num_counts.insert(A[i], v);
        }
    }
}
