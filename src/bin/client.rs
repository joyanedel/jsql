use std::{io::Write, net::TcpStream};

fn main() {
    let mut connection =
        TcpStream::connect(("localhost", 8798)).expect("Couldn't establish connection with server");

    let result = connection.write("CREATE TABLE my_table ( id VARCHAR(255) )".as_bytes());
    if result.is_err() {
        eprintln!("Error at writing: {}", result.unwrap_err());
    } else {
        println!("Bytes written: {}", result.unwrap());
    }

    connection
        .shutdown(std::net::Shutdown::Both)
        .expect("Couldn't close the connection");
}
