use std::{io::Write, net::TcpStream};

fn main() {
    let mut connection =
        TcpStream::connect(("localhost", 8798)).expect("Couldn't establish connection with server");

    let result = connection.write("CREATE TABLE my_table ( test VARCHAR(10), test_2 VARCHAR(54) )".as_bytes());
    if result.is_err() {
        eprintln!("Error at writing: {}", result.unwrap_err());
    } else {
        println!("Bytes written: {}", result.unwrap());
    }

    connection
        .shutdown(std::net::Shutdown::Both)
        .expect("Couldn't close the connection");
}
