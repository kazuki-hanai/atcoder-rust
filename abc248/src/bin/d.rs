use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut va: [usize; n],
        q_num: usize
    }

    let mut a_pos: Vec<Vec<usize>> = vec![vec![]; n + 1];

    for (i, a) in va.iter().enumerate() {
        a_pos[*a - 1].push(i);
    }

    for _ in 0..q_num {
        input! {
            l: Usize1,
            r: Usize1,
            x: Usize1,
        }

        let lower_pos = a_pos[x].partition_point(|&v| v < l);
        let upper_pos = a_pos[x].partition_point(|&v| v <= r);

        let ret = upper_pos - lower_pos;
        println!("{}", ret);
    }
}
