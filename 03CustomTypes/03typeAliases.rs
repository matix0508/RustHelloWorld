#![allow(dead_code)]

enum VeryVerboseEnumOfThingsToDWithNumbers {
    Add,
    Subtract
}



impl VeryVerboseEnumOfThingsToDWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x+y,
            Self::Subtract => x - y
        }
    }
}

type Operations = VeryVerboseEnumOfThingsToDWithNumbers;

fn main() {
    let _x = Operations::Add;
}