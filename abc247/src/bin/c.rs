use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize
    }

    let mut s = "1".to_owned();

    for i in 2..=N {
        s = format!("{} {} {}", s, i, s);
    }

    println!("{}", s);
}
