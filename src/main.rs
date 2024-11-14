use std::net::{TcpListener, TcpStream};

fn main() {
    let server = TcpListener::bind(("localhost", 8798)).expect("Couldn't bind port");

    for stream in server.incoming() {
        let client = match stream {
            Ok(client) => {
                println!(
                    "Connection established: {}",
                    client.local_addr().unwrap().ip()
                );
                client
            }
            Err(client_error) => {
                eprintln!("Error accepting incoming request: {client_error}");
                continue;
            }
        };

        handle_connection(client);
    }
}

fn handle_connection(client: TcpStream) {
    println!("Boo");
}
