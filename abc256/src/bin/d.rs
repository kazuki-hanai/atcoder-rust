use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        LR: [(i32, i32); N]
    }

    let mut logs = vec![];
    for (l, r) in LR {
        logs.push((l, 0));
        logs.push((r, 1));
    }
    logs.sort();

    let mut login_num = 0;
    for (t, state) in logs {
        if state == 0 {
            if login_num == 0 {
                print!("{} ", t);
            }
            login_num += 1;
        } else {
            if login_num == 1 {
                println!("{}", t);
            }
            login_num -= 1;
        }
    }
}
