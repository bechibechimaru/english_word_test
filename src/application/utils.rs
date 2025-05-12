use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use rand::prelude::*;

#[derive(Debug)]
struct GenerateError(String);

impl fmt::Display for GenerateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GenerateError: {}", self.0)
    }
}

impl Error for GenerateError {}

pub fn generate_random_number(times: u16, start_number: u16, end_number: u16) -> Result<Vec<u16>, Box<dyn Error>> {

    let validation = range_validation(times, start_number, end_number);
    
    if !validation{
        return Err(Box::new(GenerateError("range error".to_string())));
    }

    let mut rng = rand::rng();
    let mut random_number_list: Vec<u16> = Vec::new();
    
    // 重複確認用
    let mut temp_number_list: HashSet<u16> = HashSet::new();

    while random_number_list.len() < times.into() {
        let random_number: u16 = rng.random_range(start_number..=end_number);

        if temp_number_list.insert(random_number) {
            random_number_list.push(random_number);
        }
    }

    Ok(random_number_list)
}

pub fn generate_sql_query(book_name: &str, ids: Vec<u16>) -> String {
    let id_list = ids.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(",");
    format!("SELECT english_word, japanese_word FROM {} WHERE id IN ({})", book_name, id_list)
}

pub fn range_validation(times: u16, start_number: u16, end_number: u16) -> bool {

    let range = end_number - start_number + 1;

    if times > range {
        return false;
    }
    
    return true;
}
