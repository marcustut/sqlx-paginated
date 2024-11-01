use crate::paginated_query_as::internal::{
    QueryDateRangeParams, QueryPaginationParams, QuerySearchParams, QuerySortParams,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaginatedResponse<T> {
    pub records: Vec<T>,
    #[serde(flatten)]
    pub pagination: QueryPaginationParams,
    pub total: Option<i64>,
    pub total_pages: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FlatQueryParams {
    #[serde(flatten)]
    pub pagination: Option<QueryPaginationParams>,
    #[serde(flatten)]
    pub sort: Option<QuerySortParams>,
    #[serde(flatten)]
    pub search: Option<QuerySearchParams>,
    #[serde(flatten)]
    pub date_range: Option<QueryDateRangeParams>,
    #[serde(flatten)]
    pub filters: Option<HashMap<String, Option<String>>>,
}

#[derive(Default, Clone)]
pub struct QueryParams<'q, T> {
    pub pagination: QueryPaginationParams,
    pub sort: QuerySortParams,
    pub search: QuerySearchParams,
    pub date_range: QueryDateRangeParams,
    pub filters: HashMap<String, Option<String>>,
    pub(crate) _phantom: PhantomData<&'q T>,
}

impl<'q, T> From<FlatQueryParams> for QueryParams<'q, T> {
    fn from(params: FlatQueryParams) -> Self {
        QueryParams {
            pagination: params.pagination.unwrap_or_default(),
            sort: params.sort.unwrap_or_default(),
            search: params.search.unwrap_or_default(),
            date_range: params.date_range.unwrap_or_default(),
            filters: params.filters.unwrap_or_default(),
            _phantom: PhantomData::<&'q T>,
        }
    }
}
