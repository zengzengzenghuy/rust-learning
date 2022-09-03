mod structTemplate;
use crate::structTemplate::Person;

fn main() {
    let Andy = Person {
        name: String::from("Andy"),
        age: 52,
    };
    println!("Hello, world!");
    println!("Andy is {:?}", Andy);
}
