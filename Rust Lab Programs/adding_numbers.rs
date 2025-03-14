use std::io;

fn main() {
    let mut numbers = Vec::new();
    loop {
        println!("Enter a number (0 to stop):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };
        if num == 0 {
            break;
        }
        numbers.push(num);
    }
    println!("Even numbers in the list:");
    for num in numbers {
        if num % 2 == 0 {
            println!("{}", num);
        }
    }
}