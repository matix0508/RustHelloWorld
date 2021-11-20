struct Cardial;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardial {}
impl Blue for BlueJay {}

fn red<T: Red>(_: &T) -> &'static str { "red" }
fn blue<T: Blue>(_: &T) -> &'static str { "blue" }

fn main() {
    let cardial = Cardial;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    println!("A Cardial is {}", red(&cardial));
    println!("A blue jay is {}", blue(&blue_jay));
    // println!("A Turkey is {}", red(&_turkey));
}