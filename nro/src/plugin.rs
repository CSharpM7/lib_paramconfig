#![feature(proc_macro_hygiene)]
use super::*;
use super::data::*;
use std::{
    io,
    env,
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
    param_float: Option::<Vec<param_float>>
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

pub unsafe fn read_config(config_file: String) -> bool
{
    let mut hasContent = false;
    let contents = match fs::read_to_string(config_file.as_str()) {
        Ok(c) => c,
        Err(_) => {
            println!("[libparam_config::nro::data] `{}`", config_file.as_str());
            return false;
        }
    };
    let data: ConfigToml = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            println!("[libparam_config::nro::data] Unable to load data from `{}`", config_file.as_str());
            return false;
        }
    };
    println!("[libparam_config::nro::data] Found file: {}",config_file.as_str());
    println!("[libparam_config::nro::data] Loading params:");
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
        for param in data.param_int.unwrap() {
            let mut kinds:Vec<String> = Vec::new();
            let mut slots:Vec<i32> = Vec::new();
            if param.kinds.is_some() {
                kinds = param.kinds.unwrap();
            }
            else if mainKind != "" {
                kinds.push(mainKind.clone());
            }
            if kinds.len() < 1 {
                println!("[libparam_config::nro::data] Entry has no fighters");
                continue;
            }

            if param.slots.is_some(){
                slots = param.slots.unwrap();
            }
            else if mainSlots.len() > 0 {
                slots = mainSlots.clone();
            }
            if slots.len() < 1 {
                println!("[libparam_config::nro::data] Entry has no slots");
                continue;
            }

            let subparam = 0;
            let mut subparam_string = match param.subparam {
                Some(h) => h,
                None => String::from("")
            };
            if param.param == "article_use_type" {
                subparam_string = String::from("");
            }
            else if param.param == "kirby_cant_copy" {
                subparam_string = String::from("");
            }

            let subparam_str = subparam_string.as_str();
            let mut subparam_hash = 0;
            if param.param == "villager_cant_pocket" {
                if hash_str_to_u64(&subparam_string) != 0 {
                    subparam_hash = (get_weapon_kind_from_string(&subparam_string).abs()) as u64;
                    if subparam_hash == 999 {
                        println!("[libparam_config::nro::data] {} is an invalid weapon",&subparam_string);
                        continue;
                    }
                }
            } 
            else {
                subparam_hash = hash_str_to_u64(subparam_str);
            };
            let index = (hash_str_to_u64(param.param.as_str()),subparam_hash);

            let mut validKinds = false;
            for kind in &kinds {
                let isFighter = !(kind.contains("_") && kind != "ice_climber");
                let kind_i32 = if isFighter {get_fighter_kind_from_string(&kind)} else {get_weapon_kind_from_string(&kind)};
                if kind_i32 == 999 {
                    println!("[libparam_config::nro::data] {} is an invalid fighter",kind);
                    continue;
                }
                if kind_i32 == -999 {
                    println!("[libparam_config::nro::data] {} is an invalid weapon",kind);
                    continue;
                }
                validKinds = true;
                param_config::update_int(kind_i32,slots.clone(),index,param.value);
                //manager.update_int(kind_i32,slots.clone(),index,param.value);
            }
            print!("[");
            for kind in &kinds {
                print!("{},",kind.as_str());
            }
            if !validKinds {continue;}
            hasContent = true;

            if param.param == "article_use_type" {
                print!("] article use type: {}",param.value);
            }
            else if param.param == "kirby_cant_copy" {
                print!("] kirby cant copy");
            }
            else if param.param == "villager_cant_pocket" {
                print!("{}",format!("] villager cant pocket: {} ({})",subparam_string,index.1));
            }
            else{
                print!("(");
                for slot in slots {
                    print!("{slot},");
                }
                print!(")] {}({}): {}",param.param,subparam_str,param.value);
            }
            println!("");
        } 
    }
    if data.param_float.is_some(){
        for param in data.param_float.unwrap() {
            let mut kinds:Vec<String> = Vec::new();
            let mut slots:Vec<i32> = Vec::new();
            if param.kinds.is_some() {
                kinds = param.kinds.unwrap();
            }
            else if mainKind != "" {
                kinds.push(mainKind.clone());
            }
            if kinds.len() < 1 {
                println!("[libparam_config::nro::data] Entry has no fighters");
                continue;
            }

            if param.slots.is_some(){
                slots = param.slots.unwrap();
            }
            else if mainSlots.len() > 0 {
                slots = mainSlots.clone();
            }
            if slots.len() < 1 {
                println!("[libparam_config::nro::data] Entry has no slots");
                continue;
            }

            let subparam = 0;
            let subparam_string = match param.subparam {
                Some(h) => h,
                None => String::from("")
            };
            let subparam_str = subparam_string.as_str();

            let index = (hash_str_to_u64(param.param.as_str()),hash_str_to_u64(subparam_str));

            let mut validKinds = false;
            for kind in &kinds {
                let isFighter = !(kind.contains("_") && kind != "ice_climber");
                let kind_i32 = if isFighter {get_fighter_kind_from_string(&kind)} else {get_weapon_kind_from_string(&kind)};
                if kind_i32 == 999 {
                    println!("[libparam_config::nro::data] {} is an invalid fighter",kind);
                    continue;
                }
                if kind_i32 == -999 {
                    println!("[libparam_config::nro::data] {} is an invalid weapon",kind);
                    continue;
                }
                validKinds = true;
                param_config::update_float(kind_i32,slots.clone(),index,param.value);
            }
            print!("[");
            for kind in &kinds {
                print!("{},",kind.as_str());
            }
            if !validKinds {continue;}
            hasContent = true;

            print!("(");
            for slot in slots {
                print!("{slot},");
            }
            print!(")] {}({}): {}",param.param,subparam_str,param.value);
            println!("");
        }  
    }
    #[cfg(not(feature = "switch"))] 
    println!("[libparam_config::nro::data] Finished!");
    
    return hasContent;
}

#[cfg(feature = "switch")] 
use arcropolis_api;
pub fn find_folders() ->bool {
    #[cfg(feature = "switch")] 
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