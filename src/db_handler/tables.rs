pub enum Tables {
    TestTable,
}

impl Tables {
    pub fn table(table: Tables) -> &'static str {
        match table {
            Tables::TestTable => "test_table",
        }
    }
}

impl std::fmt::Display for Tables {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self)?;
        Ok(())
    }
}
