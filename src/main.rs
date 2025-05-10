use std::env;
use std::sync::Arc;
use std::fs::File;
use std::io::Read;

use dotenv::dotenv;
use axum::{
    routing::post,
    response::Json, Router, extract::State,
};
use validator::Validate;
use hyper::Method;
use tower_http::cors::{CorsLayer, AllowHeaders, Any};
use serde::{Deserialize, Serialize};
use rand::prelude::*;
use sqlx::mysql::MySqlPool;
use genpdf::{elements, style, Alignment, Document};
use genpdf::fonts::FontData;
use chrono::prelude::Local;
use base64::{engine::general_purpose, Engine};
use axum::response::IntoResponse;

#[derive(Debug, sqlx::FromRow)]
struct Test {
    english_word: String,
    japanese_word: String
}

#[derive(Serialize, Deserialize, Validate)]
struct Request {
    english_word_book: String,
    #[validate(range(min = 1))]
    times: u16,
    #[validate(range(min = 1))]
    start_number: u16, 
    #[validate(range(min = self.start_number))]
    end_number: u16,
}

#[derive(Serialize)]
struct Response {
    test_data: String
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    // databaseに接続する
    println!("{}", "リクエストを受け取りました。");

    let pool = MySqlPool::connect(&database_url).await?;
    let app_state = Arc::new(pool);

    let cors = CorsLayer::new()
        .allow_origin(Any) // 　本番環境では特定のオリジンのみを許可
        .allow_headers(AllowHeaders::any()) 
        .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS]);

    // Routerを作成し、/generate-testパスでハンドラを設定
    let app = Router::new()
        .route("/generate-test", post(generate_test_handler))
        .with_state(app_state)
        .layer(cors);

    let port = env::var("PORT").expect("PORT must be set.");
    let port_number = format!("0.0.0.0:{}",port);
    let listener = tokio::net::TcpListener::bind(port_number).await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn generate_test_handler(
    State(pool): State<Arc<MySqlPool>>, 
    Json(request): Json<Request>
) -> impl IntoResponse { 
    let book_name = request.english_word_book;
    let times = request.times;
    let start_number = request.start_number;
    let end_number = request.end_number;

    let row_id_list = generate_random_number(times, start_number, end_number);

    let sql_query = generate_sql_query(&book_name, row_id_list);

    let rows = execute_sql_query(&pool, &sql_query).await.unwrap();

    let english_words: Vec<String> = rows.iter().map(|r| r.english_word.clone()).collect();
    let japanese_words: Vec<String> = rows.iter().map(|r| r.japanese_word.clone()).collect();

    let question_sheet = gen_test_pdf(english_words, japanese_words);
    println!("{}", "pdfデータの生成完了");

    let file_path = "temp_question_sheet.pdf";
    question_sheet.render_to_file(file_path).expect("Failed to write PDF file");

    let mut file = File::open(file_path).expect("Failed to open PDF file.");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read PDF file");

    let pdf_base64 = general_purpose::STANDARD.encode(&buffer);

    Json(Response {
        test_data: pdf_base64,
    })
}

fn generate_random_number(times: u16, start_number: u16, end_number: u16) -> Vec<u16> {
    let mut rng = rand::rng();
    let mut ids:Vec<u16> = Vec::new();

    for _ in 1..=times {
        let random_number: u16 = rng.random_range(start_number..=end_number);
        ids.push(random_number);
    }

    println!("ランダムに問題番号を生成しました。：{:?}", ids);

    ids
}

fn generate_sql_query(book_name: &str, ids: Vec<u16>) -> String {
    let id_list = ids.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(",");
    format!("SELECT english_word, japanese_word FROM {} WHERE id IN ({})", book_name, id_list)
}

async fn execute_sql_query(pool: &MySqlPool, query: &str) -> Result<Vec<Test>, sqlx::Error> {
    let rows: Vec<Test> = sqlx::query_as(query)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

fn gen_test_pdf(question_list: Vec<String>, answer_list: Vec<String>) -> Document{
    println!("{}", "PDFファイルの生成中...");
    let title = "英単語テスト";

    // フォント読み込み
    let font_family: genpdf::fonts::FontFamily<FontData> =
        genpdf::fonts::from_files("fonts/", "NotoSansJP", None)
            .expect("failed to read fonts");

    let mut doc = genpdf::Document::new(font_family);

    // ページマージンの設定（任意）
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    // タイトル（中央寄せ・大きめ・太字）
    let style = style::Style::new().bold().with_font_size(18);
    let mut title_paragraph = elements::Paragraph::new("");
    title_paragraph.push_styled(title, style);
    title_paragraph.set_alignment(Alignment::Center);
    doc.push(title_paragraph);

    // 日付欄（右寄せ）
    let date = Local::now().format("%Y/%m/%d").to_string();
    let mut date_paragraph = elements::Paragraph::new(date);
    date_paragraph.set_alignment(Alignment::Right);
    doc.push(date_paragraph);

    // 単語リスト（番号 + 単語 + 解答欄）
    let words = question_list;

    for (i, word) in words.iter().enumerate() {
        let mut layout = elements::LinearLayout::vertical();
        layout.push(elements::Paragraph::new(""));
        layout.push(elements::Paragraph::new(format!("{}. {}                         {}", i + 1, word,"________________")));
        layout.push(elements::Paragraph::new(""));
        doc.push(layout);
    }

    doc.push(genpdf::elements::Break::new(50));

    let second_title ="英単語テスト解答";

    // タイトル（中央寄せ・大きめ・太字）
    let style = style::Style::new().bold().with_font_size(18);
    let mut title_paragraph = elements::Paragraph::new("");
    title_paragraph.push_styled(second_title, style);
    title_paragraph.set_alignment(Alignment::Center);
    doc.push(title_paragraph);

    // 日付欄（右寄せ）
    let date = Local::now().format("%Y/%m/%d").to_string();
    let mut date_paragraph = elements::Paragraph::new(date);
    date_paragraph.set_alignment(Alignment::Right);
    doc.push(date_paragraph);

    for (i, word) in words.iter().enumerate() {
        let mut layout = elements::LinearLayout::vertical();
        layout.push(elements::Paragraph::new(""));
        layout.push(elements::Paragraph::new(format!("{}. {}                         {}", i + 1, word,answer_list[i])));
        layout.push(elements::Paragraph::new(""));
        doc.push(layout);
    }

    doc
}

