#![feature(float_bits_conv)]

extern crate cssparser;

use std::str::FromStr;

use ::cssparser::{Parser, ParserInput};

fn print_f32 (prefix: &str, val: f32) {
    let bits = val.to_bits ();
    println! ("{} - value: {}, bits: 0x{:x}", prefix, val, bits);
}

fn print_f64 (prefix: &str, val: f64) {
    let bits = val.to_bits ();
    println! ("{} - value: {}, bits: 0x{:x}", prefix, val, bits);
}

fn main () {
    let my_float = f32::from_str ("0.67").unwrap ();
    let my_double = f64::from_str ("0.67").unwrap ();

    print_f32 ("f32", my_float);

    print_f64 ("f64", my_double);
    
    let mut input = ParserInput::new ("0.67");
    let mut parser = Parser::new (&mut input);
    let parsed_float = parser.expect_number ().unwrap ();
    print_f32 ("parsed f32", parsed_float);
}
