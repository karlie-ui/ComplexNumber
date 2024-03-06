#![allow(non_snake_case)] // inner attribute
#![allow(unused_imports)]

mod ComplexNumber;
use ComplexNumber::{test_CN_parsing, test_file_io};

#[allow(unused)]
fn module_test() {
    let u = ComplexNumber::ComplexNumber::new(2_f64, -1_f64);
    println!("### ComplexNumber created in main() ###");
    println!(
        "u={u}, Re[u]={}, Im[u]={}, Arg[u]={}, |u|={}",
        u.r(),
        u.i(),
        u.a(),
        u.m()
    );
}

fn main() {
    // module_test();
    // crate::ComplexNumber::test_CN();
    test_file_io();
    // let z = match test_CN_parsing() {
    //     Ok(zz) => zz,
    //     Err(zz) => panic!("Error while parsing complex number!")
    // };
    // println!("z={}", z);
    // test_CN_parsing();
}
