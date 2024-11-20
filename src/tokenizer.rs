#[derive(Debug)]
pub enum DataType {
    Varchar(u32),
    Int,
}

#[derive(Debug)]
pub enum TokenKind {
    // Create table
    Create,
    Table,
    StringLiteral(String),
    Comma,
    OpenParenthesis,
    CloseParenthesis,
    DataType(DataType),
}

impl TryFrom<&str> for TokenKind {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let token = match value {
            "CREATE" => Self::Create,
            "TABLE" => Self::Table,
            _ => return Err(()),
        };
        Ok(token)
    }
}
