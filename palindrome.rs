fn is_palindrome(input: &str) -> bool {
    let input = input.to_lowercase();
    let chars: Vec<char> = input.chars().collect();
    let (mut start, mut end) = (0, chars.len() - 1);
    while start < end {
        if chars[start] != chars[end] {
            return false;
        }
        start += 1;
        end -= 1;
    }
    true
}

fn main() {
    let test_str = "Racecar";
    if is_palindrome(test_str) {
        println!("{} is a palindrome!", test_str);
    } else {
        println!("{} is not a palindrome.", test_str);
    }
}
