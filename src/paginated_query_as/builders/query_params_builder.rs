use crate::paginated_query_as::internal::{
    get_struct_field_names, QueryDateRangeParams, QueryPaginationParams, QuerySearchParams,
    QuerySortParams, SortDirection, DEFAULT_DATE_RANGE_COLUMN_NAME, DEFAULT_MAX_PAGE_SIZE,
    DEFAULT_MIN_PAGE_SIZE, DEFAULT_PAGE,
};
use crate::QueryParams;
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::HashMap;

pub struct QueryParamsBuilder<'q, T> {
    query: QueryParams<'q, T>,
}

impl<'q, T: Default + Serialize> Default for QueryParamsBuilder<'q, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'q, T: Default + Serialize> QueryParamsBuilder<'q, T> {
    pub fn new() -> Self {
        Self {
            query: QueryParams::default(),
        }
    }

    pub fn with_pagination(mut self, page: i64, page_size: i64) -> Self {
        self.query.pagination = QueryPaginationParams {
            page: page.max(DEFAULT_PAGE),
            page_size: page_size.clamp(DEFAULT_MIN_PAGE_SIZE, DEFAULT_MAX_PAGE_SIZE),
        };
        self
    }

    pub fn with_sort(
        mut self,
        sort_column: impl Into<String>,
        sort_direction: SortDirection,
    ) -> Self {
        self.query.sort = QuerySortParams {
            sort_column: sort_column.into(),
            sort_direction,
        };
        self
    }

    pub fn with_search(
        mut self,
        search: impl Into<String>,
        search_columns: Vec<impl Into<String>>,
    ) -> Self {
        self.query.search = QuerySearchParams {
            search: Some(search.into()),
            search_columns: Some(search_columns.into_iter().map(Into::into).collect()),
        };
        self
    }

    pub fn with_date_range(
        mut self,
        date_after: Option<DateTime<Utc>>,
        date_before: Option<DateTime<Utc>>,
        column_name: Option<impl Into<String>>,
    ) -> Self {
        self.query.date_range = QueryDateRangeParams {
            date_after,
            date_before,
            date_column: column_name.map_or_else(
                || Some(DEFAULT_DATE_RANGE_COLUMN_NAME.to_string()),
                |column_name| Some(column_name.into()),
            ),
        };
        self
    }

    pub fn with_filter(mut self, key: impl Into<String>, value: Option<impl Into<String>>) -> Self {
        let key = key.into();
        let valid_fields = get_struct_field_names::<T>();

        if valid_fields.contains(&key) {
            self.query.filters.insert(key, value.map(Into::into));
        } else {
            #[cfg(feature = "tracing")]
            tracing::warn!(column = %key, "Skipping invalid filter column");
        }
        self
    }

    pub fn with_filters(
        mut self,
        filters: HashMap<impl Into<String>, Option<impl Into<String>>>,
    ) -> Self {
        let valid_fields = get_struct_field_names::<T>();

        self.query
            .filters
            .extend(filters.into_iter().filter_map(|(key, value)| {
                let key = key.into();
                if valid_fields.contains(&key) {
                    Some((key, value.map(Into::into)))
                } else {
                    #[cfg(feature = "tracing")]
                    tracing::warn!(column = %key, "Skipping invalid filter column");
                    None
                }
            }));

        self
    }

    pub fn build(self) -> QueryParams<'q, T> {
        self.query
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paginated_query_as::internal::SortDirection;
    use chrono::{DateTime, Utc};
    use std::collections::HashMap;

    #[derive(Debug, Default, Serialize)]
    struct TestModel {
        name: String,
        title: String,
        description: String,
        status: String,
        category: String,
        updated_at: DateTime<Utc>,
        created_at: DateTime<Utc>,
    }

    #[test]
    fn test_empty_params() {
        let params = QueryParamsBuilder::<TestModel>::new().build();

        assert_eq!(params.pagination.page, 1);
        assert_eq!(params.pagination.page_size, 10);
        assert_eq!(params.sort.sort_column, "created_at");
        assert!(matches!(
            params.sort.sort_direction,
            SortDirection::Descending
        ));
    }

    #[test]
    fn test_partial_params() {
        let params = QueryParamsBuilder::<TestModel>::new()
            .with_pagination(2, 10)
            .with_search("test".to_string(), vec!["name".to_string()])
            .build();

        assert_eq!(params.pagination.page, 2);
        assert_eq!(params.search.search, Some("test".to_string()));
        assert_eq!(params.pagination.page_size, 10);
        assert_eq!(params.sort.sort_column, "created_at");
        assert!(matches!(
            params.sort.sort_direction,
            SortDirection::Descending
        ));
    }

    #[test]
    fn test_invalid_params() {
        // For builder pattern, invalid params would be handled at compile time
        // But we can test the defaults
        let params = QueryParamsBuilder::<TestModel>::new()
            .with_pagination(0, 0) // Should be clamped to minimum values
            .build();

        assert_eq!(params.pagination.page, 1);
        assert_eq!(params.pagination.page_size, 10);
    }

    #[test]
    fn test_filters() {
        let mut filters = HashMap::new();
        filters.insert("status".to_string(), Some("active".to_string()));
        filters.insert("category".to_string(), Some("test".to_string()));

        let params = QueryParamsBuilder::<TestModel>::new()
            .with_filters(filters)
            .build();

        assert!(params.filters.contains_key("status"));
        assert_eq!(
            params.filters.get("status").unwrap(),
            &Some("active".to_string())
        );
        assert!(params.filters.contains_key("category"));
        assert_eq!(
            params.filters.get("category").unwrap(),
            &Some("test".to_string())
        );
    }

    #[test]
    fn test_search_with_columns() {
        let params = QueryParamsBuilder::<TestModel>::new()
            .with_search(
                "test".to_string(),
                vec!["title".to_string(), "description".to_string()],
            )
            .build();

        assert_eq!(params.search.search, Some("test".to_string()));
        assert_eq!(
            params.search.search_columns,
            Some(vec!["title".to_string(), "description".to_string()])
        );
    }

    #[test]
    fn test_full_params() {
        let params = QueryParamsBuilder::<TestModel>::new()
            .with_pagination(2, 20)
            .with_sort("updated_at".to_string(), SortDirection::Ascending)
            .with_search(
                "test".to_string(),
                vec!["title".to_string(), "description".to_string()],
            )
            .with_date_range(Some(Utc::now()), None, None::<String>)
            .build();

        assert_eq!(params.pagination.page, 2);
        assert_eq!(params.pagination.page_size, 20);
        assert_eq!(params.sort.sort_column, "updated_at");
        assert!(matches!(
            params.sort.sort_direction,
            SortDirection::Ascending
        ));
        assert_eq!(params.search.search, Some("test".to_string()));
        assert_eq!(
            params.search.search_columns,
            Some(vec!["title".to_string(), "description".to_string()])
        );
        assert!(params.date_range.date_after.is_some());
        assert!(params.date_range.date_before.is_none());
    }

    #[test]
    fn test_filter_chain() {
        let params = QueryParamsBuilder::<TestModel>::new()
            .with_filter("status", Some("active"))
            .with_filter("category", Some("test"))
            .build();

        assert_eq!(
            params.filters.get("status").unwrap(),
            &Some("active".to_string())
        );
        assert_eq!(
            params.filters.get("category").unwrap(),
            &Some("test".to_string())
        );
    }

    #[test]
    fn test_mixed_pagination() {
        let params = QueryParamsBuilder::<TestModel>::new()
            .with_pagination(2, 10)
            .with_search("test".to_string(), vec!["title".to_string()])
            .with_filter("status", Some("active"))
            .build();

        assert_eq!(params.pagination.page, 2);
        assert_eq!(params.pagination.page_size, 10);
        assert_eq!(params.search.search, Some("test".to_string()));
        assert_eq!(
            params.filters.get("status").unwrap(),
            &Some("active".to_string())
        );
    }
}
