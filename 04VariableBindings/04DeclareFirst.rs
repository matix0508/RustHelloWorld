fn main() {
    let a_binding;
    {
        let x = 2;
        a_binding = x * x;
    }

    println!("a biding: {}", a_binding);

    let another_binding;

    // println!("another binding: {}", another_binding); ERROR: uninitilized var

    another_binding = 1;

    println!("another binding: {}", another_binding);
}