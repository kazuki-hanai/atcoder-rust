extern crate proconio;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        xys: [(i64, i64); n],
    }

    if k == 1 {
        println!("Infinity");
        return;
    }

    fn is_on(xy1: (i64, i64), xy2: (i64, i64), xy3: (i64, i64)) -> bool {
        (xy1.0 - xy2.0) * (xy1.1 - xy3.1) == (xy1.0 - xy3.0) * (xy1.1 - xy2.1) 
    }

    let mut is_checked = vec![vec![false; n]; n];

    let mut ans = 0;
    for i1 in 0..n {
        for i2 in i1 + 1..n {
            if is_checked[i1][i2] {
                continue;
            }

            let mut overlap_points = vec![i1, i2];

            let mut cnt = 2;
            for i3 in i2 + 1..n {
                if is_on(xys[i1], xys[i2], xys[i3]) {
                    overlap_points.push(i3);
                    cnt += 1;
                }
            }

            for i in &overlap_points {
                for j in &overlap_points {
                    is_checked[*i as usize][*j as usize] = true;
                }
            }

            if k <= cnt {
                ans += 1;
            }
        }
    }


    println!("{}", ans);
}
