#![allow(non_snake_case)] // inner attribute

mod ComplexNumber;
use crate::ComplexNumber::test_CN;

fn main() {
    test_CN();
    let u = ComplexNumber::ComplexNumber::new(2_f64, -1_f64);
    println!("### ComplexNumber created in main() ###");
    println!("u={u}, Re[u]={}, Im[u]={}, Arg[u]={}, |u|={}", u.r(), u.i(), u.a(), u.m());
}