use sqlx::MySqlPool;

use crate::domain::model::Test;

pub async fn execute_sql_query(pool: &MySqlPool, query: &str) -> Result<Vec<Test>, sqlx::Error> {
    let rows: Vec<Test> = sqlx::query_as(query)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}
