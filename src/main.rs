use std::net::UdpSocket;

fn main() {
    let local_ipaddress = get_local_ipaddress().unwrap();
    println!("{}", local_ipaddress);
}

pub fn get_local_ipaddress() -> Option<String> {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return None,
    };

    match socket.connect("8.8.8.8:80") {
        Ok(()) => (),
        Err(_) => return None,
    };

    match socket.local_addr() {
        Ok(addr) => Some(addr.ip().to_string()),
        Err(_) => None,
    }
}
