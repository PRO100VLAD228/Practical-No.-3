use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn simple_array_sum(ar: &[i32]) -> i32 {
    ar.iter().sum()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let output_path = env::var("OUTPUT_PATH").unwrap_or_else(|_| "output.txt".to_string());
    let mut fptr = File::create(output_path).expect("Не вдалося створити файл для запису результату");

    println!("Введіть кількість елементів:");

    let _ar_count = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    println!("Введіть елементи масиву (розділені пробілами):");

    let ar: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let result = simple_array_sum(&ar);

    println!("Сума елементів масиву: {}", result);

    writeln!(&mut fptr, "{}", result).expect("Не вдалося записати результат у файл");
}
