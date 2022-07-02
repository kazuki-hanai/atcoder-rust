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

    let mut rw = vec![0; N + 1];
    for i in 0..N {
        rw[i + 1] = rw[i] + A[i];
    }

    for x in X {
        let (mut st, mut fi) = (0i64, (N - 1) as i64);
        while st <= fi {
            let te: i64 = (st + fi) as i64 / 2;
            if A[te as usize] < x {
                st = te + 1;
            } else {
                fi = te - 1;
            }
        }

        let mut res = x * st as i64;
        res -= rw[(fi + 1) as usize];
        res += rw[N] - rw[st as usize];
        res -= x * (N - st as usize) as i64;
        println!("{}", res);
    }
}
