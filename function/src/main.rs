fn five() -> i32 {
    5
}

fn main() {
    println!("Hello, world!");
    let x = five();
    let y = plus_one(6);

    println!("The value of y: {y}");
    another_function(x, 'H');
}

fn another_function(x: i32, unit_label: char) {
    println!("The measurement is: {x}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
