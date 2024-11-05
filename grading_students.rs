use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn grading_students(grades: &[i32]) -> Vec<i32> {
    grades.iter().map(|&grade| {
        if grade >= 38 {
            let next_multiple_of_5 = (grade + 4) / 5 * 5;
            if next_multiple_of_5 - grade < 3 {
                next_multiple_of_5
            } else {
                grade
            }
        } else {
            grade
        }
    }).collect()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    println!("Введіть кількість оцінок:");
    let grades_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    println!("Введіть оцінки (кожна на новому рядку):");
    let mut grades: Vec<i32> = Vec::with_capacity(grades_count as usize);

    for _ in 0..grades_count {
        let grades_item = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        grades.push(grades_item);
    }

    let result = grading_students(&grades);

    println!("Оцінки після округлення:");
    for grade in &result {
        println!("{}", grade);
    }

    let output_path = env::var("OUTPUT_PATH").unwrap_or_else(|_| "output.txt".to_string());
    let mut fptr = File::create(output_path).expect("Не вдалося створити файл для запису результату");

    for i in 0..result.len() {
        writeln!(&mut fptr, "{}", result[i]).ok();
    }
}
