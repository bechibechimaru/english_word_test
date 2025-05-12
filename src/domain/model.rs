use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, sqlx::FromRow)]
pub struct Test {
    pub english_word: String,
    pub japanese_word: String
}

#[derive(Serialize, Deserialize, Validate)]
pub struct Request {
    pub english_word_book: String,
    #[validate(range(min = 1))]
    pub times: u16,
    #[validate(range(min = 1))]
    pub start_number: u16, 
    #[validate(range(min = "self.start_number"))]
    pub end_number: u16,
}

#[derive(Serialize)]
pub struct Response {
    pub test_data: String
}
