fn main() {
    let string = "meatball";
    let pig_string = pigify(string.to_string());
    println!("{pig_string}");
}

fn pigify(mut string: String) -> String {
    let chars = string.char_indices();
    for (index, letter) in chars {
        if index == 0 {
            if letter == 'a' || letter == 'e' || letter == 'i' || letter == 'o' || letter == 'u'{
                string.push_str("hay");
                return string;
            }
            else{
                let mut pig = string[index + 1..].to_string();
                pig.push(letter);
                pig.push_str("ay");
                return pig;
            }
        }
    }
    "Error".to_string()
}