use std::io::{self, BufRead};

fn divisible_sum_pairs(n: i32, k: i32, ar: &[i32]) -> i32 {
    let mut count = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            if (ar[i as usize] + ar[j as usize]) % k == 0 {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = divisible_sum_pairs(n, k, &ar);

    println!("{}", result);
}
