use proconio::input;
fn main() {
    input! {
        n: usize,
        q: usize,
        x: [usize; q],
    }

    fn swap(a: &mut Vec<usize>, i: usize, j: usize) {
        let tmp = a[i];
        a[i] = a[j];
        a[j] = tmp;
    }

    let mut poss: Vec<usize> = (1..=n).collect();
    let mut vals: Vec<usize> = (1..=n).collect();

    for x_elem in x {
        let p0 = poss[x_elem - 1] - 1;
        let p1 = if p0 != n - 1 { p0 + 1 } else { p0 - 1 };
        let v0 = vals[p0] - 1;
        let v1 = vals[p1] - 1;

        swap(&mut vals, p0, p1);
        swap(&mut poss, v0, v1);
    }

    for val in vals {
        print!("{} ", val);
    }
    println!();
}
