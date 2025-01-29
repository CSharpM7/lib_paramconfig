
#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    non_snake_case,
    unused,
    warnings
)]
#![deny(
    deprecated
)]

use once_cell::sync::Lazy;

use param_config::{
    *
};
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

pub mod data;
pub mod plugin;

#[skyline::main(name = "libparam_config")]
pub fn main() {
    println!("[libparam_config::nro] Loading...");
    if !plugin::install() {
        println!("[libparam_config::nro] No param data found");
    }
    else{
        println!("[libparam_config::nro] Loaded");
    }
}
