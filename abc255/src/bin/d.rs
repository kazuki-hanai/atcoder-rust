use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Q: usize,
        mut A: [i64; N],
        X: [i64; Q],
    }

    A.sort();

    let mut diff_from_left = vec![0; N];
    let mut diff_from_right = vec![0; N];
    for i in 1..N {
        diff_from_left[i] = diff_from_left[i - 1] + (A[i] - A[i - 1]) * i as i64;
        diff_from_right[(N - 1) - i] =
            diff_from_right[(N - 1) - i + 1] + (A[(N - 1) - i + 1] - A[((N - 1) - i)]) * i as i64;
    }

    for x in X.iter() {
        let i = A.partition_point(|a| a < x);
        let mut diff_sum = 0;
        if 0 < i {
            if i < N && *x == A[i] {
                diff_sum += diff_from_left[i];
            } else {
                diff_sum += diff_from_left[i - 1] + (x - A[i - 1]) * i as i64;
            }
        }
        if i == 0 {
            diff_sum += diff_from_right[i] + (A[i] - x) * N as i64;
        } else if i < N {
            if i < N - 1 && *x == A[i] {
                diff_sum += diff_from_right[i + 1] + (A[i + 1] - x) * (N - i - 1) as i64;
            } else {
                diff_sum += diff_from_right[i] + (A[i] - x) * (N - i) as i64;
            }
        }
        println!("{}", diff_sum);
    }
}
