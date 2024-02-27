#![allow(non_snake_case)] // inner attribute

// use std::cmp::Ordering;

enum Type {
    Real,
    Complex,
}

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

struct ComplexNumber {
    re: f64,
    im: f64,
    t: Type,
}

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

impl PartialEq for ComplexNumber {
    fn eq(&self, other: &Self) -> bool {
        (&self.im == &other.im) && (&self.re == &other.re) 
    }
}

fn main() {
    let u = ComplexNumber::new(2_f64, -1_f64);
    let w = ComplexNumber::new(2_f64, 0_f64);
    let v = ComplexNumber::new(3_f64, -1_f64);
    let z = ComplexNumber::new(2_f64, -1_f64);
    println!("Number u={}{:+}i is {}", u.re, u.im, u.t.get());
    println!("Number w={}{:+}i is {}", w.re, w.im, w.t.get());
    println!("Number v={}{:+}i is {}", v.re, v.im, v.t.get());
    println!("Number z={}{:+}i is {}", z.re, z.im, z.t.get());
    println!("Are u and w equal? {}", match u == w {true => "Yes!", false => "No!"});
    println!("Are u and v equal? {}", match u == v {true => "Yes!", false => "No!"});
    println!("Are u and z equal? {}", match u == z {true => "Yes!", false => "No!"});
}
