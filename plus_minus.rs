use std::io::{self, BufRead};

fn plus_minus(arr: &[i32]) {
    let n = arr.len() as f64; 
    let positive_count = arr.iter().filter(|&&x| x > 0).count() as f64;
    let negative_count = arr.iter().filter(|&&x| x < 0).count() as f64;
    let zero_count = arr.iter().filter(|&&x| x == 0).count() as f64;

    println!("{:.6}", positive_count / n);
    println!("{:.6}", negative_count / n);
    println!("{:.6}", zero_count / n);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    println!("Введіть кількість елементів у масиві:");
    let _n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .expect("Невірний формат числа");

    println!("Введіть елементи масиву, розділені пробілами:");
    let arr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().expect("Невірний формат числа"))
        .collect();

    plus_minus(&arr);
}
