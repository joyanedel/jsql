use bytes::Buf;

use super::column::Column;

#[derive(PartialEq)]
pub struct TableDefinition {
    columns: Vec<Column>,
}

impl TryFrom<&[u8]> for TableDefinition {
    type Error = ();
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let buf = value.to_owned();
        let mut buf: &[u8] = buf.as_ref();

        let column_length = buf.get_u8();
        let mut columns = Vec::with_capacity(column_length.to_owned() as usize);

        loop {
            let column_used_bytes = buf.get_u32();
            let column_def = Column::try_from(&buf[..column_used_bytes as usize]);
            if column_def.is_err() {
                return Err(());
            }

            columns.push(column_def.unwrap());

            if columns.len() == column_length.into() {
                break;
            }

            buf.advance(column_used_bytes as usize);
        }

        Ok(Self { columns })
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::column::{Column, ColumnDataType};

    use super::TableDefinition;

    #[test]
    fn table_definition_from_bytes_correctly_parsed() {
        let bytes = [
            2, 0, 0, 0, 5, 0, 10, 2, 97, 98, 0, 0, 0, 5, 0, 20, 2, 97, 99,
        ];
        let result = TableDefinition::try_from(bytes.as_ref());

        assert!(result.is_ok_and(|r| {
            r == TableDefinition {
                columns: vec![
                    Column {
                        name: "ab".to_string(),
                        column_type: ColumnDataType::Varchar(10),
                    },
                    Column {
                        name: "ac".to_string(),
                        column_type: ColumnDataType::Varchar(20),
                    },
                ],
            }
        }))
    }
}
