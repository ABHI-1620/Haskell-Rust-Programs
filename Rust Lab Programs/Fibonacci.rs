use std::io;

fn main() {
    println!("Enter the number of Fibonacci numbers to generate:");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: usize = n.trim().parse().expect("Please type a number!");
    let mut fib_sequence = vec![0, 1];
    for _ in 2..n {
        let next_fib = fib_sequence[fib_sequence.len() - 1] + 
        fib_sequence[fib_sequence.len() - 2];
        fib_sequence.push(next_fib);
    }
    println!("Fibonacci sequence up to {} numbers:", n);
    for num in fib_sequence.iter().take(n) {
        print!("{} ", num);
    }
}