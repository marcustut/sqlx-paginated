use crate::paginated_query_as::examples::build_query_with_safe_defaults;
use crate::paginated_query_as::internal::{quote_identifier, SortDirection};
use crate::{FlatQueryParams, PaginatedResponse, QueryParams};
use serde::Serialize;
use sqlx::postgres::PgArguments;
use sqlx::{postgres::Postgres, query::QueryAs, Execute, FromRow, IntoArguments, Pool};

pub struct PaginatedQueryBuilder<'q, T, A>
where
    T: for<'r> FromRow<'r, <Postgres as sqlx::Database>::Row> + Send + Unpin,
{
    query: QueryAs<'q, Postgres, T, A>,
    params: QueryParams<'q, T>,
    totals_count_enabled: bool,
    build_query_fn: fn(&QueryParams<T>) -> (Vec<String>, PgArguments),
}

impl<'q, T, A> PaginatedQueryBuilder<'q, T, A>
where
    T: for<'r> FromRow<'r, <Postgres as sqlx::Database>::Row>
        + Send
        + Unpin
        + Serialize
        + Default
        + 'static,
    A: 'q + IntoArguments<'q, Postgres> + Send,
{
    pub fn new(query: QueryAs<'q, Postgres, T, A>) -> Self {
        Self {
            query,
            params: FlatQueryParams::default().into(),
            totals_count_enabled: true,
            build_query_fn: |params| build_query_with_safe_defaults::<T, Postgres>(params),
        }
    }

    pub fn with_query_builder(
        self,
        build_query_fn: fn(&QueryParams<T>) -> (Vec<String>, PgArguments),
    ) -> Self {
        Self {
            build_query_fn,
            ..self
        }
    }

    pub fn with_params(mut self, params: impl Into<QueryParams<'q, T>>) -> Self {
        self.params = params.into();
        self
    }

    pub fn disable_totals_count(mut self) -> Self {
        self.totals_count_enabled = false;
        self
    }

    pub async fn fetch_paginated(
        self,
        pool: &Pool<Postgres>,
    ) -> Result<PaginatedResponse<T>, sqlx::Error> {
        let base_sql = self.build_base_query();
        let (conditions, main_arguments) = (self.build_query_fn)(&self.params);

        let where_clause = self.build_where_clause(&conditions);

        let (total, total_pages) = if self.totals_count_enabled {
            let (_, count_arguments) = (self.build_query_fn)(&self.params);
            let count_sql = format!(
                "{} SELECT COUNT(*) FROM base_query{}",
                base_sql, where_clause
            );
            let count: i64 = sqlx::query_scalar_with(&count_sql, count_arguments)
                .fetch_one(pool)
                .await?;

            let pages = if count == 0 {
                0
            } else {
                (count + self.params.pagination.page_size - 1) / self.params.pagination.page_size
            };

            (Some(count), Some(pages))
        } else {
            (None, None)
        };

        // Execute main query
        let mut main_sql = format!("{} SELECT * FROM base_query{}", base_sql, where_clause);
        main_sql.push_str(&self.build_order_clause());

        let pagination = &self.params.pagination;
        main_sql.push_str(&format!(
            " LIMIT {} OFFSET {}",
            pagination.page_size,
            (pagination.page - 1) * pagination.page_size
        ));

        let records = sqlx::query_as_with::<Postgres, T, _>(&main_sql, main_arguments)
            .fetch_all(pool)
            .await?;

        Ok(PaginatedResponse {
            records,
            pagination: pagination.clone(),
            total,
            total_pages,
        })
    }

    fn build_base_query(&self) -> String {
        format!("WITH base_query AS ({})", self.query.sql())
    }

    fn build_where_clause(&self, conditions: &[String]) -> String {
        if conditions.is_empty() {
            String::new()
        } else {
            format!(" WHERE {}", conditions.join(" AND "))
        }
    }

    fn build_order_clause(&self) -> String {
        let order = match self.params.sort.sort_direction {
            SortDirection::Ascending => "ASC",
            SortDirection::Descending => "DESC",
        };
        format!(
            " ORDER BY {} {}",
            quote_identifier(&self.params.sort.sort_column),
            order
        )
    }
}
