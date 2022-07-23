use proconio::input;

fn main() {
    input! {
        N: usize,
        X: usize,
        Y: usize,
    }

    #[derive(PartialEq)]
    enum StoneType {
        Red,
        Blue,
    }
    type Level = usize;

    let mut blue_num = 0;
    let mut stack: Vec<(StoneType, usize, usize)> = Vec::new();
    stack.push((StoneType::Red, N, 1));

    while let Some(stone) = stack.pop() {
        if stone.0 == StoneType::Red {
            if stone.1 == 1 {
                continue;
            }
            stack.push((StoneType::Red, stone.1 - 1, stone.2));
            stack.push((StoneType::Blue, stone.1, stone.2 * X));
        } else {
            if stone.1 == 1 {
                blue_num += stone.2;
                continue;
            }
            stack.push((StoneType::Red, stone.1 - 1, stone.2));
            stack.push((StoneType::Blue, stone.1 - 1, stone.2 * Y));
        }
    }

    println!("{}", blue_num);
}
