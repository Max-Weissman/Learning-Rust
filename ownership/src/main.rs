fn main() {
    println!("{}",find_string(" No"))
}

fn find_string(string: &str) -> &str {
    for (indexino, letterino) in string.chars().enumerate() {
        if letterino == ' ' {
            return &string[0..indexino];
        }
    }
    return string;
}