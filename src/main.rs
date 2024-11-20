use bytes::BytesMut;
use std::io;
use tokenizer::TokenKind;
use tokio::net::{TcpListener, TcpStream};

pub mod tokenizer;

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
    let mut buffer = BytesMut::with_capacity(1024);
    client.try_read_buf(&mut buffer).unwrap();
    let raw_frame = String::from_utf8_lossy(buffer.as_ref());
    println!("My buffer: {}", raw_frame);

    parse_frame(&raw_frame);
}

fn parse_frame(raw_frame: &str) {
    let tokens: Vec<_> = raw_frame.split(" ").collect();
    let token_kinds: Vec<_> = tokens.into_iter().map(TokenKind::try_from).collect();
    println!("Tokens: {:?}", token_kinds);
}
