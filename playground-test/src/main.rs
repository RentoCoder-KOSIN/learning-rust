fn main() {
    let s = String::from("miurarento");
    let size = first_word(&s);

    println!("size of {s} is {size}");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
