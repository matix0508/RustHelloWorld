fn give_adult(drink: Option<&str>) {
    match drink {
        Some("Lemonade") => println!("Yuck! Too Sugary"),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No drink? Oh well"),
    }
}

fn drink(drink: Option<&str>) {
    let inside = drink.unwrap();
    if inside == "lemonade" {panic!("AAAAaaaaaaa!!!"); }

    println!("I love {}s", inside);
}

fn main() {
    let water = Some("water");
    let lemonade = Some("lemonade");
    let void = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    let nothing = None;

    drink(coffee);
    drink(nothing);

    drink(coffee);
    drink(nothing);
}