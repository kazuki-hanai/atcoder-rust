use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    };

    let mut dp = vec![vec![0usize; k + 1]; n + 1];
    dp[0][0] = 1;
    for i in 0..n {
        for j in 0..=k {
            for k_ in 1..=m {
                if k_ + j <= k {
                    dp[i + 1][j + k_] += dp[i][j];
                    dp[i + 1][j + k_] %= MOD;
                }
            }
        }
    }

    println!("{}", dp[n].iter().sum::<usize>() % MOD);
}
