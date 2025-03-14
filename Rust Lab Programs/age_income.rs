use std::io;

fn main() {
    println!("Enter your age:");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read line");
    let age: i32 = age.trim().parse().expect("Please type a number!");

    println!("Enter your income (in ₹):");
    let mut income = String::new();
    io::stdin().read_line(&mut income).expect("Failed to read line");
    let income: i32 = income.trim().parse().expect("Please type a number!");

    if age < 21 {
        println!("You are ineligible for a loan due to age.");
    } else if age >= 21 && age <= 60 {
        if income > 50000 {
            println!("You are eligible for a loan.");
        } else {
            println!("You are ineligible due to low income.");
        }
    } else {
        println!("You need a guarantor for the loan.");
    }
}