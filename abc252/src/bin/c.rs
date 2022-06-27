use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }

    let S: Vec<String> = (0..N)
        .map(|_| {
            input! {s: String};
            s
        })
        .collect();

    let mut dp = [1_000_000_000; 100 + 1];
}
