use std::env;
use std::time::Instant;

use rand::prelude::*;
use sqlx::mysql::MySqlPoolOptions;
use genpdf::{elements, style, Alignment, Document};
use genpdf::fonts::FontData;
use chrono::prelude::Local;
use dotenv::dotenv;

#[derive(Debug, sqlx::FromRow)]
struct Test {
    english_word: String,
    japanese_word: String
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    dotenv().ok();

    let start = Instant::now();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    // databaseに接続する
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await?;

    let times = 10;
    let start_number = 1;
    let end_number = 2027;

    let row_id_list = generate_random_number(times, start_number, end_number);

    let id_list = row_id_list.iter()
        .map(|id| id.to_string()) 
        .collect::<Vec<_>>()
        .join(",");

    let sql_query = format!(
        "SELECT * FROM shisutan WHERE id IN ({});",
        id_list
    );

    let rows: Vec<Test>= sqlx::query_as(&sql_query)
        .fetch_all(&pool)
        .await?;

    let english_words: Vec<String> = rows.iter().map(|r| r.english_word.clone()).collect();
    let japanese_words: Vec<String> = rows.iter().map(|r| r.japanese_word.clone()).collect();
    
    println!("問題集：{:?}",english_words);   
    println!("解答集: {:?}", japanese_words);

    let question_sheet = gen_pdf(english_words, japanese_words);

    question_sheet.render_to_file("./question_sheet.pdf").expect("failed to write PDF file");

    let duration = start.elapsed();

    println!("実行時間：{:?}", duration);

    Ok(())
}

fn generate_random_number(times: u16, start_number: u16, end_number: u16) -> Vec<u16> {
    let mut rng = rand::rng();
    let mut ids:Vec<u16> = Vec::new();

    for _ in 1..=times {
        let random_number: u16 = rng.random_range(start_number..=end_number);
        ids.push(random_number);
    }

    return ids;
}

fn gen_pdf(question_list: Vec<String>, answer_list: Vec<String>) -> Document{
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
    println!("date : {}", date);
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
    println!("date : {}", date);
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
    

    // ファイル出力
    // doc.render_to_file("./demo_style.pdf").expect("failed to write PDF file");

    println!("PDFファイルの生成が完了しました。");

    return doc
}