use proconio::input;

fn main() {
    input! {
        N: usize,
        X: usize,
        Y: usize,
    }

    let mut red_nums_table = vec![0; N + 1];
    let mut blue_nums_table = vec![0; N + 1];
    red_nums_table[N] = 1;
    blue_nums_table[N] = 0;
    for i in (2..=N).rev() {
        blue_nums_table[i] += red_nums_table[i] * X;
        red_nums_table[i - 1] = red_nums_table[i] + blue_nums_table[i];
        blue_nums_table[i - 1] = blue_nums_table[i] * Y;
    }

    println!("{}", blue_nums_table[1]);
}
