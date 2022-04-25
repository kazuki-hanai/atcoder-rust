fn main() {
    proconio::input! {
        mut s: proconio::marker::Chars,
    };
    s.sort_by(|a, b| a.cmp(b));
    println!("{}", String::from_iter(s));
}
