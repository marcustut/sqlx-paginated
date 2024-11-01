use crate::paginated_query_as::internal::QueryDialect;

pub struct SqliteDialect;

impl QueryDialect for SqliteDialect {
    fn quote_identifier(&self, ident: &str) -> String {
        format!("\"{}\"", ident.replace('"', "\"\""))
    }

    fn placeholder(&self, _position: usize) -> String {
        "?".to_string()
    }

    fn type_cast(&self, _value: &str) -> String {
        String::new()
    }
}
