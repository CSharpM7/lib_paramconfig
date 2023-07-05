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
    lib::lua_const::*
};
use std::{
    collections::HashMap, 
    sync::Arc,
    arch::asm,
};
use parking_lot::RwLock;

pub mod data;
mod hook;

#[skyline::main(name = "libparam_commonconfig")]
pub fn main() {
    println!("[libparam_commonconfig::main] Loading...");
    if data::install() {
        println!("[libparam_commonconfig::main] Hooking...");
        hook::install();
    }
    else{
        println!("[libparam_commonconfig::main] No commonconfig data found");
    }
}