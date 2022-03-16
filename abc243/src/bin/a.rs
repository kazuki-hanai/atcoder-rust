use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        xys: [(i32, i32);n],
        s: Chars,
    }

    let mut collision_map: HashMap<i32, char> = HashMap::new();
    let mut is_collision = false;
    for (i, xy) in xys.iter().enumerate() {
        let y = xy.1;
        if collision_map.contains_key(&y) {
            let dir = match collision_map.get(&y) {
                Some(y) => *y,
                None => {
                    panic!();
                }
            };
            if dir != s[i] {
                is_collision = true;
                break;
            }
        } else {
            collision_map.insert(y, s[i]);
        }
    }

    println!("{}", if is_collision { "Yes" } else { "No" });
}
