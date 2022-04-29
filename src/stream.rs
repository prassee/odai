use gluesql::prelude::{Glue, SledStorage};

pub(crate) struct StreamTxfrm {
    prefix: String,
    glue: Glue<gluesql::sled_storage::sled::IVec, SledStorage>,
}

impl StreamTxfrm {
    // @TODO a schema struct instance to be passed which has vec of columns along with type
    // the table will be created based on the schema instance

    pub fn new(prefix: String) -> Self {
        let mut glue = Glue::new(SledStorage::new("data/doc-db").unwrap());
        let del_name = format!("DROP TABLE IF EXISTS {prefix}");
        let cre_name = format!("CREATE TABLE {prefix} (id INTEGER)");
        for sql in vec![del_name, cre_name] {
            let output = glue.execute(&sql).unwrap();
            println!("{:?} for {:?}", output, prefix);
        }
        Self { prefix, glue }
    }

    pub fn add(mut self, data: i32) {
        let table_name = self.prefix;
        let insert = format!("INSERT INTO {table_name} VALUES ({data})");
        let output = self.glue.execute(&insert).unwrap();
        println!("{:?} for {:?}", output, table_name);
    }

    // @TODO a method required to convert the kafka consumer transform fn's  (map, flatmap etc)
    // to write to table
    // if multiple transform fu's are involved, schema of the last transform to match the schema
    // definition created above.
}
