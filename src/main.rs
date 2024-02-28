#![allow(non_snake_case)] // inner attribute

// use std::cmp::Ordering;

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
trait Number <T, U> {
    fn new(r: T, i: U) -> ComplexNumber<T, U>;
}
*/

/// Structure representing a complex number
struct ComplexNumber {
    re: f64,
    im: f64,
    t: Type,
}

/// Buld-in functions for ComplexNumber structure
impl ComplexNumber {
    fn new(r: f64, i: f64) -> ComplexNumber {
        ComplexNumber {
            re: r,
            im: i,
            t: match i as i64 {
                0 => Type::Real,
                _ => Type::Complex
            }
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

/// Testing function
fn main() {
    let u = ComplexNumber::new(2_f64, -1_f64);
    let w = ComplexNumber::new(2_f64, 0_f64);
    let v = ComplexNumber::new(3_f64, -1_f64);
    let z = ComplexNumber::new(2_f64, -1_f64);
    println!("### Complex numbers ### ");
    println!("Number u={}{:+}i is {}", u.re, u.im, u.t.get());
    println!("Number w={}{:+}i is {}", w.re, w.im, w.t.get());
    println!("Number v={}{:+}i is {}", v.re, v.im, v.t.get());
    println!("Number z={}{:+}i is {}", z.re, z.im, z.t.get());
    println!("### Logical operators tests ### ");
    println!("Are u and w equal? {}", match u == w {true => "Yes!", false => "No!"});
    println!("Are u and v equal? {}", match u == v {true => "Yes!", false => "No!"});
    println!("Are u and z equal? {}", match u == z {true => "Yes!", false => "No!"});
    println!("Are w and v different? {}", match u != v {true => "Yes!", false => "No!"});
}
