use std::io::{self, BufRead};

fn breaking_records(scores: &[i32]) -> Vec<i32> {
    let mut highest = scores[0];
    let mut lowest = scores[0];
    let mut high_count = 0;
    let mut low_count = 0;

    for &score in scores.iter().skip(1) {
        if score > highest {
            highest = score;
            high_count += 1;
        } else if score < lowest {
            lowest = score;
            low_count += 1;
        }
    }

    vec![high_count, low_count]
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let scores: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = breaking_records(&scores);

    println!("{} {}", result[0], result[1]);
}
