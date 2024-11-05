use std::io::{self, BufRead};

fn page_count(n: i32, p: i32) -> i32 {
    let from_start = p / 2;
    let from_end = (n / 2) - (p / 2);
    
    std::cmp::min(from_start, from_end)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    let n: i32 = stdin_iterator.next().unwrap().unwrap().trim().parse().unwrap();
    let p: i32 = stdin_iterator.next().unwrap().unwrap().trim().parse().unwrap();

    let result = page_count(n, p);

    println!("{}", result); 
}
