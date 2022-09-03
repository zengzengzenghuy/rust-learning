#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: u8,
}
#[derive(Debug)]
pub struct Teacher {
    pub name: String,
    pub age: u8,
    pub title: String,
}
pub trait School {
    fn new(name: String) -> Self;
    fn birthday(&mut self);
    fn subject(&self);
}

impl School for Person {
    fn new(name: String) -> Person {
        return Person { name, age: 0 };
    }

    fn birthday(&mut self) {
        self.age += 1;
        println!(
            "Happy Birthday {}, you are now {} year(s) old",
            self.name, self.age
        );
    }

    fn subject(&self) {
        println!("{} took subject Rust!", self.name);
    }
}

impl School for Teacher {
    fn new(name: String) -> Teacher {
        return Teacher {
            name,
            age: 0,
            title: String::from("Teacher"),
        };
    }

    fn birthday(&mut self) {
        self.age += 1;
        println!(
            "Happy Birthday {}, you are now {} year(s) old",
            self.name, self.age
        );
    }

    fn subject(&self) {
        println!("{} teaches subject Rust!", self.name);
    }
}
// Bug Box, dyn
fn new_user(job: &str, name: String) -> Box<dyn > {
    if job == "Student" {
        return Box::new(Person { name, age: 13 });
    } else {
        return Box::new(Teacher {
            name,
            age: 24,
            title: String::from("Teacher"),
        });
    }
}
