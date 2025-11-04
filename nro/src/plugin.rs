#![feature(proc_macro_hygiene)]
use super::*;
use super::data::*;
use std::{
    path::Path,
    convert::TryInto,
    str::FromStr,
    fs,
    thread::{self},
    sync::Mutex
};

use once_cell::sync::Lazy;

use serde_derive::Deserialize;
use toml;

const IDENTIFIER: &str = "config_param.toml";

// Top level struct to hold the TOML data.
#[derive(Deserialize)]
struct ConfigToml {
    kind: Option::<String>,
    slots: Option::<Vec<i32>>,
    param_int: Option::<Vec<param_int>>,
    param_float: Option::<Vec<param_float>>,
    attribute_mul: Option::<Vec<attribute_mul>>,
    param_int_mul: Option::<Vec<param_int_mul>>,
}

#[derive(Deserialize)]
struct param_int {
    param: String,
    subparam: Option::<String>,
    value: i32,
    kinds: Option::<Vec<String>>,
    slots: Option::<Vec<i32>>,
}
#[derive(Deserialize)]
struct param_float {
    param: String,
    subparam: Option::<String>,
    value: f32,
    kinds: Option::<Vec<String>>,
    slots: Option::<Vec<i32>>,
}
#[derive(Deserialize)]
struct attribute_mul {
    param: String,
    subparam: Option::<String>,
    value: f32,
    kinds: Option::<Vec<String>>,
    slots: Option::<Vec<i32>>,
}
#[derive(Deserialize)]
struct param_int_mul {
    param: String,
    subparam: Option::<String>,
    value: f32,
    kinds: Option::<Vec<String>>,
    slots: Option::<Vec<i32>>,
}

pub unsafe fn update_param(
    mainKind: &String, mainSlots: &Vec<i32>,
    p_param: String, p_subparam: Option::<String>,
    p_value_i: i32,p_value_f: f32,p_value_type: i32,
    p_kinds: Option::<Vec<String>>,p_slots: Option::<Vec<i32>>,
) -> bool
{
    let mut kinds:Vec<String> = Vec::new();
    let mut slots:Vec<i32> = Vec::new();
    if p_kinds.is_some() {
        kinds = p_kinds.unwrap();
    }
    else if mainKind != "" {
        kinds.push(mainKind.clone());
    }
    if kinds.len() < 1 {
        println!("[libparam_config::nro::plugin] Entry has no fighters");
        return false;
    }

    if p_slots.is_some(){
        slots = p_slots.unwrap();
    }
    else if mainSlots.len() > 0 {
        slots = mainSlots.clone();
    }
    if slots.len() < 1 {
        println!("[libparam_config::nro::plugin] Entry has no slots");
        return false;
    }

    let subparam = 0;
    let mut subparam_string = match p_subparam {
        Some(h) => h,
        None => String::from("")
    };
    if p_value_type == param_config::PARAM_TYPE_INT {
        if p_param == "article_use_type" {
            subparam_string = String::from("");
        }
        else if p_param == "kirby_cant_copy" {
            subparam_string = String::from("");
        }
    }

    let subparam_str = subparam_string.as_str();
    let mut subparam_hash = 0;
    if p_param == "villager_cant_pocket" 
    || p_param == "rosetta_cant_pocket" {
        if hash_str_to_u64(&subparam_string) != 0 {
            subparam_hash = (get_weapon_kind_from_string(&subparam_string).abs()) as u64;
            if subparam_hash == 999 {
                println!("[libparam_config::nro::plugin] {} is an invalid weapon",&subparam_string);
                return false;
            }
        }
    } 
    else {
        subparam_hash = hash_str_to_u64(subparam_str);
    };
    let index = (hash_str_to_u64(p_param.as_str()),subparam_hash);

    let mut validKinds = false;
    let use_int = p_value_type == param_config::PARAM_TYPE_INT;

    for kind in &kinds {
        let isFighter = !(kind.contains("_") && kind != "ice_climber");
        let kind_i32 = if isFighter {get_fighter_kind_from_string(&kind)} else {get_weapon_kind_from_string(&kind)};
        if kind_i32 == 999 {
            println!("[libparam_config::nro::plugin] {} is an invalid fighter",kind);
            continue;
        }
        if kind_i32 == -999 {
            println!("[libparam_config::nro::plugin] {} is an invalid weapon",kind);
            continue;
        }
        validKinds = true;
        match p_value_type {
            param_config::PARAM_TYPE_FLOAT => {param_config::update_float(kind_i32,slots.clone(),index,p_value_f);}
            param_config::PARAM_TYPE_ATTR_MUL => {param_config::update_attribute_mul(kind_i32,slots.clone(),index,p_value_f);}
            param_config::PARAM_TYPE_INT_MUL => {param_config::update_int_mul(kind_i32,slots.clone(),index,p_value_f);}
            _ => {param_config::update_int(kind_i32,slots.clone(),index,p_value_i);}
        }
        
        //manager.update_int(kind_i32,slots.clone(),index,p_value);
    }
    print!("[");
    for kind in &kinds {
        print!("{},",kind.as_str());
    }
    if !validKinds {return false;}

    let p_value_str = if use_int {p_value_i.to_string()} else {p_value_f.to_string()};
    if p_param == "article_use_type" {
        print!("] article use type: {}",p_value_str);
    }
    else if p_param == "kirby_cant_copy" {
        print!("] kirby cant copy");
    }
    else if p_param == "villager_cant_pocket" {
        print!("{}",format!("] villager cant pocket: {} ({})",subparam_string,index.1));
    }
    else if p_param == "rosetta_cant_pull" {
        print!("{}",format!("] Rosalina cant pull: {} ({})",subparam_string,index.1));
    }
    else{
        print!("(");
        for slot in slots {
            print!("{slot},");
        }
        print!(")] {}({}): {}",p_param,subparam_str,p_value_str);
    }
    println!("");
    return true;
}
pub unsafe fn read_config(config_file: String) -> bool
{
    let mut hasContent = false;
    let contents = match fs::read_to_string(config_file.as_str()) {
        Ok(c) => c,
        Err(_) => {
            println!("[libparam_config::nro::plugin] `{}`", config_file.as_str());
            return false;
        }
    };
    let data: ConfigToml = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            println!("[libparam_config::nro::plugin] Unable to load data from `{}`", config_file.as_str());
            return false;
        }
    };
    println!("[libparam_config::nro::plugin] Found file: {}",config_file.as_str());
    println!("[libparam_config::nro::plugin] Loading params:");
    let mut mainKind = String::from("");
    let mut mainSlots = Vec::new();
    if data.kind.is_some(){
        mainKind = data.kind.unwrap();
    }
    if data.slots.is_some(){
        mainSlots = data.slots.unwrap();
    }
    //let mut manager = PARAM_MANAGER.write();
    
    if data.param_int.is_some(){
        println!("[libparam_config::nro::plugin] Ints:");
        for param in data.param_int.unwrap() {
            //println!("{}?",param.param);
            hasContent = update_param(&mainKind,&mainSlots,
                param.param,param.subparam,
                param.value,0.0,param_config::PARAM_TYPE_INT,
                param.kinds,param.slots) || hasContent;
        } 
    }
    if data.param_float.is_some(){
        println!("");
        println!("[libparam_config::nro::plugin] Floats:");
        for param in data.param_float.unwrap() {
            //println!("{}?",param.param);
            hasContent = update_param(&mainKind,&mainSlots,
                param.param,param.subparam,
                0,param.value,param_config::PARAM_TYPE_FLOAT,
                param.kinds,param.slots) || hasContent;
        }  
    }
    if data.attribute_mul.is_some(){
        println!("");
        println!("[libparam_config::nro::plugin] Attribute Muls:");
        for param in data.attribute_mul.unwrap() {
            //println!("{}?",param.param);
            hasContent = update_param(&mainKind,&mainSlots,
                param.param,param.subparam,
                0,param.value,param_config::PARAM_TYPE_ATTR_MUL,
                param.kinds,param.slots) || hasContent;
        }  
    }
    if data.param_int_mul.is_some(){
        println!("");
        println!("[libparam_config::nro::plugin] Int Muls:");
        for param in data.param_int_mul.unwrap() {
            //println!("{}?",param.param);
            hasContent = update_param(&mainKind,&mainSlots,
                param.param,param.subparam,
                0,param.value,param_config::PARAM_TYPE_INT_MUL,
                param.kinds,param.slots) || hasContent;
        }  
    }
    #[cfg(not(feature = "test"))] 
    println!("[libparam_config::nro::plugin] Finished!");
    println!("");
    
    return hasContent;
}

#[cfg(not(feature = "test"))] 
use arcropolis_api;
pub fn find_folders() ->bool {
    #[cfg(not(feature = "test"))]
    unsafe {
        let mut folders_found = false;
        for entry in std::fs::read_dir("sd:/ultimate/mods").unwrap() {
            let entry = entry.unwrap();
            let mut path = entry.path();
            if path.is_dir() {
                path.push(IDENTIFIER);
                if Path::new(&path).exists() {
                    path.pop();

                    let folder = format!("{}",path.display());

                    let is_enabled = arcropolis_api::is_mod_enabled(arcropolis_api::hash40(folder.as_str()).as_u64());
                    if is_enabled {
                        folders_found = folders_found | read_config(format!("{}/{}", folder.as_str(),IDENTIFIER));
                    }
                }
            }
        }
        return folders_found;
    }
    return true;
}

pub fn install() -> bool {

    let folder_thread = std::thread::Builder::new()
    .stack_size(32*512*256)
    .spawn(|| {
        data::build_fighter_table();
        data::build_weapon_table();
        find_folders()
    })
    .unwrap()
    .join();

    return folder_thread.unwrap();
}