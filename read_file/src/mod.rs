use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
    pub address: String,
    pub phone_numbers: Vec<String>,
}
