#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

struct Unit;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32
}

#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point
}

fn main() {
    let name = String::from
}