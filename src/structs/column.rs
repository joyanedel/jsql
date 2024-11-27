use std::io::{self, ErrorKind};

pub enum ColumnDataType {
    Varchar(u8),
}

pub struct Column {
    name: String,
    column_type: ColumnDataType,
}

impl TryFrom<&[u8]> for ColumnDataType {
    type Error = ErrorKind;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let _type = value.first().unwrap();
        let length = value.get(1).unwrap();

        match _type {
            0 => Ok(Self::Varchar(length.to_owned())),
            _ => unimplemented!()
        }
    }
}

impl TryFrom<&[u8]> for Column {
    type Error = ErrorKind;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let column_type = ColumnDataType::try_from(&value[..2]);
        if column_type.is_err() {
            return Err(self::ErrorKind::Other);
        }

        let column_type = column_type.unwrap();

        // from byte 3 to .. is the name
        let column_name_length = value.get(2).unwrap();
        let name = String::from_utf8_lossy(&value[3..(3 + (column_name_length.to_owned() as usize))]);

        Ok(Self {
            name: name.to_string(),
            column_type
        })
    }
}
