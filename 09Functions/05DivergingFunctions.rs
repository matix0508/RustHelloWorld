// #![feature(never_type)]

// fn main() {
//     let x: ! = panic!("This call never returns.");
//     println!("You will never see this line!");
// }

fn main() {
    fn sum_odd_numers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            let addition: u32 = match i%2 == 1 {
                true => i,
                // On the other hand, the "continue" expression does not return
                // u32, but it is still fine, because it never returns and therefore
                // does not violate the type requirements of the match expression.
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!("Sum of odd numbers up to 9: {}", sum_odd_numers(9));
}

