#![allow(non_snake_case)] // inner attribute

// use std::cmp::Ordering;
use std::io;
use std::ops::{Add, Div, Mul, Sub};
use std::fmt;
use derive_getters::Getters;

/// Type enum (type of a number)
enum Type {
    Real,
    Complex,
}

/// Type enum functions
impl Type {
    fn get(&self) -> &str {
        match self {
            Type::Real => "real",
            Type::Complex => "complex",
        }
    }
}

/*
pub trait Number {
    fn new(r: f64, i: f64) -> Self;
}
*/

/// Structure representing a complex number
#[derive(Getters)]
pub struct ComplexNumber {
    #[getter(rename = "r")]
    re: f64,
    #[getter(rename = "i")]
    im: f64,
    a: f64,
    m: f64,
    t: Type,
}

/// Buld-in functions for ComplexNumber structure
impl ComplexNumber {
    pub fn new(r: f64, i: f64) -> ComplexNumber {
        ComplexNumber {
            re: r,
            im: i,
            m: (r * r + i * i).sqrt(),
            a: (i).atan2(r),
            t: match i as i64 {
                0 => Type::Real,
                _ => Type::Complex,
            },
        }
    }
}

/// Comparing two complex numbers
impl PartialEq for ComplexNumber {
    fn eq(&self, other: &Self) -> bool {
        (&self.im == &other.im) && (&self.re == &other.re)
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(&other)
    }
}

/// '+' operator for ComplexNumber
impl Add<&ComplexNumber> for &ComplexNumber {
    type Output = ComplexNumber;
    fn add(self, other: &ComplexNumber) -> ComplexNumber {
        ComplexNumber::new(self.re + other.re, self.im + other.im)
    }
}
impl Add<f64> for &ComplexNumber {
    type Output = ComplexNumber;
    fn add(self, other: f64) -> ComplexNumber {
        ComplexNumber::new(self.re + other, self.im)
    }
}

/// '-' operator for ComplexNumber
impl Sub<&ComplexNumber> for &ComplexNumber {
    type Output = ComplexNumber;
    fn sub(self, other: &ComplexNumber) -> ComplexNumber {
        ComplexNumber::new(self.re - other.re, self.im - other.im)
    }
}

/// '*' operator for ComplexNumber
impl Mul<&ComplexNumber> for &ComplexNumber {
    type Output = ComplexNumber;
    fn mul(self, other: &ComplexNumber) -> ComplexNumber {
        let m = self.m * other.m;
        let a = self.a + other.a;
        ComplexNumber::new(m * a.cos(), m * a.sin())
    }
}

/// '/' operator for ComplexNumber
impl Div<&ComplexNumber> for &ComplexNumber {
    type Output = ComplexNumber;
    fn div(self, other: &ComplexNumber) -> ComplexNumber {
        let m = self.m / other.m;
        let a = self.a - other.a;
        ComplexNumber::new(m * a.cos(), m * a.sin())
    }
}

/// Converting to String
impl fmt::Display for ComplexNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{:+}i", self.re, self.im)
    }
}

/// Function parsing complex number
fn parse_complex(num: &String, s: char) -> Result<ComplexNumber, ComplexNumber>  {
    let num: &str = num.trim().trim_end_matches('i');
    match num.find(s) {
        Some(i) => Ok(ComplexNumber::new(
            match num.get(0..i) {
                Some(x) => match x.trim().parse() {
                    Ok(xx) => xx,
                    Err(_) => 0f64
                },
                None => 0f64,
            },
            match num.get(i+1..) {
                Some(y) => match y.trim().parse() {
                    Ok(yy) => yy,
                    Err(_) => 0f64
                },
                None => 0f64,
            },
        )),
        None => {
            println!("Error in find(), None returned");
            Err(ComplexNumber::new(0f64, 0f64))
        }
    }
}

/// Function getting user input (not finished)
pub fn get_user_input() -> Result<ComplexNumber, ComplexNumber> {
    println!("Please input a complex number in form a+bi");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    // println!("{num}");
    parse_complex(&input, '+')
}

// use crate::ComplexNumber::ComplexNumber::get_user_input;
// use crate::ComplexNumber;

/// Testing function
pub fn test_CN() {
    let u = ComplexNumber::new(2_f64, -1_f64);
    let w = ComplexNumber::new(2_f64, -1_f64);
    let v = ComplexNumber::new(3_f64, 6_f64);
    // let z = ComplexNumber::new(2_f64, -1_f64);
    // let z = ComplexNumber::new(3_f64, -1_f64) + ComplexNumber::new(3_f64, -1_f64);
    println!("### Complex numbers ### ");
    println!("Number u={u} is {}", u.t.get());
    println!("Number w={w} is {}", w.t.get());
    println!("Number v={v} is {}", v.t.get());
    println!("### Logical operators tests ### ");
    println!(
        "Are u and w equal? {}",
        match u == w {
            true => "Yes!",
            false => "No!",
        }
    );
    println!(
        "Are u and v equal? {}",
        match u == v {
            true => "Yes!",
            false => "No!",
        }
    );
    println!(
        "Are w and v different? {}",
        match u != v {
            true => "Yes!",
            false => "No!",
        }
    );
    println!("### Arithmetic operators tests ### ");
    let mut z = &w + &v;
    println!("Number z=w+v={z}");
    println!("arg(z)={}, |z|={}", z.a * 180. / 3.1415, z.m);
    z = &v + 1.2;
    println!("Number z=v+1.2={z}");
    z = &w - &v;
    println!("Number z=w-v={z}");
    z = &w * &v;
    println!("Number z=w*v={z}");
    z = &w / &v;
    println!("Number z=w/v={z}");
    let x = match get_user_input() {
        Ok(xx) => xx,
        Err(xx) => xx
    };
    println!("x = {x}");
}