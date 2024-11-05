use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut primary_diagonal_sum = 0;
    let mut secondary_diagonal_sum = 0;

    for i in 0..n {
        primary_diagonal_sum += arr[i][i];
        secondary_diagonal_sum += arr[i][n - i - 1];
    }

    (primary_diagonal_sum - secondary_diagonal_sum).abs()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let output_path = env::var("OUTPUT_PATH").unwrap_or_else(|_| "output.txt".to_string());
    let mut fptr = File::create(output_path).expect("Не вдалося створити файл для запису результату");

    println!("Введіть розмір квадратної матриці:");
    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<usize>()
        .expect("Невірний формат числа");

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n);

    println!("Введіть елементи матриці построчно, розділені пробілами:");
    for i in 0..n {
        arr.push(Vec::with_capacity(n));
        arr[i] = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.parse::<i32>().expect("Невірний формат числа"))
            .collect();
    }

    let result = diagonal_difference(&arr);

    println!("Різниця між діагоналями: {}", result);

    writeln!(&mut fptr, "{}", result).expect("Не вдалося записати результат у файл");
}
