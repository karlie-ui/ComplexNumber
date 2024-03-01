#![allow(non_snake_case)] // inner attribute

// use std::cmp::Ordering;
use std::io;
use std::ops::{Add, Div, Mul, Sub};

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

trait Number {
    fn new(r: f64, i: f64) -> Self;
}

/// Structure representing a complex number
struct ComplexNumber {
    re: f64,
    im: f64,
    arg: f64,
    z: f64,
    t: Type,
}

/// Buld-in functions for ComplexNumber structure
impl Number for ComplexNumber {
    fn new(r: f64, i: f64) -> ComplexNumber {
        ComplexNumber {
            re: r,
            im: i,
            z: (r * r + i * i).sqrt(),
            arg: (i).atan2(r),
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
        let z = self.z * other.z;
        let arg = self.arg + other.arg;
        ComplexNumber::new(z * arg.cos(), z * arg.sin())
    }
}

/// '/' operator for ComplexNumber
impl Div<&ComplexNumber> for &ComplexNumber {
    type Output = ComplexNumber;
    fn div(self, other: &ComplexNumber) -> ComplexNumber {
        let z = self.z / other.z;
        let arg = self.arg - other.arg;
        ComplexNumber::new(z * arg.cos(), z * arg.sin())
    }
}

/// Function parsing complex number
fn parse_complex(num: &str, s: char) -> Result<(f64, f64), (f64, f64)>  {
    match num.find(s) {
        Some(i) => Ok((
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
            Err((0f64, 0f64))
        }
    }
}

/// Function getting user input (not finished)
fn get_user_input() -> Result<(f64, f64), (f64, f64)> {
    println!("Please input a complex number in form a+bi");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let num: &str = input.trim().trim_end_matches('i');
    println!("{num}");
    parse_complex(&num, '+')
}

/// Testing function
fn main() {
    let u = ComplexNumber::new(2_f64, -1_f64);
    let w = ComplexNumber::new(2_f64, -1_f64);
    let v = ComplexNumber::new(3_f64, 6_f64);
    // let z = ComplexNumber::new(2_f64, -1_f64);
    // let z = ComplexNumber::new(3_f64, -1_f64) + ComplexNumber::new(3_f64, -1_f64);
    println!("### Complex numbers ### ");
    println!("Number u={}{:+}i is {}", u.re, u.im, u.t.get());
    println!("Number w={}{:+}i is {}", w.re, w.im, w.t.get());
    println!("Number v={}{:+}i is {}", v.re, v.im, v.t.get());
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
    let mut z: ComplexNumber = &w + &v;
    println!("Number z=w+v={}{:+}i", z.re, z.im);
    println!("arg(z)={}, |z|={}", z.arg * 180. / 3.1415, z.z);
    z = &v + 1.2;
    println!("Number z=v+1.2={}{:+}i", z.re, z.im);
    z = &w - &v;
    println!("Number z=w-v={}{:+}i", z.re, z.im);
    z = &w * &v;
    println!("Number z=w*v={:.2}{:+.2}i", z.re, z.im);
    z = &w / &v;
    println!("Number z=w/v={:.2}{:+.2}i", z.re, z.im);
    let x = match get_user_input() {
        Ok(xx) => xx,
        Err(xx) => xx
    };
    println!("x = {}{:+}i", x.0, x.1);
}
