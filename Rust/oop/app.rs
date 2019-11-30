extern crate chrono;
use std::io;
use chrono::prelude::*; //{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use chrono::format::ParseError;

fn main() {
    println!("OOP in Rust!\n");

    //use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};

    let dt = Utc.ymd(1970, 5, 8).and_hms(12,0,9);  //NaiveDateTime::from_timestamp(61, 0), Utc);    
    //println!("dt, {}", dt);
    let d = Demographics {
        DateOfBirth: dt
    };

    let a = Employee {
        Demo: d,
        Per: Person {
            Name: "Tyler".to_string()
        }
    };

    //a.Print();
    //a.Per.Print();

    // Composition - Demographics object in Employee
    println!("Age: {}", a.Demo.GetAge());

    // Polymorphism not supported
	poly(&a);

	// Struct embedding is not inheritance
    poly_p(&a.Per); // Overloading not supported 

	// Polymorphism as trait (interface) with pointer to Employee object
	poly_prt(&a);
    
    let mut name = String::new();
    println!("Press Enter to quit.");
    io::stdin().read_line(&mut name)
        .expect("Failed to read line.");
}

fn poly_p(p: &Person) {
    p.Print();
}

fn poly(e: &Employee) {
    e.Print();
}

fn poly_prt<T>(thing: &T) where T: Printable {
    thing.Print();
}

pub trait Printable {
    fn Print(&self);
}

pub struct Person {
    pub Name: String
}

impl Printable for Person {
    fn Print(&self) {
        println!("Person Print {}", self.Name);
    }
}

pub struct Demographics {
    // Cannot make this private and not require initialization 
    // age: i32,
    pub DateOfBirth: chrono::DateTime<chrono::offset::Utc>
}

impl Demographics {
    pub fn GetAge(self: &Self) -> i32 {
        return Utc::now().year() - self.DateOfBirth.year();
    }
}

pub struct Employee {
    pub Per: Person,
    pub Demo: Demographics 
}

impl Printable for Employee {
    fn Print(&self) {
        println!("Employee Print {}", self.Per.Name);
        println!("Employee Age {}", self.Demo.GetAge());
    }
}
