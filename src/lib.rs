#![crate_name = "param_config"]
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
use lazy_static::lazy_static;
mod hook;

pub fn hash_str_to_u64(param: &str) -> u64
{
    if param.starts_with("0x"){
        match u64::from_str_radix(param.trim_start_matches("0x"), 16){
            Ok(hex) => return hex,
            Err(err) => {println!("[libparam_config::nro::data] Failed to parse {}",param); return 0}
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

lazy_static! {
    static ref HOOK_ARTICLES: RwLock<bool> = RwLock::new(false);
    static ref HOOK_PARAMS: RwLock<bool> = RwLock::new(false);
    static ref HOOK_KIRBY: RwLock<bool> = RwLock::new(false);
    static ref HOOK_VILLAGER: RwLock<bool> = RwLock::new(false);
    static ref IS_HOOKED_ARTICLES: RwLock<bool> = RwLock::new(false);
    static ref IS_HOOKED_PARAMS: RwLock<bool> = RwLock::new(false);
    static ref IS_HOOKED_KIRBY: RwLock<bool> = RwLock::new(false);
    static ref IS_HOOKED_VILLAGER: RwLock<bool> = RwLock::new(false);
    static ref HASH_ANY: RwLock<u64> = RwLock::new(0);
}

pub fn can_Hook_Articles() -> bool {
    return *HOOK_ARTICLES.read() && !is_Hooked_Articles();
}
pub fn can_Hook_Params() -> bool {
    return *HOOK_PARAMS.read() && !is_Hooked_Params();
}
pub fn can_Hook_Kirby() -> bool {
    return *HOOK_KIRBY.read() && !is_Hooked_Kirby();
}
pub fn can_Hook_Villager() -> bool {
    return *HOOK_VILLAGER.read() && !is_Hooked_Villager();
}
pub fn is_Hooked_Articles() -> bool {
    return *IS_HOOKED_ARTICLES.read();
}
pub fn is_Hooked_Params() -> bool {
    return *IS_HOOKED_PARAMS.read();
}
pub fn is_Hooked_Kirby() -> bool {
    return *IS_HOOKED_KIRBY.read();
}
pub fn is_Hooked_Villager() -> bool {
    return *IS_HOOKED_VILLAGER.read();
}
pub fn set_hash_any() {
    if *HASH_ANY.read() == 0 {
        *HASH_ANY.write() = hash_str_to_u64("any");
    }
}

pub struct CharacterParam {
    pub kind: i32,
    pub has_all_slots: bool,
    pub slots: Vec<i32>,
    pub ints: HashMap<(u64,u64),i32>,
    pub floats: HashMap<(u64,u64),f32>,
    pub attribute_muls: HashMap<(u64,u64),f32>,
    pub mul_ints: HashMap<(u64,u64),f32>,
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
        else if let Some(value) = self.ints.get(&(*HASH_ANY.read(),param_hash)){
            return Some(*value);
        }
        return None;
    }
    pub fn get_float(&self, param_type: u64, param_hash: u64) -> Option<f32> {
        if let Some(value) = self.floats.get(&(param_type,param_hash)){
            return Some(*value);
        }
        else if let Some(value) = self.floats.get(&(*HASH_ANY.read(),param_hash)){
            return Some(*value);
        }
        return None;
    }
    pub fn get_attribute_mul(&self, param_type: u64, param_hash: u64) -> Option<f32> {
        if let Some(value) = self.attribute_muls.get(&(param_type,param_hash)){
            return Some(*value);
        }
        else if let Some(value) = self.attribute_muls.get(&(*HASH_ANY.read(),param_hash)){
            return Some(*value);
        }
        return None;
    }
    pub fn get_int_param_mul(&self, param_type: u64, param_hash: u64) -> Option<f32> {
        if let Some(value) = self.mul_ints.get(&(param_type,param_hash)){
            return Some(*value);
        }
        else if let Some(value) = self.mul_ints.get(&(*HASH_ANY.read(),param_hash)){
            return Some(*value);
        }
        return None;
    }
}

pub struct ParamManager {
    pub kinds: Vec<i32>,
    pub has_all: bool,
    pub params: Vec<CharacterParam>
}

pub const PARAM_TYPE_INT: i32 = 0;
pub const PARAM_TYPE_FLOAT: i32 = 1;
pub const PARAM_TYPE_ATTR_MUL: i32 = 2;
pub const PARAM_TYPE_INT_MUL: i32 = 3;

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
            if kind == *FIGHTER_KIND_ALL {
                self.has_all = true;
            }
        }
        self.params.push(params);
    }
    
    pub fn get_param_by_slot(&self,kind: i32, slot: i32) -> Option<&CharacterParam> {
        for params in &self.params{
            if (params.kind == kind) {
                if params.slots.contains(&slot) || params.has_all_slots {
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

    fn update_value(&mut self,kind: i32, slots: Vec<i32>,index: (u64,u64),value_i: i32, value_f: f32,value_type: i32) {
        for param in &mut self.params {
            if (param.kind == kind) {
                if param.slots == slots {
                    match value_type {
                        PARAM_TYPE_FLOAT => {param.floats.insert(index, value_f);}
                        PARAM_TYPE_ATTR_MUL => {param.attribute_muls.insert(index, value_f);}
                        PARAM_TYPE_INT_MUL => {param.mul_ints.insert(index, value_f);}
                        _ => {param.ints.insert(index, value_i);}
                    }
                    
                    return;
                }
            }
        }
        let mut newparams = CharacterParam {
            kind: kind,
            has_all_slots: (slots.contains(&-1)),
            slots: slots,
            ints: HashMap::new(),
            floats: HashMap::new(),
            attribute_muls: HashMap::new(),
            mul_ints: HashMap::new(),
        };
        match value_type {
            PARAM_TYPE_FLOAT => {newparams.floats.insert(index, value_f);}
            PARAM_TYPE_ATTR_MUL => {newparams.attribute_muls.insert(index, value_f);}
            PARAM_TYPE_INT_MUL => {newparams.mul_ints.insert(index, value_f);}
            _ => {newparams.ints.insert(index,value_i);}
        }
        self.push(newparams);
    }
    pub fn update_int(&mut self,kind: i32, slots: Vec<i32>,index: (u64,u64),value: i32) {
        self.update_value(kind,slots,index,value,0.0,PARAM_TYPE_INT);
    }
    pub fn update_float(&mut self,kind: i32, slots: Vec<i32>,index: (u64,u64),value: f32) {
        self.update_value(kind,slots,index,0,value,PARAM_TYPE_FLOAT);
    }
    pub fn update_attribute_mul(&mut self,kind: i32, slots: Vec<i32>,index: (u64,u64),value: f32) {
        self.update_value(kind,slots,index,0,value,PARAM_TYPE_ATTR_MUL);
    }
    pub fn update_int_mul(&mut self,kind: i32, slots: Vec<i32>,index: (u64,u64),value: f32) {
        self.update_value(kind,slots,index,0,value,PARAM_TYPE_INT_MUL);
    }
    
}

lazy_static! {
    pub static ref PARAM_MANAGER: RwLock<ParamManager> = RwLock::new(ParamManager::new());
}

pub struct FighterParamModule {
    pub manager: ParamManager
}

impl FighterParamModule {
    #[export_name = "FighterParamModule__has_kind"]
    pub extern "C" fn has_kind(kind: i32) -> bool {
        let mut manager = PARAM_MANAGER.read();
        return manager.kinds.contains(&kind) || manager.has_all;
    }

    #[export_name = "FighterParamModule__get_int_param"]
    pub extern "C" fn get_int_param(kind: i32, slot: i32, param_type: u64, param_hash: u64) -> Option<i32> {
        let mut manager = PARAM_MANAGER.read();
        for params in &manager.params {
            if (params.kind == kind || params.kind == *FIGHTER_KIND_ALL) {
                if params.slots.contains(&slot) || params.has_all_slots {
                    if let Some(value) = params.get_int(param_type, param_hash){
                        return Some(value);
                    }
                }
            }
        }
        return None;
    }
    #[export_name = "FighterParamModule__get_float_param"]
    pub extern "C" fn get_float_param(kind: i32, slot: i32, param_type: u64, param_hash: u64) -> Option<f32> {
        let mut manager = PARAM_MANAGER.read();
        for params in &manager.params {
            if (params.kind == kind || params.kind == *FIGHTER_KIND_ALL) {
                if params.slots.contains(&slot) || params.has_all_slots {  
                    if let Some(value) = params.get_float(param_type, param_hash){
                        return Some(value);
                    }      
                }
            }
        }
        return None;
    }
    #[export_name = "FighterParamModule__get_attribute_mul"]
    pub extern "C" fn get_attribute_mul(kind: i32, slot: i32, param_type: u64, param_hash: u64) -> Option<f32> {
        let mut manager = PARAM_MANAGER.read();
        for params in &manager.params {
            if (params.kind == kind || params.kind == *FIGHTER_KIND_ALL) {
                if params.slots.contains(&slot) || params.has_all_slots {  
                    if let Some(value) = params.get_attribute_mul(param_type, param_hash){
                        return Some(value);
                    }      
                }
            }
        }
        return None;
    }
    #[export_name = "FighterParamModule__get_int_param_mul"]
    pub extern "C" fn get_int_param_mul(kind: i32, slot: i32, param_type: u64, param_hash: u64) -> Option<f32> {
        let mut manager = PARAM_MANAGER.read();
        for params in &manager.params {
            if (params.kind == kind || params.kind == *FIGHTER_KIND_ALL) {
                if params.slots.contains(&slot) || params.has_all_slots {  
                    if let Some(value) = params.get_int_param_mul(param_type, param_hash){
                        return Some(value);
                    }      
                }
            }
        }
        return None;
    }
    #[export_name = "FighterParamModule__get_article_use_type"]
    pub extern "C" fn get_article_use_type(kind: i32) -> Option<i32> {
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

    #[export_name = "FighterParamModule__can_kirby_copy"]
    pub extern "C" fn can_kirby_copy(kind: i32, slot: i32) -> bool {
        let mut manager = PARAM_MANAGER.read();
        for params in &manager.params {
            if (params.kind == kind || params.kind == *FIGHTER_KIND_ALL) {
                if params.slots.contains(&slot) || params.has_all_slots {
                    let article_hash = hash_str_to_u64("kirby_cant_copy");
                    if let Some(value) = params.get_int(article_hash,0){
                        return false;
                    }
                }
            }
        }
        return true;
    }

    #[export_name = "FighterParamModule__can_villager_pocket"]
    pub extern "C" fn can_villager_pocket(kind: i32, slot: i32, weapon_kind: i32) -> bool {
        let mut manager = PARAM_MANAGER.read();
        for params in &manager.params {
            if (params.kind == kind || params.kind == *FIGHTER_KIND_ALL) {
                if params.slots.contains(&slot) || params.has_all_slots {
                    let article_hash = hash_str_to_u64("villager_cant_pocket");
                    if let Some(value) = params.get_int(article_hash,weapon_kind.abs() as u64) {
                        return false;
                    }
                    else if let Some(value) = params.get_int(article_hash,0) {
                        return false;
                    }
                }
            }
        }
        return true;
    }
}

#[no_mangle]
/// Updates (or creates) a new param value based on fighter/weapon kind and current alternate costume (slot)
///
/// # Arguments
///
/// * `kind` - Fighter/Weapon kind, as commonly used like *FIGHTER_KIND_MARIOD. If it's a weapon, use a negative number.
/// * `slots` - Array of effected slots
/// * `index` - (hash40(""),hash40("")) for param/subparam hashes. For common params, the second argument should be 0.
/// * `value` - Value for the param
///
/// # Example
///
/// ```
/// // remove doc's walljump on slot 1
/// let slots = vec![1];
/// let param = (hash40("wall_jump_type"),0 as u64);
/// param_config::update_int(*FIGHTER_KIND_MARIOD, slots.clone(), param, 0);
/// ```
pub extern "C" fn update_int(kind: i32, slots: Vec<i32>,index: (u64,u64),value: i32)
{
    let mut manager = PARAM_MANAGER.write();
    manager.update_int(kind,slots.clone(),index,value);

    if index.0 == hash40("article_use_type"){
        *HOOK_ARTICLES.write() = true;
        hook::install_articles();
    }
    else if index.0 == hash40("kirby_cant_copy"){
        *HOOK_KIRBY.write() = true;
        hook::install_kirby();
    }
    else if index.0 == hash40("villager_cant_pocket"){
        *HOOK_VILLAGER.write() = true;
        hook::install_villager();
    }
    else {
        *HOOK_PARAMS.write() = true;
        hook::install_params();
    }
}

#[no_mangle]
/// Updates (or creates) a new param value based on fighter/weapon kind and current alternate costume (slot)
///
/// # Arguments
///
/// * `kind` - Fighter/Weapon kind, as commonly used like *FIGHTER_KIND_MARIOD. If it's a weapon, use a negative number.
/// * `slots` - Array of effected slots
/// * `param` - (hash40(""),hash40(""),i32) for param/subparam hashes and values. For common params, the second argument should be 0.
///
/// # Example
///
/// ```
/// // remove doc's walljump on slot 1
/// let slots = vec![1];
/// let param = (hash40("wall_jump_type"),0 as u64,0);
/// param_config::update_int_2(*FIGHTER_KIND_MARIOD, slots.clone(), param);
/// ```
pub extern "C" fn update_int_2(kind: i32, slots: Vec<i32>,param: (u64,u64,i32))
{
    update_int(kind,slots,(param.0,param.1),param.2);
}

#[no_mangle]
/// Updates (or creates) a new param value based on fighter/weapon kind and current alternate costume (slot)
/// Recommended to only do this for vl.prc entries, and not fighter attributes (see update_attribute_mul)
///
/// # Arguments
///
/// * `kind` - Fighter/Weapon kind, as commonly used like *FIGHTER_KIND_MARIOD. If it's a weapon, use a negative number.
/// * `slots` - Array of effected slots
/// * `index` - (hash40(""),hash40("")) for param/subparam hashes. For common params, the second argument should be 0.
/// * `value` - Value for the param
///
/// # Example
///
/// ```
/// // Set Doc's Down Special Buoyancy to 3.0 on slot 1
/// let slots = vec![1];
/// let param = (hash40("param_special_lw"),hash40("buoyancy"));
/// param_config::update_float(*FIGHTER_KIND_MARIOD, slots.clone(), param, 3.0);
/// ```
pub extern "C" fn update_float(kind: i32, slots: Vec<i32>,index: (u64,u64),value: f32)
{
    let mut manager = PARAM_MANAGER.write();
    manager.update_float(kind,slots,index,value);
    *HOOK_PARAMS.write() = true;
    hook::install_params();
}

#[no_mangle]
/// Updates (or creates) a new param value based on fighter/weapon kind and current alternate costume (slot)
/// Recommended to only do this for vl.prc entries, and not fighter attributes (see update_attribute_mul)
/// 
/// # Arguments
///
/// * `kind` - Fighter/Weapon kind, as commonly used like *FIGHTER_KIND_MARIOD. If it's a weapon, use a negative number.
/// * `slots` - Array of effected slots
/// * `param` - (hash40(""),hash40(""),f32) for param/subparam hashes and value. For common params, the second argument should be 0.
///
///
/// # Example
///
/// ```
/// // Set Doc's Down Special Buoyancy to 3.0 on slot 1
/// let slots = vec![1];
/// let param = (hash40("param_special_lw"),hash40("buoyancy"), 3.0);
/// param_config::update_float_2(*FIGHTER_KIND_MARIOD, slots.clone(), param);
/// ```
pub extern "C" fn update_float_2(kind: i32, slots: Vec<i32>,param: (u64,u64,f32))
{
    update_float(kind,slots,(param.0,param.1),param.2);
}

#[no_mangle]
/// Updates (or creates) an attribute multiplier based on fighter/weapon kind and current alternate costume (slot)
///
/// # Arguments
///
/// * `kind` - Fighter/Weapon kind, as commonly used like *FIGHTER_KIND_MARIOD. If it's a weapon, use a negative number.
/// * `slots` - Array of effected slots
/// * `index` - (hash40(""),hash40("")) for param/subparam hashes. For common params, the second argument should be 0.
/// * `value` - Value for the param
///
/// # Example
///
/// ```
/// // let doc run twice as fast on slot 1
/// let slots = vec![1];
/// let param = (hash40("run_speed_max"),0 as u64);
/// param_config::update_attribute_mul(*FIGHTER_KIND_MARIOD, slots.clone(), param, 2.0);
/// ```
pub extern "C" fn update_attribute_mul(kind: i32, slots: Vec<i32>,index: (u64,u64),value: f32)
{
    let mut manager = PARAM_MANAGER.write();
    manager.update_attribute_mul(kind,slots,index,value);
    *HOOK_PARAMS.write() = true;
    hook::install_params();
}

#[no_mangle]
/// Updates (or creates) an attribute multiplier based on fighter/weapon kind and current alternate costume (slot)
///
/// # Arguments
///
/// * `kind` - Fighter/Weapon kind, as commonly used like *FIGHTER_KIND_MARIOD. If it's a weapon, use a negative number.
/// * `slots` - Array of effected slots
/// * `param` - (hash40(""),hash40(""),f32) for param/subparam hashes and value. For common params, the second argument should be 0.
///
///
/// # Example
///
/// ```
/// // let doc run twice as fast on slot 1
/// let slots = vec![1];
/// let param = (hash40("run_speed_max"),0 as u64, 2.0);
/// param_config::update_attribute_mul_2(*FIGHTER_KIND_MARIOD, slots.clone(), param);
/// ```
pub extern "C" fn update_attribute_mul_2(kind: i32, slots: Vec<i32>,param: (u64,u64,f32))
{
    update_attribute_mul(kind,slots,(param.0,param.1),param.2);
}

pub extern "C" fn update_int_mul(kind: i32, slots: Vec<i32>,index: (u64,u64),value: f32)
{
    let mut manager = PARAM_MANAGER.write();
    manager.update_int_mul(kind,slots,index,value);
    *HOOK_PARAMS.write() = true;
    hook::install_params();
}
pub extern "C" fn update_int_mul_2(kind: i32, slots: Vec<i32>,param: (u64,u64,f32))
{
    update_int_mul(kind,slots,(param.0,param.1),param.2);
}
#[no_mangle]
/// Changes the article use type, potentially allowing the article to be spawned during different game states
///
/// # Arguments
///
/// * `kind` - Weapon Kind
/// * `use_type` - i32 value of desired *ARTICLE_USETYPE const
///
/// # Example
///
/// ```
/// // Prevent Kirby from copying Dr Mario's first alt
/// let slots = vec![1];
/// param_config::set_article_use_type(*WEAPON_KIND_MARIOD_CAPSULEBLOCK, *ARTICLE_USETYPE_FINAL);
/// ```
pub extern "C" fn set_article_use_type(kind: i32, use_type: i32)
{
    update_int(-(kind.abs()),vec![1],(hash40("article_use_type"),0),use_type);
}
#[no_mangle]
/// Prevents Kirby from copying the ability of a fighter kind and slot
///
/// # Arguments
///
/// * `kind` - Fighter kind, as commonly used like *FIGHTER_KIND_MARIOD.
/// * `slots` - Array of effected slots
///
/// # Example
///
/// ```
/// // Prevent Kirby from copying Dr Mario's first alt
/// let slots = vec![1];
/// param_config::disable_kirby_copy(*FIGHTER_KIND_MARIOD, slots.clone());
/// ```
pub extern "C" fn disable_kirby_copy(kind: i32, slots: Vec<i32>)
{
    update_int(kind,slots,(hash40("kirby_cant_copy"),0),0);
}

#[no_mangle]
/// Prevents Villager/Isabelle from pocketting a weapon if it spawned from a given fighter kind and slot
///
/// # Arguments
///
/// * `kind` - Fighter kind, as commonly used like *FIGHTER_KIND_MARIOD.
/// * `slots` - Array of effected slots
/// * `weapon_kind` - Weapon kind. If this is 0, then all weapons spawned from kind/slots will be accounted for
///
/// # Example
///
/// ```
/// // Prevent Villager from pocketing Dr Mario's first alt's Pill
/// let slots = vec![1];
/// param_config::disable_villager_pocket(*FIGHTER_KIND_MARIOD, slots.clone(), *WEAPON_KIND_MARIOD_DRCAPSULE);
/// ```
pub extern "C" fn disable_villager_pocket(kind: i32, slots: Vec<i32>, weapon_kind: i32)
{
    update_int(kind,slots,(hash40("villager_cant_pocket"),weapon_kind.abs() as u64),0);
}