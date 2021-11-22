static NUM: i32 = 18;

fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn reference_lifetime() {
    {
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);
    }
    {
        let lifetime_num = 9;
        let coerce_static = coerce_static(&lifetime_num);
        println!("coerced_static: {}", coerce_static);
    }
    println!("NUM: {} stays accessibale!", NUM);
}

use std::fmt::Debug;

fn print_it(input: impl Debug + 'static) {
    println!("`static value passed in is {:?}", input);
}

fn trait_bound() {
    let i = 5;
    print_it(i);

    print_it(&i);
}

fn main() {
    reference_lifetime();

    trait_bound();

    
}