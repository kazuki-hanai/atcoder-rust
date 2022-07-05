use proconio::input;

fn calc_sum_of_n(n: i64) -> i64 {
    n * (n + 1) / 2
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: i64,
        A: i64,
        B: i64,
    }

    let mut sum = calc_sum_of_n(N);
    sum -= calc_sum_of_n(N / A) * A;
    sum -= calc_sum_of_n(N / B) * B;

    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    fn lcm(a: i64, b: i64) -> i64 {
        let d = gcd(a, b);
        a / d * b
    }

    let lcm_of_a_b = lcm(A, B);
    sum += calc_sum_of_n(N / lcm_of_a_b) * lcm_of_a_b;

    println!("{}", sum);
}
