fn main() {
    let long_lived_binding = 1;
    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);

        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);

        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
    }

    println!("outsied inner block: {}", shadowed_binding);

    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);

    // println!("outer short: {}", short_lived_binding); // Error
 
    println!("outer long: {}", long_lived_binding);
}