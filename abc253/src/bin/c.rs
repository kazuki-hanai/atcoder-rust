use std::collections::BTreeMap;

use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! { Q: i64 }

    let mut map = BTreeMap::new();
    for _ in 0..Q {
        input! { q: u8 }
        if q == 1 {
            input! { x: i64 }
            if let Some(cnt) = map.get_mut(&x) {
                *cnt += 1;
            } else {
                map.insert(x, 1);
            }
        } else if q == 2 {
            input! { x: i64, c: i64 };
            if let Some(cnt) = map.get_mut(&x) {
                *cnt -= c;
                if *cnt <= 0 {
                    map.remove(&x);
                }
            }
        } else if q == 3 {
            let (min_x, _) = map.iter().next().unwrap();
            let (max_x, _) = map.iter().next_back().unwrap();
            println!("{}", max_x - min_x);
        }
    }
}
