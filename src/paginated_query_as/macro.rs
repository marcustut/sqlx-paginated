#[macro_export]
macro_rules! paginated_query_as {
    ($query:expr) => {{
        PaginatedQueryBuilder::new(sqlx::query_as($query))
    }};
    ($type:ty, $query:expr) => {{
        PaginatedQueryBuilder::new(sqlx::query_as::<_, $type>($query))
    }};
}
