#![allow(
    non_snake_case,
    unused,
    warnings
)]
#![deny(
    deprecated
)]
#[macro_use]
extern crate lazy_static;

mod example_fighter;

#[skyline::main(name = "libparam_config_example_nro")]
pub fn main() {
    example_fighter::install();
}
