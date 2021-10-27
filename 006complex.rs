use std::fmt;

#[derive(Debug)]
struct Complex {
    real: f64,
    imaginary: f64
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = 
            if self.imaginary > 0.0 {"+"} else {"-"};
        return write!(f, "{} {} {}i", self.real, sign, self.imaginary.abs());
    }
}

fn main() {
    let z = Complex {
        real: 12.5,
        imaginary: -12.6
    };

    let w = Complex {
        real: -5.8,
        imaginary: 6.5
    };

    println!("z = {}, w = {}", z, w);
}