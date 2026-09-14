use std::io;

fn main() {
    let month = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "Octber",
        "November",
        "December",
    ];

    for m in month {
        println!("{m}");
    }

    let a = [1, 2, 3, 4, 5];

    loop {
        println!("Please enter an array index max{}.", a.len() - 1);

        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };
        let element = a[index];

        println!("The value of the element at index {index} is: {element}");

        break;
    }
}
