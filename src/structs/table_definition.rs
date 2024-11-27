use bytes::Buf;

use super::column::Column;

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
        }
        
        Ok(Self { columns })
    }
}
