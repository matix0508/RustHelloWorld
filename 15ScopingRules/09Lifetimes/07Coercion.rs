fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn main() {
    let first = 2; //  Longer lifetime

    {
        let second = 3; // Shorter Lifetime;
        println!("The product is {}", multiply(&second, &first));
        println!("{} is the first", choose_first(&first, &second));
    };
}