use proconio::input;

const MAX_P: u64 = 1_000_000;

fn main() {
    input! {
        n: u64,
    }

    let mut prime_list = vec![];
    let mut is_prime_list = vec![true; MAX_P as usize];
    for i in 2..MAX_P {
        if is_prime_list[i as usize] {
            prime_list.push(i);
            let mut j = 2;
            while i * j < MAX_P {
                is_prime_list[(i * j) as usize] = false;
                j += 1;
            }
        }
    }

    let mut ans = 0;
    let mut j = prime_list.len() - 1;
    for (i, &p) in prime_list.iter().enumerate() {
        let mut q = prime_list[j];
        while i < j && n < p * q * q * q {
            j -= 1;
            q = prime_list[j];
        }
        if i >= j {
            break;
        }
        ans += j - i;
    }

    println!("{}", ans);
}
