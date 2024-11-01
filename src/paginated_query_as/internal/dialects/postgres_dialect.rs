use crate::paginated_query_as::internal::{get_postgres_type_casting, QueryDialect};

pub struct PostgresDialect;

impl QueryDialect for PostgresDialect {
    fn quote_identifier(&self, ident: &str) -> String {
        format!("\"{}\"", ident.replace('"', "\"\""))
    }

    fn placeholder(&self, position: usize) -> String {
        format!("${}", position)
    }

    fn type_cast(&self, value: &str) -> String {
        get_postgres_type_casting(value).to_string()
    }
}
