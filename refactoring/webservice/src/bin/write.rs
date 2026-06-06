use std::fmt::Write;
use std::fmt::Display;
use std::fmt::Result;
use std::fmt::Error;

pub fn main() {
    let mut s = String::new();
    write!(&mut s, "Hello, {}!", "world").unwrap();
    assert_eq!(s, "Hello, world!");
   
}


pub struct MyStruct {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub city: String,
    pub state: String,
}

pub trait Fly {
    fn fly(&self, speed: i32);
}

impl Fly for MyStruct {
    fn fly(&self, speed: i32) {
        println!("I'm flying at {} mph!", speed);
    }
}



impl Display for MyStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result {
        // fatal runtime error: stack overflow, aborting
        // write!(f, "{}", self)
        write!(f, "MyStruct: name: {}, age: {}, email: {}, phone: {}, address: {}, city: {}, state: {}", 
        self.name, self.age, self.email, self.phone, 
        self.address, self.city, self.state)

    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_my_struct() {
        let my_struct = MyStruct {
            name: "Guagua".to_string(),
            age: 25,
            email: "guagua@example.com".to_string(),
            phone: "1234567890".to_string(),
            address: "123 Main St".to_string(),
            city: "Anytown".to_string(),
            state: "CA".to_string(),
        };

        println!("{}", my_struct);
    }

    #[test]
    fn test_fly() {
        let my_struct = MyStruct {
            name: "Guagua".to_string(),
            age: 25,
            email: "guagua@example.com".to_string(),
            phone: "1234567890".to_string(),
            address: "123 Main St".to_string(),
            city: "Anytown".to_string(),
            state: "CA".to_string(),
        };

        my_struct.fly(100);
    }
}
