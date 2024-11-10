use crate::paginated_query_as::internal::{
    default_date_range_column, default_page, default_page_size, default_search_columns,
    default_sort_column, default_sort_direction, deserialize_page, deserialize_page_size,
    deserialize_search, deserialize_search_columns,
};

use crate::QuerySortDirection;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct QueryPaginationParams {
    #[serde(deserialize_with = "deserialize_page", default = "default_page")]
    pub page: i64,
    #[serde(
        deserialize_with = "deserialize_page_size",
        default = "default_page_size"
    )]
    pub page_size: i64,
}

impl Default for QueryPaginationParams {
    fn default() -> Self {
        Self {
            page: default_page(),
            page_size: default_page_size(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct QuerySortParams {
    #[serde(default = "default_sort_column")]
    pub sort_column: String,
    #[serde(default = "default_sort_direction")]
    pub sort_direction: QuerySortDirection,
}

impl Default for QuerySortParams {
    fn default() -> Self {
        Self {
            sort_column: default_sort_column(),
            sort_direction: default_sort_direction(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub struct QuerySearchParams {
    #[serde(deserialize_with = "deserialize_search")]
    pub search: Option<String>,
    #[serde(
        deserialize_with = "deserialize_search_columns",
        default = "default_search_columns"
    )]
    pub search_columns: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub struct QueryDateRangeParams {
    #[serde(default = "default_date_range_column")]
    pub date_column: Option<String>,
    pub date_after: Option<DateTime<Utc>>,
    pub date_before: Option<DateTime<Utc>>,
}
