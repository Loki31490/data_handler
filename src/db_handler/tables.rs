#[allow(unused_imports)]
use Tables::*;


#[allow(dead_code)]
pub enum Tables {
    TestTable,
    UserTable,
    CollaboratorTable,
    TransactionTable,
    StockTable,
    SupplierTable,
    BrandTable,
    CategoryTable,
}

// Associations enum/values
impl Tables {
    // Return the name of the choosen table as a str
    pub fn table(table: Tables) -> &'static str {
        match table {
            Tables::TestTable => "test_table",
            Tables::UserTable => "user_table",
            Tables::CollaboratorTable => "collaborator_table",
            Tables::TransactionTable => "transaction_table",
            Tables::StockTable => "stock_table",
            Tables::SupplierTable => "supplier_table",
            Tables::BrandTable => "brand_table",
            Tables::CategoryTable => "category_table",
        }
    }
}

impl std::fmt::Display for Tables {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self)?;
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn tables() {
    assert_eq!(Tables::table(TestTable), "test_table");
    assert_eq!(Tables::table(UserTable), "user_table");
    assert_eq!(Tables::table(CollaboratorTable), "collaborator_table");
    assert_eq!(Tables::table(TransactionTable), "transaction_table");
    assert_eq!(Tables::table(StockTable), "stock_table");
    assert_eq!(Tables::table(SupplierTable), "supplier_table");
    assert_eq!(Tables::table(BrandTable), "brand_table");
    assert_eq!(Tables::table(CategoryTable), "category_table");
}
