
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
/*
use std::{
    io,
    env,
    path::Path,
    convert::TryInto,
    str::FromStr,
    fs,
    thread::{self}
};*/
use parking_lot::RwLock;
use lazy_static::lazy_static;


pub fn hash_str_to_u64(param: &str) -> u64
{
    if param.starts_with("0x"){
        match u64::from_str_radix(param.trim_start_matches("0x"), 16){
            Ok(hex) => return hex,
            Err(err) => {println!("[libparam_config::data] Failed to parse {}",param); return 0}
        };
    }
    else 
    {
        return match param {
            "" => 0,
            " " => 0,
            _ => hash40(param),
        };
    }
}

pub struct CharacterParam {
    pub kind: i32,
    //pub is_fighter: bool,
    pub slots: Vec<i32>,
    pub ints: HashMap<(u64,u64),i32>,
    pub floats: HashMap<(u64,u64),f32>
}
impl PartialEq for CharacterParam {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind 
        && self.slots == other.slots
    }
}
impl Eq for CharacterParam {}
impl CharacterParam {
    pub fn get_int(&self, param_type: u64, param_hash: u64) -> Option<i32> {
        if let Some(value) = self.ints.get(&(param_type,param_hash)){
            return Some(*value);
        }
        return None;
    }
    pub fn get_float(&self, param_type: u64, param_hash: u64) -> Option<f32> {
        if let Some(value) = self.floats.get(&(param_type,param_hash)){
            return Some(*value);
        }
        let count = self.floats.len();
        println!("Count: {count}");
        return None;
    }
    pub fn insert_float(&mut self, param_type: u64, param_hash: u64, value: f32) {
        self.floats.insert((param_type,param_hash), value);
    }
}

pub struct ParamManager {
    pub kinds: Vec<i32>,
    pub has_all: bool,
    pub params: Vec<CharacterParam>
}

impl ParamManager {
    pub(crate) fn new() -> Self {
        Self {
            kinds: Vec::new(),
            has_all: false,
            params: Vec::new(),
        }
    }
    pub fn push(&mut self, params: CharacterParam) {
        let kind = params.kind;
        if !(self.kinds.contains(&kind)) {
            self.kinds.push(kind);
            //if kind == -1 {
            //    self.has_all = true;
            //}
        }
        self.params.push(params);
    }
    
    pub fn get_param_by_slot(&self,kind: i32, slot: i32) -> Option<&CharacterParam> {
        for params in &self.params{
            if (params.kind == kind) {
                if params.slots.contains(&slot) {
                    return Some(params);
                }
            }
        }
        return None
    }
    pub fn get_param(&self,kind: i32, slots: Vec<i32>) -> Option<&CharacterParam> {
        for params in &self.params{
            if (params.kind == kind) {
                if params.slots == slots {
                    return Some(params);
                }
            }
        }
        return None
    }

    pub fn update_int(&mut self,kind: i32, slots: Vec<i32>,index: (u64,u64),value: i32) {
        for param in &mut self.params {
            if (param.kind == kind) {
                if param.slots == slots {
                    param.ints.insert(index, value);
                    return;
                }
            }
        }
        let mut newparams = CharacterParam {
            kind: kind,
            slots: slots,
            ints: HashMap::new(),
            floats: HashMap::new()
        };
        newparams.ints.insert(index,value);
        self.push(newparams);
    }
    pub fn update_float(&mut self,kind: i32, slots: Vec<i32>,index: (u64,u64),value: f32) {
        for param in &mut self.params {
            if (param.kind == kind) {
                if param.slots == slots {
                    param.floats.insert(index, value);
                    return;
                }
            }
        }
        let mut newparams = CharacterParam {
            kind: kind,
            slots: slots,
            ints: HashMap::new(),
            floats: HashMap::new()
        };
        newparams.floats.insert(index,value);
        self.push(newparams);
    }
    
}

lazy_static! {
    pub static ref PARAM_MANAGER: RwLock<ParamManager> = RwLock::new(ParamManager::new());
}

pub unsafe fn add_int(kind: i32, slots: Vec<i32>,index: (u64,u64),value: i32)
{
    let mut manager = PARAM_MANAGER.write();
    manager.update_int(kind,slots.clone(),index,value);
}
pub unsafe fn add_float(kind: i32, slots: Vec<i32>,index: (u64,u64),value: f32)
{
    let mut manager = PARAM_MANAGER.write();
    manager.update_float(kind,slots.clone(),index,value);
}


pub struct FighterParamModule {
    pub manager: ParamManager
}

impl FighterParamModule {
    #[export_name = "FighterParamModule__has_kind"]
    pub extern "Rust" fn has_kind(kind: i32) -> bool {
        let mut manager = PARAM_MANAGER.read();
        return manager.kinds.contains(&kind) || manager.has_all;
    }

    #[export_name = "FighterParamModule__get_int_param"]
    pub extern "Rust" fn get_int_param(kind: i32, slot: i32, param_type: u64, param_hash: u64) -> Option<i32> {
        let mut manager = PARAM_MANAGER.read();
        for params in &manager.params {
            if (params.kind == kind) {
                if params.slots.contains(&slot) {
                    if let Some(value) = params.get_int(param_type, param_hash){
                        return Some(value);
                    }
                }
            }
        }
        return None;
    }
    #[export_name = "FighterParamModule__get_float_param"]
    pub extern "Rust" fn get_float_param(kind: i32, slot: i32, param_type: u64, param_hash: u64) -> Option<f32> {
        let mut manager = PARAM_MANAGER.read();
        for params in &manager.params {
            if (params.kind == kind) {
                if params.slots.contains(&slot) {                    
                    if let Some(value) = params.get_float(param_type, param_hash){
                        return Some(value);
                    }      
                }
            }
        }
        return None;
    }
    #[export_name = "FighterParamModule__get_article_use_type"]
    pub extern "Rust" fn get_article_use_type(kind: i32) -> Option<i32> {
        let mut manager = PARAM_MANAGER.read();
        for params in &manager.params {
            if (params.kind == kind) {
                let article_hash = hash_str_to_u64("article_use_type");
                if let Some(value) = params.get_int(article_hash,0){
                    return Some(value);
                }
            }
        }
        return None;
    }
}