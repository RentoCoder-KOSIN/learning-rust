struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn st_print(user: User) {
    println!("{}", user.active);
    println!("{}", user.username);
    println!("{}", user.email);
    println!("{}", user.sign_in_count);
}

fn main() {
    let username = String::from("RentoCoder-KOSIN");
    let email = String::from("xxx@example.com");
    let user1 = build_user(email, username);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    st_print(user2);
}
