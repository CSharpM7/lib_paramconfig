#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    non_snake_case,
    unused
)]
#![deny(
    deprecated
)]

#[macro_use]
extern crate lazy_static;
use std::{
    env,
    io::*,
    collections::HashMap, 
    sync::Arc,
    arch::asm,
};
use parking_lot::RwLock;
pub mod data;

pub fn hash40(param: &str) -> u64 {
    return hash40::hash40(param).0;
}
fn main() {
    unsafe{
        match env::current_dir() {
            Ok(path) => {
                let mut file = path.display().to_string();
                file.push_str(r"\config_param.toml");
                data::read_config(format!("{}",file));
            },
            Err(_) => {
                println!("Could not find dir");
                std::process::exit(1);
            }
        };

    }
}
