use std::io::{self, BufRead};

fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let apple_count = apples.iter()
        .filter(|&&distance| a + distance >= s && a + distance <= t)
        .count();

    let orange_count = oranges.iter()
        .filter(|&&distance| b + distance >= s && b + distance <= t)
        .count();

    println!("Кількість яблук на будинку: {}", apple_count);
    println!("Кількість апельсинів на будинку: {}", orange_count);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    println!("Введіть межі будинку (s і t):");
    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let s = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let t = first_multiple_input[1].trim().parse::<i32>().unwrap();

    println!("Введіть позиції дерев (a і b):");
    let second_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let a = second_multiple_input[0].trim().parse::<i32>().unwrap();
    let b = second_multiple_input[1].trim().parse::<i32>().unwrap();

    println!("Введіть кількість яблук і апельсинів (m і n):");
    let third_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let _m = third_multiple_input[0].trim().parse::<i32>().unwrap();
    let _n = third_multiple_input[1].trim().parse::<i32>().unwrap();

    println!("Введіть відстані для кожного яблука:");
    let apples: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    println!("Введіть відстані для кожного апельсина:");
    let oranges: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    count_apples_and_oranges(s, t, a, b, &apples, &oranges);
}
