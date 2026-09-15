fn last_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut idx: usize = 0;
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            idx = i + 1;
        }
    }
    &s[idx..]
}

fn main() {
    let sentence = String::from("Rust is fun");
    let lw = last_word(&sentence);
    println!("{lw}");

    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[2..];
    println!("{:?}", slice);
}
