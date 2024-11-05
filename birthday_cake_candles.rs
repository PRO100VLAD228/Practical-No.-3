use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn birthday_cake_candles(candles: &[i32]) -> i32 {
    let max_height = *candles.iter().max().unwrap();
    candles.iter().filter(|&&height| height == max_height).count() as i32
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    println!("Введіть кількість свічок:");
    let _candles_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    println!("Введіть висоти свічок, розділені пробілами:");
    let candles: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().expect("Введіть цілі числа"))
        .collect();

    let result = birthday_cake_candles(&candles);

    println!("Кількість найвищих свічок, які можна задути: {}", result);

    let output_path = env::var("OUTPUT_PATH").unwrap_or_else(|_| "output.txt".to_string());
    let mut fptr = File::create(output_path).expect("Не вдалося створити файл для запису результату");

    writeln!(&mut fptr, "{}", result).ok();
}
