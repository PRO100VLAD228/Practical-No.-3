use std::io::{self, BufRead};

fn mini_max_sum(arr: &[i32]) {
    let total_sum: i32 = arr.iter().sum();
    let min_sum = total_sum - arr.iter().max().unwrap();
    let max_sum = total_sum - arr.iter().min().unwrap();
    println!("{} {}", min_sum, max_sum);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    println!("Введіть п'ять цілих чисел, розділених пробілами:");
    let arr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().expect("Будь ласка, введіть цілі числа"))
        .collect();

    if arr.len() == 5 {
        mini_max_sum(&arr);
    } else {
        println!("Будь ласка, введіть рівно п'ять чисел.");
    }
}
