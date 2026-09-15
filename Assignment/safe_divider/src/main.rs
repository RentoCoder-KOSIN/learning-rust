fn safe_divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}

fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse::<i32>()
}

//OptionのときはSome, Noneで
//ResultのときはOk, Errってこと

fn main() {
    match safe_divide(10, 2) {
        Some(result) => println!("divided!: {result}"),
        None => println!("not divided"),
    };

    match safe_divide(5, 0) {
        Some(result) => println!("divided!: {result}"),
        None => println!("not divided"),
    }

    // println!("{:?}", parse_number("123"));
    // println!("{:?}", parse_number("abc"));

    match parse_number("123") {
        Ok(n) => println!("OK: {n}"),
        Err(e) => println!("Err: {e}"),
    }

    match parse_number("abc") {
        Ok(n) => println!("OK: {n}"),
        Err(e) => println!("Err: {e}"),
    }
}
