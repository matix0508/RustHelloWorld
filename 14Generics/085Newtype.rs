struct Years(i64);
fn main() {
    let years = Years(42);
    let _years_as_primitive_1: i64 = years.0;
    let Years(_years_as_primitive_2) = years;
}