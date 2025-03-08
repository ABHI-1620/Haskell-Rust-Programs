fn main() {
    println!("Even numbers from 1 to 20:");
    for i in 1..=20 {
        if i % 2 == 0 {
            println!("{}", i);
        }
    }
}

