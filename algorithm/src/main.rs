use std::io::{Write, stdin, stdout};

mod input;
mod max;

fn main() {
    println!("\x1B[2J");

    let mut numbers: Vec<i32> = Vec::new();

    loop {
        let mut input_buffer: String = String::new();

        println!("Available Option: ");
        println!("1. Update Numbers");
        println!("2. Search Numbers Maximum Value");
        print!("Your call? : ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input_buffer).unwrap();
        let call: i32 = input_buffer.trim().parse().unwrap();

        if call == 1 {
            numbers = input::numbers();
        } else if call == 2 {
            continue;
        }
    }
}
