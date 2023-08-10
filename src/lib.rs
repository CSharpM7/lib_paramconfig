
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


use smash::{
    hash40,
    app::{lua_bind::*, *},
    lib::lua_const::*,
    phx::*,
};
use std::{
    collections::HashMap, 
    sync::Arc,
    arch::asm,
};
use parking_lot::RwLock;

pub mod data;
mod hook;

#[skyline::main(name = "libparam_config")]
pub fn main() {
    println!("[libparam_config::main] Loading...");
    if data::install() {
        //Delay install for testing purposes...I don't think this should be necessary on V1
        let hook_thread = std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(100));
            println!("[libparam_config::main] Hooking...");
            hook::install();
        });
        hook_thread.join();
    }
    else{
        println!("[libparam_config::main] No param data found");
    }
}
