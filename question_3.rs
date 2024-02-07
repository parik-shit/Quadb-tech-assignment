fn shortest_word(string: &str) -> Option<&str> {
    string.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
    let input_string = "You will remember";
    if let Some(shortest) = shortest_word(input_string) {
        println!("Shortest word: {}", shortest);
    } else {
        println!("No words in the input buffer.");
    }
}