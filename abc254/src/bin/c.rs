use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [u64; N]
    }

    if K == 1 {
        println!("Yes");
        return;
    }

    let mut array_by_k: Vec<Vec<u64>> = vec![vec! {}; K];
    for i in 0..N {
        array_by_k[i % K].push(A[i]);
    }

    for i in 0..K {
        array_by_k[i].sort_by(|a, b| b.cmp(a));
    }

    let mut sorted = true;
    let mut bef = 0;
    for i in 0..N {
        if let Some(val) = array_by_k[i % K].pop() {
            if bef <= val {
                bef = val;
            } else {
                sorted = false;
                break;
            }
        } else {
            sorted = false;
            break;
        }
    }

    if sorted {
        println!("Yes");
    } else {
        println!("No");
    }
}
