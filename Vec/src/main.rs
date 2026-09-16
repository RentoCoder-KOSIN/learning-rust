fn main() {
    let mut ports: Vec<u16> = Vec::new();
    ports.push(80);
    ports.push(443);
    ports.push(22);
    ports.push(8080);

    // let ports = vec![80, 443, 22, 8080];

    let open_web_ports: Vec<&u16> = ports.iter().filter(|&&p| p == 80 || p == 443).collect();

    println!("{:?}", open_web_ports);

    let total: u16 = ports.iter().sum();
    println!("{sum}");

    let first = ports[0];
    println!("{first}");

    for port in &ports {
        // borrow each element (ports still usable after)
        println!("{port}");
    }

    println!("{ports:?}");

    for port in ports {
        // takes ownership -- ports is moved, unusable after
        println!("{port}");
    }

    // println!("[{ports}:?]"); error
}
