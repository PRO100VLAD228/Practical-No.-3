use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn time_conversion(s: &str) -> String {
    let hours = &s[0..2];
    let minutes = &s[3..5];
    let seconds = &s[6..8];
    let period = &s[8..10];

    let mut hour: i32 = hours.parse().unwrap();

    if period == "PM" && hour != 12 {
        hour += 12;
    }
    else if period == "AM" && hour == 12 {
        hour = 0;
    }

    format!("{:02}:{:02}:{:02}", hour, minutes, seconds)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    println!("Введіть час у форматі 12-годинного формату (наприклад, 07:05:45PM):");
    let s = stdin_iterator.next().unwrap().unwrap();

    let result = time_conversion(&s);

    println!("Час у 24-годинному форматі: {}", result);

    let output_path = env::var("OUTPUT_PATH").unwrap_or_else(|_| "output.txt".to_string());
    let mut fptr = File::create(output_path).expect("Не вдалося створити файл для запису результату");

    writeln!(&mut fptr, "{}", result).ok();
}
