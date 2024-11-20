use std::{io::Write, net::TcpStream};

fn main() {
    let mut connection =
        TcpStream::connect(("localhost", 8798)).expect("Couldn't establish connection with server");

    let _ = connection.write("CREATE CREATE TABLE hola".as_bytes());

    connection
        .shutdown(std::net::Shutdown::Both)
        .expect("Couldn't close the connection");
}
