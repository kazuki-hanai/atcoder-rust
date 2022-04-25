fn main() {
    proconio::input! {
        a: u32,
        b: u32,
        c: u32,
        x: u32
    }

    let prob: f32 = if x <= a {
        1f32
    } else if a + 1 <= x && x <= b {
        c as f32 / (b - a) as f32
    } else {
        0f32
    };

    println!("{:.6}", prob);
}
