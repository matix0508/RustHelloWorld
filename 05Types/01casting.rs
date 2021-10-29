#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    let integer: u8 = decimal; // ERROR: no implicit conversion

    let integer = decimal as u8;
    let character = integer as char;

    let character = decimal as char; // Error: float cannot be converted to char

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    
}