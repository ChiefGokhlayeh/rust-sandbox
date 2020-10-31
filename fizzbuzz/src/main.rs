use std::io;
use std::io::prelude::*;
use std::ops;

fn main() {
    loop {
        let number = prompt_number(1..101);
        fizz_buzz(number);
    }
}

fn fizz_buzz(number: i32) {
    match (number % 3 == 0, number % 5 == 0) {
        (true, true) => println!("FizzBuzz"),
        (true, false) => println!("Fizz"),
        (false, true) => println!("Buzz"),
        (false, false) => println!("{}", number),
    }
}

fn prompt_number(range: ops::Range<i32>) -> i32 {
    let mut buffer = String::new();

    loop {
        buffer.clear();

        print!(
            "Please enter a number in the range of {} to {}: ",
            range.start,
            range.end - 1
        );
        io::stdout().flush().ok().expect("Could not flush stdout");
        let parse_result = match io::stdin().read_line(&mut buffer) {
            Ok(_) => buffer.trim().parse::<i32>(),
            Err(error) => {
                println!("error: {}", error);
                continue;
            }
        };

        match parse_result {
            Ok(num) => {
                if !range.contains(&num) {
                    println!(
                        "error: number not in range {} to {}",
                        range.start,
                        range.end - 1
                    );
                    continue;
                }
                return num;
            }
            Err(error) => {
                println!("error: {}", error);
            }
        }
    }
}
