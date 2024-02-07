fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    
    let mut prefix = strings[0].clone();
    
    for s in &strings[1..] {
        while !s.starts_with(&prefix) {
            prefix.pop();
        }
        if prefix.is_empty() {
            break;
        }
    }
    
    prefix
}

fn main() {
    let strings = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    
    let common_prefix = longest_common_prefix(&strings);
    
    println!("Longest Common Prefix: {}", common_prefix);
}
