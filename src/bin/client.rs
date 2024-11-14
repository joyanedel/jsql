use std::net::TcpStream;

fn main() {
    let connection =
        TcpStream::connect(("localhost", 8798)).expect("Couldn't establish connection with server");

    connection
        .shutdown(std::net::Shutdown::Both)
        .expect("Couldn't close the connection");
}
