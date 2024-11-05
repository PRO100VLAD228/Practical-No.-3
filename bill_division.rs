use std::io::{self, BufRead};

fn bon_appetit(bill: &[i32], k: i32, b: i32) {
    let total: i32 = bill.iter().sum();
    let actual_charge = (total - bill[k as usize]) / 2;
    if b == actual_charge {
        println!("Bon Appetit"); 
    } else {
        println!("{}", b - actual_charge); 
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let _n = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let bill: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let b = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    bon_appetit(&bill, k, b);
}
