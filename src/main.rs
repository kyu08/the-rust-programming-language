fn main() {
    let words = "white snake";
    let first_word = get_first_word(words);
    println!("{}", first_word);
}

fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    s
}
