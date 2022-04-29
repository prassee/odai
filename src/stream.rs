use gluesql::prelude::{Glue, SledStorage};

pub(crate) struct StreamTxfrm {
    glue: Glue<gluesql::sled_storage::sled::IVec, SledStorage>,
}

pub(crate) struct Column {
    pub name: String,
    pub data_type: String,
}

pub(crate) struct Schema {
    pub name: String,
    pub columns: Vec<Column>,
}

impl StreamTxfrm {
    // @TODO a schema struct instance to be passed which has
    // vec of columns along with type
    // the table will be created based on the schema instance

    pub fn new(schema: &Schema) -> Self {
        let mut glue = Glue::new(SledStorage::new("data/doc-db").unwrap());
        for sql in schema.register_table() {
            let output = glue.execute(&sql).unwrap();
            println!("{:?} for {:?}", output, schema.name);
        }
        Self { glue }
    }

    pub fn add(mut self, table_name: String, data: i32) {
        let insert = format!("INSERT INTO {table_name} VALUES ({data},{data})");
        let output = self.glue.execute(&insert).unwrap();
        println!("{:?} for {:?}", output, table_name);
    }

    // @TODO a method required to convert the kafka consumer transform fn's  (map, flatmap etc)
    // to write to table
    // if multiple transform fu's are involved, schema of the last transform to match the schema
    // definition created above.
}

impl Schema {
    pub fn new(name: String, columns: Vec<Column>) -> Self {
        Self { name, columns }
    }

    pub fn register_table(&self) -> Vec<String> {
        let table_name = &self.name;
        let drop_table_ddl = format!("drop table if exists {table_name}");
        let mut create_table_ddl = format!("create table {table_name} (").to_string();
        for col in &self.columns {
            let name = &col.name;
            let data_type = &col.data_type;
            create_table_ddl.push_str(&format!("{name} {data_type},"));
        }
        create_table_ddl.push_str(")");
        return vec![drop_table_ddl, create_table_ddl];
    }
}
