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
    let stream = stream::StreamTxfrm::new("txns".to_string());
    stream.add(100);
}
