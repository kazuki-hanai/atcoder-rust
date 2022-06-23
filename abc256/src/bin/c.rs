use proconio::input;

fn main() {
    input! {
        h1: i32,
        h2: i32,
        h3: i32,
        w1: i32,
        w2: i32,
        w3: i32
    };

    let mut ans = 0;
    for a11 in 1..=28 {
        for a12 in 1..=28 {
            let a13 = h1 - a11 - a12;
            if a13 < 1 {
                continue;
            }
            for a21 in 1..=28 {
                let a31 = w1 - a11 - a21;
                if a31 < 1 {
                    continue;
                }
                for a22 in 1..=28 {
                    let a23 = h2 - a22 - a21;
                    let a32 = w2 - a22 - a12;
                    let a33 = h3 - a32 - a31;

                    if a23 < 1 || a32 < 1 || a33 < 1 {
                        continue;
                    }

                    if a11 + a12 + a13 == h1
                        && a21 + a22 + a23 == h2
                        && a31 + a32 + a33 == h3
                        && a11 + a21 + a31 == w1
                        && a12 + a22 + a32 == w2
                        && a13 + a23 + a33 == w3
                    {
                        ans += 1;
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
