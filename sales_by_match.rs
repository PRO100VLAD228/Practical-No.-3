use std::io::{self, BufRead};
use std::collections::HashMap;

fn sock_merchant(ar: &[i32]) -> i32 {
    let mut color_count = HashMap::new();

    for &color in ar {
        *color_count.entry(color).or_insert(0) += 1;
    }

    color_count.values().map(|&count| count / 2).sum()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap(); 

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let result = sock_merchant(&ar);

    println!("{}", result); 
}
