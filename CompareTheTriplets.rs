use std::io::{self, BufRead};

/*
 * Complete the 'compare_triplets' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY a
 *  2. INTEGER_ARRAY b
 */

fn compare_triplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = vec![0, 0];
    for (x, y) in a.iter().zip(b.iter()) {
        if x > y {
            result[0] += 1;
        } else if x < y {
            result[1] += 1;
        }
    }
    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // Підказка для введення першого масиву
    println!("Введіть масив a (числа, розділені пробілами):");
    let a: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().expect("Невірний формат числа"))
        .collect();

    // Підказка для введення другого масиву
    println!("Введіть масив b (числа, розділені пробілами):");
    let b: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().expect("Невірний формат числа"))
        .collect();

    // Виконуємо порівняння
    let result = compare_triplets(&a, &b);

    // Виводимо результат у консоль
    println!("Результат порівняння: {} {}", result[0], result[1]);
}
