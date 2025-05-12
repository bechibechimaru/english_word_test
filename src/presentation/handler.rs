use std::sync::Arc;
use std::fs::File;
use std::io::Read;

use axum::response::IntoResponse;
use axum::{
    extract::State,
    response::Json,
};
use hyper::StatusCode;
use sqlx::MySqlPool;
use base64::{engine::general_purpose, Engine};

use crate::domain::model::{Request, Response};
use crate::application::utils::{generate_random_number, generate_sql_query};
use crate::infra::db::execute_sql_query;
use crate::infra::pdf::gen_test_pdf;

pub async fn generate_test_handler(
    State(pool): State<Arc<MySqlPool>>, 
    Json(request): Json<Request>
) -> Result<impl IntoResponse, (StatusCode, String)> { 

    let book_name = request.english_word_book;
    let times = request.times;
    let start_number = request.start_number;
    let end_number = request.end_number;

    let row_id_list = match generate_random_number(times, start_number, end_number) {
        Ok(ids) => ids,
        Err(e) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("ランダム番号生成に失敗しました: {}", e),
            ));
        }
    };

    println!("[DEBUG]:[リクエスト]問題数:{}, テスト範囲{}~{}: [レスポンス]要素数:{}, 生成数値{:?}", times, start_number,end_number, row_id_list.len(), row_id_list);

    let sql_query = generate_sql_query(&book_name, row_id_list);

    let rows = execute_sql_query(&pool, &sql_query).await.unwrap();

    let english_words: Vec<String> = rows.iter().map(|r| r.english_word.clone()).collect();
    let japanese_words: Vec<String> = rows.iter().map(|r| r.japanese_word.clone()).collect();

    let question_sheet = gen_test_pdf(english_words, japanese_words);

    let file_path = "temp_question_sheet.pdf";
    question_sheet.render_to_file(file_path).expect("Failed to write PDF file");

    let mut file = File::open(file_path).expect("Failed to open PDF file.");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read PDF file");

    let pdf_base64 = general_purpose::STANDARD.encode(&buffer);

    Ok(Json(Response {
        test_data: pdf_base64,
    }))
}
