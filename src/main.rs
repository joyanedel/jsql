use bytes::{BufMut, BytesMut};
use sqlparser::{
    ast::{CreateTable, DataType, Statement},
    dialect::GenericDialect,
    parser::Parser,
};
use std::{io::{self, Read}, path::Path};
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
        let _ = create_table(ct_query);
    }
}

fn create_table(statement: &CreateTable) -> io::Result<()> {
    let table_name = statement.name.to_string();

    // columns are store as
    // <Column type byte><Column data type length><Column name length><Column name>
    // Example for column 'test' varchar(255): 02554test -> 0x00ff0474657374 = 0x(00)(ff)(04)(76 65
    // 73 74)
    let columns = statement
        .clone()
        .columns
        .into_iter()
        .filter_map(|def| match def.data_type {
            DataType::Varchar(value_max_length) => Some((
                def.name,
                0_u8,
                match value_max_length {
                    Some(sqlparser::ast::CharacterLength::IntegerLength {
                        length: c_length,
                        ..
                    }) => c_length as u8,
                    _ => 255_u8,
                },
            )),
            _ => None,
        })
        .collect::<Vec<_>>();
    let table_exists = Path::new(&format!("{}.djsql", table_name)).exists();

    if table_exists {
        return Ok(());
    }

    // all columns are store as
    // <Columns length><Column definition> where column definition is defined above
    // Example 2 columns: 'test' varchar(10) and 'test_2' varchar(54) might be stored as follow
    // 2 0 10 4 test 0 54 6 test_2 = 0x(02) (000a04 76657374) (003606 766573745f32)
    let mut create_table_buffer = BytesMut::with_capacity(1024);
    create_table_buffer.put_u8(columns.len() as u8);
    for (column_name, column_data_type, column_value_length) in columns {
        let column_name = column_name.value.as_bytes();
        // put column value type
        create_table_buffer.put_u8(column_data_type);
        // put column value length
        create_table_buffer.put_u8(column_value_length);
        // put column name length
        create_table_buffer.put_u8(column_name.len() as u8);
        // put column name
        create_table_buffer.put(column_name);
    }

    println!("Create table hex: {:#?}", create_table_buffer.bytes());

    Ok(())
}
