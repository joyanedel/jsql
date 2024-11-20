use bytes::BytesMut;
use sqlparser::{ast::Statement, dialect::GenericDialect, parser::Parser};
use std::io;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> io::Result<()> {
    let server = TcpListener::bind(("localhost", 8798))
        .await
        .expect("Couldn't bind port");
    loop {
        let (socket, _) = server.accept().await.unwrap();
        handle_connection(socket);
    }
}

fn handle_connection(client: TcpStream) {
    let raw_frame = read_buffer(&client);
    println!("My buffer: {}", raw_frame);

    parse_frame(&raw_frame);
}

fn read_buffer(client: &TcpStream) -> String {
    let mut buffer = BytesMut::with_capacity(1024);
    loop {
        match client.try_read_buf(&mut buffer) {
            Ok(0) => break,
            Ok(_) => continue,
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
            Err(_) => panic!("this should not happen"),
        }
    }

    String::from_utf8_lossy(buffer.as_ref()).to_string()
}

fn parse_frame(raw_frame: &str) {
    let dialect = GenericDialect {};
    let ast = Parser::parse_sql(&dialect, raw_frame);

    if ast.is_err() {
        return;
    }

    let ast = ast.unwrap();
    let query = ast.first().unwrap();

    if let Statement::CreateTable(ct_query) = query {
        println!(
            "\
            Create Table
            query Table name: {:?},
            Columns: {:?}\
        ",
            ct_query.name, ct_query.columns
        );
    }
}
