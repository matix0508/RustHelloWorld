fn drink(beverage: &str) {
    if beverage == "lemonade" { panic!("AAAaaaa!!!!!");}

    println!("Some refreshing {} is all I need.", beverage);
}

fn main() {
    drink("water");
    drink("lemonade");
}