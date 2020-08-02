#![feature(proc_macro_hygiene)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

mod per_frame;
mod utils;

#[skyline::main(name = "SpikesOnlyGamemode")]
pub fn main() {
    per_frame::install();
}
