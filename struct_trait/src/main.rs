mod structTemplate;
use crate::structTemplate::{Person, School, Teacher};

fn main() {
    let Andy = Person {
        name: String::from("Andy"),
        age: 52,
    };
    // print the whole struct using Debug format {:?}
    println!("Andy is {:?}", Andy);
    // use normal print to print the element
    println!("Andy's age {}", Andy.age);
    // define using new fn
    let mut David: Person = School::new(String::from("Daivd"));
    println!("David is {:?}", David);
    let mut Bob: Teacher = School::new(String::from("Bob"));
    println!("Bob is {:?}", Bob);
    // return ()
    let a = Bob.subject();
    println!("{:?}", a);
}
