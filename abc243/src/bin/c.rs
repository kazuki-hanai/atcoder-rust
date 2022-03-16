use proconio::input;
use proconio::marker::Chars;
use std::cmp;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        xys: [(i32, i32);n],
        s: Chars,
    }

    let mut collision_map: HashMap<i32, (i32, i32)> = HashMap::new();
    let mut is_collision = false;
    for (i, xy) in xys.iter().enumerate() {
        let x = xy.0;
        let y = xy.1;
        let dir = s[i];
        if collision_map.contains_key(&y) {
            let (most_left, most_right) = match collision_map.get(&y) {
                Some(lr) => *lr,
                None => {
                    panic!();
                }
            };
            if dir == 'L' {
                collision_map.insert(y, (cmp::max(x, most_left), most_right));
            } else {
                collision_map.insert(y, (most_left, cmp::min(x, most_right)));
            }
        } else {
            if dir == 'L' {
                collision_map.insert(y, (x, std::i32::MAX));
            } else {
                collision_map.insert(y, (std::i32::MIN, x));
            }
        }

        let (most_left, most_right) = match collision_map.get(&y) {
            Some(lr) => *lr,
            None => {
                panic!();
            }
        };

        if most_left > most_right {
            is_collision = true;
            break;
        }
    }

    println!("{}", if is_collision { "Yes" } else { "No" });
}
