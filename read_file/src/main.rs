use serde::Deserialize;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct User {
    fingerprint: u8,
    location: String,
}

#[derive(Deserialize, Debug)]
struct Data {
    data: Vec<User>,
}
fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<Data, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}
fn sqr(x: u8) -> u8 {
    x + x
}

fn main() {
    let u = read_user_from_file("./data/test.json").unwrap();
    println!("{:#?}", u);
    println!("1 fingerprint= {:#?}", u.data[1].fingerprint);
    let sqr_x = sqr(u.data[1].fingerprint);
    println!("Sqr of 1 fingerprint = {:#?}", sqr_x);
}
// TODO: implement in R1CS
