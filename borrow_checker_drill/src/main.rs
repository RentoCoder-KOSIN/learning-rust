fn count_length(s: &String) -> usize {
    s.len()
}

fn shout(message: &mut String) {
    message.push_str("HiHIHI");
}

fn main() {
    let mut message = String::from("I'm RentoCoder-KOSIN");
    let len = count_length(&message);
    println!("{len}");
    shout(&mut message);
    println!("{message}");
}
