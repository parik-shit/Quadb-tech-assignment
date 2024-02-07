fn reverse_string(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

fn main() {
    let original_str = "hello";
    
    let reversed_str1 = reverse_string(original_str);
    println!("Original string: {}", original_str);
    println!("Reversed string: {}", reversed_str1);
}
