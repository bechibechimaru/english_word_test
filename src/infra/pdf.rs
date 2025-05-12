use genpdf::{elements, style, Alignment, Document};
use genpdf::fonts::FontData;
use chrono::prelude::Local;

pub fn gen_test_pdf(question_list: Vec<String>, answer_list: Vec<String>) -> Document {
    println!("{}", "PDFファイルの生成中...");
    let title = "英単語テスト";

    let font_family: genpdf::fonts::FontFamily<FontData> =
        genpdf::fonts::from_files("fonts/", "NotoSansJP", None)
            .expect("failed to read fonts");

    let mut doc = genpdf::Document::new(font_family);

    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    let style = style::Style::new().bold().with_font_size(18);
    let mut title_paragraph = elements::Paragraph::new("");
    title_paragraph.push_styled(title, style);
    title_paragraph.set_alignment(Alignment::Center);
    doc.push(title_paragraph);

    let date = Local::now().format("%Y/%m/%d").to_string();
    let mut date_paragraph = elements::Paragraph::new(date);
    date_paragraph.set_alignment(Alignment::Right);
    doc.push(date_paragraph);

    for (i, word) in question_list.iter().enumerate() {
        let mut layout = elements::LinearLayout::vertical();
        layout.push(elements::Paragraph::new(""));
        layout.push(elements::Paragraph::new(format!("{}. {}                         {}", i + 1, word,"________________")));
        layout.push(elements::Paragraph::new(""));
        doc.push(layout);
    }

    doc.push(genpdf::elements::Break::new(50));

    let second_title ="英単語テスト解答";

    let mut title_paragraph = elements::Paragraph::new("");
    title_paragraph.push_styled(second_title, style);
    title_paragraph.set_alignment(Alignment::Center);
    doc.push(title_paragraph);

    let date = Local::now().format("%Y/%m/%d").to_string();
    let mut date_paragraph = elements::Paragraph::new(date);
    date_paragraph.set_alignment(Alignment::Right);
    doc.push(date_paragraph);

    for (i, word) in question_list.iter().enumerate() {
        let mut layout = elements::LinearLayout::vertical();
        layout.push(elements::Paragraph::new(""));
        layout.push(elements::Paragraph::new(format!("{}. {}                         {}", i + 1, word, answer_list[i])));
        layout.push(elements::Paragraph::new(""));
        doc.push(layout);
    }

    doc
}
