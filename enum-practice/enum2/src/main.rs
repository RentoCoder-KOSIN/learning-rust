enum PortStatus {
    Open(u16),
    Closed,
    Timeout(u64),
}

fn describe(status: PortStatus) -> String {
    match status {
        PortStatus::Open(port) => format!("Port{port} is opened"),
        PortStatus::Closed => String::from("closed"),
        PortStatus::Timeout(secs) => format!("Timeout {secs}s"),
    }
}

fn main() {
    let status1 = PortStatus::Open(80);
    let status2 = PortStatus::Closed;
    let status3 = PortStatus::Timeout(443);
    println!("{}", describe(status1));
    println!("{}", describe(status2));
    println!("{}", describe(status3));
}
