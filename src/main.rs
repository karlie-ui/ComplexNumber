#![allow(non_snake_case)] // inner attribute

mod ComplexNumber;

#[allow(unused)]
fn module_test() {
    let u = ComplexNumber::ComplexNumber::new(2_f64, -1_f64);
    println!("### ComplexNumber created in main() ###");
    println!("u={u}, Re[u]={}, Im[u]={}, Arg[u]={}, |u|={}", u.r(), u.i(), u.a(), u.m());
}

fn main() {
    // crate::ComplexNumber::test_CN();
    crate::ComplexNumber::test_file_io();
}