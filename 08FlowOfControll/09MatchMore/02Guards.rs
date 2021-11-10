fn main() {
    let pair = (2, -2);

    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("There are twins"),
        (x, y) if x + y == 0 => println!("Antimater, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation"),
    }

    let number: u8 = 4;

    match number {
        i if i ==0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => println!("Fell through"), // Should not be possible to reach
    }
}