use rand::prelude::*;

pub fn generate_random_number(times: u16, start_number: u16, end_number: u16) -> Vec<u16> {
    let mut rng = rand::rng();
    let mut ids: Vec<u16> = Vec::new();

    for _ in 1..=times {
        let random_number: u16 = rng.random_range(start_number..=end_number);
        ids.push(random_number);
    }

    println!("ランダムに問題番号を生成しました。：{:?}", ids);

    ids
}

pub fn generate_sql_query(book_name: &str, ids: Vec<u16>) -> String {
    let id_list = ids.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(",");
    format!("SELECT english_word, japanese_word FROM {} WHERE id IN ({})", book_name, id_list)
}
