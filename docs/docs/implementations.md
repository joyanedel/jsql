# Implementations

This page aims to explain how some components are implemented, the codification used in order to obtain valuable results and so on.

## Create table

When creating tables, the system will create a `.djsql`, which stands for Definition JSQL, containing table metadata.

The metadata is stored in binary codification with the sole purpose of optimize store utilization. It contains the number of columns of the table and for each column:

* Column used bytes (the bytes to store the following metadata excluding these bytes) `4 bytes`
* Column data type `1 byte`
* Column data type length (in bytes) `1 byte`
* Column name length `1 byte`
* Column name (ASCII) `<Column name length> bytes`

### Example
If we create a simple table with the query
```sql
CREATE TABLE my_table ( col_1 VARCHAR(10), col_2 VARCHAR(54) );
```
The file `my_table.djsql` will be created with the following information (before encoded):

```
2
8 0 10 5 col_1
8 0 54 5 col_2
```

Which will be stored as follow (visualized in hex for better visualization)

```hex
0x0200000008000a05636f6c5f3100000008003605636f6c5f32
```
