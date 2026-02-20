#![crate_name = "param_config"]
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
mod app;

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
    static ref IN_GAME: RwLock<bool> = RwLock::new(false);
    static ref HOOK_ARTICLES: RwLock<bool> = RwLock::new(false);
    static ref HOOK_PARAMS: RwLock<bool> = RwLock::new(false);
    static ref HOOK_KIRBY: RwLock<bool> = RwLock::new(false);
    static ref HOOK_VILLAGER: RwLock<bool> = RwLock::new(false);
    static ref HOOK_ROSETTA: RwLock<bool> = RwLock::new(false);
    static ref IS_HOOKED_ARTICLES: RwLock<bool> = RwLock::new(false);
    static ref IS_HOOKED_PARAMS: RwLock<bool> = RwLock::new(false);
    static ref IS_HOOKED_KIRBY: RwLock<bool> = RwLock::new(false);
    static ref IS_HOOKED_VILLAGER: RwLock<bool> = RwLock::new(false);
    static ref IS_HOOKED_ROSETTA: RwLock<bool> = RwLock::new(false);
    static ref HASH_ANY: RwLock<u64> = RwLock::new(0);
}

pub fn is_in_game() -> bool {
    return *IN_GAME.read();
}
pub fn can_hook_articles() -> bool {
    return *HOOK_ARTICLES.read() && !is_hooked_articles();
}
pub fn can_hook_params() -> bool {
    return *HOOK_PARAMS.read() && !is_hooked_params();
}
pub fn can_hook_kirby() -> bool {
    return *HOOK_KIRBY.read() && !is_hooked_kirby();
}
pub fn can_hook_villager() -> bool {
    return *HOOK_VILLAGER.read() && !is_hooked_villager();
}
pub fn can_hook_rosetta() -> bool {
    return *HOOK_ROSETTA.read() && !is_hooked_rosetta();
}
pub fn is_hooked_articles() -> bool {
    return *IS_HOOKED_ARTICLES.read();
}
pub fn is_hooked_params() -> bool {
    return *IS_HOOKED_PARAMS.read();
}
pub fn is_hooked_kirby() -> bool {
    return *IS_HOOKED_KIRBY.read();
}
pub fn is_hooked_villager() -> bool {
    return *IS_HOOKED_VILLAGER.read();
}
pub fn is_hooked_rosetta() -> bool {
    return *IS_HOOKED_ROSETTA.read();
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

    #[export_name = "FighterParamModule__get_kirby_inhale_behavior"]
    pub extern "C" fn get_kirby_inhale_behavior(kind: i32, slot: i32, weapon_kind: i32) -> Option<i32> {
        let mut manager = PARAM_MANAGER.read();
        for params in &manager.params {
            if (params.kind == kind) {
                let target_hash = hash_str_to_u64("kirby_inhale_behavior");
                if let Some(value) = params.get_int(target_hash,weapon_kind.abs() as u64){
                    return Some(value);
                }
                else if let Some(value) = params.get_int(target_hash,0){
                    return Some(value);
                }
            }
        }
        return None;
    }

    #[export_name = "FighterParamModule__get_villager_pocket_behavior"]
    pub extern "C" fn get_villager_pocket_behavior(kind: i32, slot: i32, weapon_kind: i32) -> Option<i32> {
        let mut manager = PARAM_MANAGER.read();
        for params in &manager.params {
            if (params.kind == kind) {
                let param_hash = hash_str_to_u64("villager_pocket_behavior");
                if let Some(value) = params.get_int(param_hash,weapon_kind.abs() as u64){
                    return Some(value);
                }
                else if let Some(value) = params.get_int(param_hash,0){
                    return Some(value);
                }
                let dep_hash = hash_str_to_u64("villager_cant_pocket");
                if let Some(value) = params.get_int(dep_hash,weapon_kind.abs() as u64) {
                    return Some(POCKET_BEHAVIOR_MISFIRE);
                }
                else if let Some(value) = params.get_int(dep_hash,0) {
                    return Some(POCKET_BEHAVIOR_MISFIRE);
                }
            }
        }
        return None;
    }

    #[export_name = "FighterParamModule__get_rosetta_pull_behavior"]
    pub extern "C" fn get_rosetta_pull_behavior(kind: i32, slot: i32, weapon_kind: i32) -> Option<i32> {
        let mut manager = PARAM_MANAGER.read();
        for params in &manager.params {
            if (params.kind == kind || params.kind == *FIGHTER_KIND_ALL) {
                if params.slots.contains(&slot) || params.has_all_slots {
                    println!("has dude");
                    let param_hash = hash_str_to_u64("rosetta_pull_behavior");
                    if let Some(value) = params.get_int(param_hash,weapon_kind.abs() as u64) {
                        return Some(value);
                    }
                    else if let Some(value) = params.get_int(param_hash,0) {
                        return Some(value);
                    }
                }
            }
        }
        return None;
    }
}


/// Inhale/Pocket/Pull will do whatever they normal do when interacting with this weapon
pub const POCKET_BEHAVIOR_ORIGINAL: i32 = 0x0;

/// Inhale/Pocket/Pull will ignore this weapon
pub const POCKET_BEHAVIOR_IGNORE: i32 = 0x1;

/// Pocket will delete the weapon, Villager won't change statuses
pub const POCKET_BEHAVIOR_DELETE: i32 = 0x2;

/// Pocket will delete the weapon, but Villager will go into their Missed state
pub const POCKET_BEHAVIOR_MISFIRE: i32 = 0x3;

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
    else if index.0 == hash40("kirby_cant_copy")
    || index.0 == hash40("kirby_inhale_behavior") {
        *HOOK_KIRBY.write() = true;
        hook::install_kirby();
    }
    else if index.0 == hash40("villager_pocket_behavior")
    || index.0 == hash40("villager_cant_pocket") {
        *HOOK_VILLAGER.write() = true;
        hook::install_villager();
    }
    else if index.0 == hash40("rosetta_pull_behavior"){
        *HOOK_ROSETTA.write() = true;
        hook::install_rosetta();
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
/// // Change Doc's entrance article use type
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
/// Determines what Kirby will do when attempting to inhale this weapon.
/// Dedede, Wario, and Kirby's relevant copy abilities won't be affected
///
/// # Arguments
///
/// * `kind` - Fighter kind, as commonly used like *FIGHTER_KIND_MARIOD.
/// * `slots` - Array of effected slots
/// * `weapon_kind` - Weapon kind. If this is 0, then all weapons spawned from kind/slots will be accounted for
/// * `behavior` - param_config:POCKET_BEHAVIOR const
///
/// # Example
///
/// ```
/// // Prevent Kirby from pocketing Dr Mario's first alt's Pill
/// let slots = vec![1];
/// param_config::set_kirby_inhale_behavior(*FIGHTER_KIND_MARIOD, slots.clone(), 
/// *WEAPON_KIND_MARIOD_DRCAPSULE,param_config::POCKET_BEHAVIOR_MISFIRE);
/// ```
pub extern "C" fn set_kirby_inhale_behavior(kind: i32, slots: Vec<i32>, weapon_kind: i32, behavior: i32)
{
    update_int(kind,slots,(hash40("kirby_inhale_behavior"),weapon_kind.abs() as u64),behavior);
}

#[no_mangle]
/// Determines what Villager will do when attempting to pocket this weapon. This also affects Kirby's Villager/Isabelle ability
///
/// # Arguments
///
/// * `kind` - Fighter kind, as commonly used like *FIGHTER_KIND_MARIOD.
/// * `slots` - Array of effected slots
/// * `weapon_kind` - Weapon kind. If this is 0, then all weapons spawned from kind/slots will be accounted for
/// * `behavior` - param_config:POCKET_BEHAVIOR const
///
/// # Example
///
/// ```
/// // Prevent Villager from pocketing Dr Mario's first alt's Pill
/// let slots = vec![1];
/// param_config::set_villager_pocket_behavior(*FIGHTER_KIND_MARIOD, slots.clone(), 
/// *WEAPON_KIND_MARIOD_DRCAPSULE,param_config::POCKET_BEHAVIOR_MISFIRE);
/// ```
pub extern "C" fn set_villager_pocket_behavior(kind: i32, slots: Vec<i32>, weapon_kind: i32, behavior: i32)
{
    update_int(kind,slots,(hash40("villager_pocket_behavior"),weapon_kind.abs() as u64),behavior);
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
    set_villager_pocket_behavior(kind,slots,weapon_kind,POCKET_BEHAVIOR_MISFIRE);
}

#[no_mangle]
/// Prevents Rosalina from using Down Special (Gravitational Pull) on a weapon if it spawned from a given fighter kind and slot
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
/// // Prevent Rosa from pulling Dr Mario's first alt's Pill
/// let slots = vec![1];
/// param_config::set_rosetta_pull_behavior(*FIGHTER_KIND_MARIOD, slots.clone(), 
/// *WEAPON_KIND_MARIOD_DRCAPSULE,param_config::POCKET_BEHAVIOR_MISFIRE);
/// ```
pub extern "C" fn set_rosetta_pull_behavior(kind: i32, slots: Vec<i32>, weapon_kind: i32, behavior: i32)
{
    update_int(kind,slots,(hash40("rosetta_pull_behavior"),weapon_kind as u64),behavior);
}

/// This flag is true if Rosalina has pulled the object
///
/// # Example
///
/// ```
/// // Delete if pulled
/// if WorkModule::is_flag(weapon.module_accessor, param_config::WEAPON_INSTANCE_WORK_ID_FLAG_ROSETTA_PULLED) {
/// smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
/// }
/// 
/// ```
pub const WEAPON_INSTANCE_WORK_ID_FLAG_ROSETTA_PULLED: i32 = 0x20000FFF;