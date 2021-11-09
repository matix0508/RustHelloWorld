#![allow(unreachable_code)]

fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // this would break only the inner loop
            // break; 

            break 'outer; // this breaks the outer loop
        }
        println!("this point will never be reached");
    }
    println!("Exited the outer loop");
}