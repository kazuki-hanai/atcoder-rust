use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        nx: (usize, u128),
        mut s: Chars,
    }
    let mut x = nx.1;

    let mut optimized_s = Vec::<char>::new();
    for c in s {
        if let Some(last) = optimized_s.last() {
            if (*last == 'L' || *last == 'R') && c == 'U' {
                optimized_s.pop();
            } else {
                optimized_s.push(c);
            }
        } else {
            optimized_s.push(c);
        }
    }

    for c in optimized_s {
        match c {
            'U' => {
                if x % 2 == 0 {
                    x = x / 2;
                } else {
                    x = (x - 1) / 2;
                }
            }
            'L' => {
                x = 2 * x;
            }
            'R' => {
                x = 2 * x + 1;
            }
            _ => {
                panic!();
            }
        }
    }

    println!("{}", x);
}
