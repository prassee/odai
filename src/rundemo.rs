use crate::stream;

/*
after a map operation
by explicitly enforce it with generic params of the resulting stream.
- how to read the generic type params and create table
maintain app level schema registry :-
- i.e key - name of the table value - json schema of the table
- makes code verbose
*/
pub fn insert_select_demo() {
    let schema = stream::Schema::new(
        "txns_table".to_string(),
        vec![
            stream::Column {
                name: "id".to_string(),
                data_type: "INTEGER".to_string(),
            },
            stream::Column {
                name: "value".to_string(),
                data_type: "INTEGER".to_string(),
            },
        ],
    );
    let stream = stream::StreamTxfrm::new(&schema);
    stream.add(schema.name, 100);
    // stream.add(100);
}
