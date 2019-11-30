extern crate chrono;
use std::io;
use chrono::prelude::*;

fn main() {
    println!("OOP in Rust!\n");

    let mut a = Employee {
        demo: Demographics {
            age: 0,
            date_of_birth: Utc.ymd(1970, 5, 8).and_hms(12,0,9)
        },
        person: Person {
            name: "Tyler".to_string()
        }
    };

    // Composition - Demographics object in Employee
    println!("Age: {}", a.demo.calc_age());

    // Polymorphism not supported
	poly(&a);

	// Struct embedding is not inheritance
    poly_p(&a.person); // Overloading not supported 

	// Polymorphism as trait (interface) with pointer to Employee object
	poly_prt(&a);

    let mut name = String::new();
    println!("Press Enter to quit.");
    io::stdin().read_line(&mut name)
               .expect("Failed to read line.");
}

fn poly_p(p: &Person) {
    p.print();
}

fn poly(e: &Employee) {
    e.print();
}

fn poly_prt<T>(thing: &T) where T: Printable {
    thing.print();
}

pub trait Printable {
    fn print(&self);
}

pub struct Person {
    pub name: String
}

impl Printable for Person {
    fn print(&self) {
        println!("Person Print {}", self.name);
    }
}

pub struct Demographics {
    // Cannot make this private and not require initialization 
    age: i32,
    pub date_of_birth: chrono::DateTime<chrono::offset::Utc>
}

impl Demographics {
    pub fn calc_age(self: &mut Self) -> i32 {
        self.age = Utc::now().year() - self.date_of_birth.year();
        return self.age;
    }
}

pub struct Employee {
    pub person: Person,
    pub demo: Demographics 
}

impl Printable for Employee {
    fn print(&self) {
        println!("Employee Print {}", self.person.name);
        println!("Employee Age {}", self.demo.age);
    }
}
