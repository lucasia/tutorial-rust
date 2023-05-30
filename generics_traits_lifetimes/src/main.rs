use std::cmp::PartialOrd;
use crate::traits::{Summary, Tweet};

pub mod generics;
mod traits;
mod lifetimes;

fn main() {
    generics::generics_main();

    traits::traits_main();

    lifetimes::lifetimes_main();

}