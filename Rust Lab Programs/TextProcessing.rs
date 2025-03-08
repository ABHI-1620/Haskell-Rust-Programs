use std::io;

fn extract_word(input: &str, start: usize, end: usize) -> &str {
    &input[start..end]
}

fn main() {
    let mut sentence = String::new();
    println!("Enter a sentence: ");
    io::stdin().read_line(&mut sentence).expect("Failed to read line");
    let sentence = sentence.trim();
    println!("You entered: {}", sentence);
    let start = sentence.find("Rust").unwrap_or(0);
    let end = start + "Rust".len();
    let word = extract_word(sentence, start, end);
    println!("Extracted word: {}", word);
    let modified_sentence = sentence.replace("Rust", "Rust is awesome");
    println!("Modified sentence: {}", modified_sentence);
}