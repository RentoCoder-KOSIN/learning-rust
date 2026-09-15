fn print_length(s: &String) -> usize {
    s.len()
}

fn main() {
    let original = String::from("RentoCoder-KOSIN");
    let moved = original;

    println!("{moved}");

    let data = String::from("RentoCoder-KOSIN2");
    let data_copy = data.clone();

    println!("{data}, {data_copy}");

    let text = String::from("hello");
    let len = print_length(&text);
    println!("{text}, {len}");
}
