use std::time::Instant;

use rand::prelude::*;
use sqlx::mysql::MySqlPoolOptions;
// use sqlx::FromRow;

#[derive(Debug, sqlx::FromRow)]
struct User {
    name: String
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    let start = Instant::now();

    // databaseに接続する
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:Swimming3003@localhost/test_db").await?;

    let times = 5;
    let start_number = 1;
    let end_number = 55;

    let row_id_list = generate_random_number(times, start_number, end_number);

    let id_list = row_id_list.iter()
        .map(|id| id.to_string()) 
        .collect::<Vec<_>>()
        .join(",");

    println!("id_list : {}", id_list);

    let sql_query = format!(
        "SELECT name FROM user WHERE id IN ({});",
        id_list
    );
    println!("sql_query : {}", sql_query);

    let rows: Vec<User>= sqlx::query_as(&sql_query)
        .fetch_all(&pool)
        .await?;

    println!("セレクト結果：{:?}",rows);    

    let duration = start.elapsed();

    println!("実行時間：{:?}", duration);

    Ok(())
}

fn generate_random_number(times: u16, start_number: u16, end_number: u16) -> Vec<u16> {
    let mut rng = rand::rng();
    let mut ids:Vec<u16> = Vec::new();

    for _ in 0..=times {
        let random_number: u16 = rng.random_range(start_number..=end_number);
        ids.push(random_number);
        
        println!("生成されたランダムID：{}", random_number);
    }

    return ids;
}

