pub mod garden;

use crate::garden::vegetables::VegetableTypes;

fn main() {
    let plant = VegetableTypes::Asparagus;
    println!("I'm growing: {:?}!", plant);
}
