enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn show_message(message: &Message) {
    match message {
        Message::Quit => {
            println!("Quit");
        }

        Message::Move { x, y } => {
            println!("x = {x}, y = {y}");
        }

        Message::Write(text) => {
            println!("{text}");
        }

        Message::ChangeColor(r, g, b) => {
            println!("{r}, {g}, {b}");
        }
    }
}

fn main() {
    let mut message = Message::Quit;
    show_message(&message);

    message = Message::Move { x: 30, y: 60 };
    show_message(&message);

    message = Message::Write(String::from("hello"));
    show_message(&message);

    message = Message::ChangeColor(255, 0, 0);
    show_message(&message);
}
