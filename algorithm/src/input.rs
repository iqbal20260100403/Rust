use std::io::{Write, stdin, stdout};

pub fn numbers() -> Vec<i32> {
    let mut nums: Vec<i32> = Vec::new();
    let mut InputBuffer: String = String::new();

    loop {
        println!("Numbers: {nums:?}");
        println!("To exit, write anything but number");
        print!("Enter an Integer number: ");
        stdout().flush().unwrap();

        stdin().read_line(&mut InputBuffer).unwrap();

        let num: i32 = match InputBuffer.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number, Exiting");
                break;
            }
        };

        nums.push(num);

        print!("\x1B[2J");
        InputBuffer.clear();
    }

    return nums;
}
