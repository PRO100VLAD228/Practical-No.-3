use std::io::{self, BufRead};

fn staircase(n: i32) {
    for i in 1..=n {
        let spaces = " ".repeat((n - i) as usize);
        let hashes = "#".repeat(i as usize);
        println!("{}{}", spaces, hashes);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    println!("Введіть висоту сходів (ціле число):");
    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .expect("Будь ласка, введіть ціле число.");

    staircase(n);
}
