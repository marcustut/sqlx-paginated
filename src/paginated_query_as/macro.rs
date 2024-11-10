#[macro_export]
macro_rules! paginated_query_as {
    ($query:expr) => {{
        paginated_query_as($query)
    }};
    ($type:ty, $query:expr) => {{
        paginated_query_as::<$type>($query)
    }};
}
