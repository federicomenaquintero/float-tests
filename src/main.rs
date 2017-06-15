#![feature(float_bits_conv)]

extern crate cssparser;

use std::str::FromStr;

use ::cssparser::{Parser, ParserInput, Token, ToCss};

fn print_f32 (prefix: &str, val: f32) {
    let bits = val.to_bits ();
    println! ("{} - value: {}, bits: {b:b} (0x{b:x})", prefix, val, b = bits);
}

fn print_f64 (prefix: &str, val: f64) {
    let bits = val.to_bits ();
    println! ("{} - value: {}, bits: {b:b} (0x{b:x})", prefix, val, b = bits);
}

fn main () {
    let my_float = f32::from_str ("0.67").unwrap ();
    let my_double = f64::from_str ("0.67").unwrap ();

    print_f32 ("f32::from_str", my_float);

    print_f64 ("f64::from_str", my_double);
    
    let mut input = ParserInput::new ("0.67%");
    let mut parser = Parser::new (&mut input);

    let token = parser.next ().unwrap ();
    match token {
        Token::Percentage (ref pv) => {
            println! ("Token.to_css_string() = \"{}\"", token.to_css_string ());
            print_f32 ("Token::Percentage(PercentageValue.value)", pv.unit_value);
        }

        _ => {
            panic! ("expected a number");
        }
    }
}
