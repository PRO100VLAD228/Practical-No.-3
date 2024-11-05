use std::io::{self, BufRead};

fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let max_a = *a.iter().max().unwrap();
    let min_b = *b.iter().min().unwrap();

    let mut count = 0;

    for x in (max_a..=min_b).step_by(max_a as usize) {
        if b.iter().all(|&bi| bi % x == 0) && a.iter().all(|&ai| x % ai == 0) {
            count += 1;
        }
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let _n = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let _m = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let brr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let total = get_total_x(&arr, &brr);

    println!("{}", total);
}
