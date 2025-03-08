fn main() {
    let mut num = 1;
    println!("Odd numbers from 1 to 20:");
    while num <= 20 {
        if num % 2 != 0 {
            println!("{}", num);
        }
        num += 1;
    }
}

