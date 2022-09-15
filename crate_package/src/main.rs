mod my_struct;
use std::convert::TryInto;

// crate will check for the root path same as src
use crate::my_struct::{plus_two, MyStruct};
// use the name of folder, and Rust will check for mod.rs
mod nested;
// use crate::nested::another::Another;
// use crate::nested::nestedStruct;
// another way
// use crate::nested::{nestedStruct, Another};
// another way
mod prelude {
    pub use crate::nested::{nestedStruct, plus_four, Another};
}
use crate::nested::plus_three;
use crate::prelude::*;
use rand::prelude::*;
mod bird;
fn main() {
    let rd: u8 = random();
    println!("random number= {}", rd);
    let ms = MyStruct { nombo: 32 };
    println!("{} ", ms.nombo);
    let ns: nestedStruct = nestedStruct { nombo_nested: 94 };
    println!("{}", ns.nombo_nested);
    let asd: Another = Another { another_nombo: 87 };
    println!("{}", asd.another_nombo);
    let x = 23;
    let y: u32 = plus_one(x);
    println!("y= {}", y);
    // .into turn to u64
    let z: u64 = plus_two(x.into());
    println!("z= {}", z);
    let a: u32 = plus_three(x);
    println!("a= {}", a);
    let b: u32 = plus_four(x);
    println!("b= {}", b);
    // let c: u8 = plus_eight(x.try_into().unwrap());
    // println!("c= {}", c);
}

fn plus_one(x: u32) -> u32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn plus_one_works() {
        let y: u32 = plus_one(23);
        assert_eq!(y, 24);
    }
    #[test]
    fn plus_two_works() {
        let z: u64 = plus_two(43);
        assert_eq!(z, 45);
    }
    #[test]
    fn plus_three_works() {
        let a: u32 = plus_three(56);
        assert_eq!(a, 59);
    }
    #[test]
    fn plus_four_works() {
        let b: u32 = plus_four(82);
        assert_eq!(b, 86);
    }
}
