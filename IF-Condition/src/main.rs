use std::io;

fn main() {
    let mut input = String::new();

    println!("Input your score!");
    io::stdin().read_line(&mut input).unwrap();

    let number: i32 = input.trim().parse().unwrap();

    if number == 100 {
        println!("Perfect");
    } else if number >= 80 {
        println!("Very good");
    } else if number >= 60 {
        println!("Good");
    } else if number >= 40 {
        println!("So so");
    } else {
        println!("Bad");
    }
}
