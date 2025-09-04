pub trait TableHandler {
    fn create_table(&self, table_name: &str, columns: Vec<&str>) -> Result<(), String> {
        unimplemented!("create_table() unimplemented")
    }
    fn insert_into_table(&self, table_name: &str, values: Vec<&str>) -> Result<(), String> {
        unimplemented!("insert_into_table() unimplemented")
    }
    fn select_from_table(
        &self,
        table_name: &str,
        columns: Vec<&str>,
    ) -> Result<Vec<Vec<String>>, String> {
        unimplemented!("select_from_table() unimplemented")
    }
    fn update_table(
        &self,
        table_name: &str,
        column: &str,
        value: &str,
        condition: &str,
    ) -> Result<(), String> {
        unimplemented!("update_table() unimplemented")
    }
    fn delete_from_table(&self, table_name: &str, condition: &str) -> Result<(), String> {
        unimplemented!("delete_from_table() unimplemented")
    }
}
