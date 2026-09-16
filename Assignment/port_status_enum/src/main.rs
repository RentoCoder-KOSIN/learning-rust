enum PortStatus {
    Open(u16),
    Closed,
    Filtered,
}

// --- println! vs format! ---
// println!  : builds a string AND prints it immediately (returns ())
// format!   : builds a string and RETURNS it as a String (does not print)
// -> Use format! inside functions that need to return a String (like describe()).

// --- format! vs String::from ---
// format!("...{var}...")  : use when the string needs to embed a variable
//                            (the content changes at runtime)
// String::from("...")     : use for a fixed string with no interpolation
// x.to_string()           : roughly equivalent to format!("{}", x)
//                            when x implements Display

fn describe(status: &PortStatus) -> String {
    match status {
        PortStatus::Open(port) => format!("[port: {port}] is open"),
        PortStatus::Closed => String::from("closed"),
        PortStatus::Filtered => String::from("filtered"),
    }
}

fn main() {
    let status1 = PortStatus::Open(443);
    let status2 = PortStatus::Closed;
    let status3 = PortStatus::Filtered;

    println!("{}", describe(&status1));
    println!("{}", describe(&status2));
    println!("{}", describe(&status3));

    // if let
    if let PortStatus::Open(port) = &status1 {
        println!("if let.... [port: {port}] is open");
    }
}
