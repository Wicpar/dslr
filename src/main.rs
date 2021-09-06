use crate::math::percentiles;

pub mod math;

fn main() {
    println!("Hello, world!");

    println!("{:?}", percentiles(&[0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0], &[0.0, 0.25, 0.50, 0.75, 1.0]))
}
