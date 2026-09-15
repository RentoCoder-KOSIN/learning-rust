fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let s = String::from("hello world");

    // let hello = &s[0..5];
    let hello = &s[..5];
    // let world = &s[6..11];
    let world = &s[6..];

    println!("{hello}, {world}");

    let s2 = first_word(&s);
    let s3 = &s[s2.len() + 1..];
    println!("{s2} {s3}");
}
