use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn a_very_big_sum(ar: &[i64]) -> i64 {
    ar.iter().sum()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let output_path = env::var("OUTPUT_PATH").unwrap_or_else(|_| "output.txt".to_string());
    let mut fptr = File::create(output_path).expect("Не вдалося створити файл для запису результату");

    println!("Введіть кількість чисел:");

    let _ar_count = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .expect("Невірний формат числа");

    println!("Введіть числа через пробіл:");
    let ar: Vec<i64> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i64>().expect("Невірний формат числа"))
        .collect();

    let result = a_very_big_sum(&ar);

    println!("Сума чисел: {}", result);

    writeln!(&mut fptr, "{}", result).expect("Не вдалося записати результат у файл");
}
