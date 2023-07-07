#![feature(proc_macro_hygiene)]
use super::*;
use std::{
    io,
    env,
    path::Path,
    convert::TryInto,
    str::FromStr,
    fs,
    thread::{self}
};
use arcropolis_api;

use lazy_static::lazy_static;

//use std::sync::RwLock;

use serde_derive::Deserialize;
use toml;


pub struct CharacterParam {
    pub kind: i32,
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
        return None;
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
            if kind == -1 {
                self.has_all = true;
            }
        }
        self.params.push(params);
    }
    
    pub fn get_params_by_slot(&self,kind: i32, slot: i32) -> Option<&CharacterParam> {
        for params in &self.params{
            if (params.kind == kind) {
                if params.slots.contains(&slot) {
                    return Some(params);
                }
            }
        }
        return None
    }
    
}

lazy_static! {
    pub static ref PARAM_MANAGER: RwLock<ParamManager> = RwLock::new(ParamManager::new());
}


pub struct FighterParamModule {
    pub manager: ParamManager
}

/// An additional module to be used with Smash's `BattleObject` class. This handles storing and retrieving primitive variables
/// that you want to associate with a specific object (such as associating a gimmick timer with mario or dk)
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
}

const IDENTIFIER: &str = "config_param.toml";

pub fn get_kind_from_string(target_kind: &str) -> i32 {
    let kind_map = HashMap::from([
    ("all","-1"),
    ("sonic","29"),
    ("demon","5c"),
    ("dolly","55"),
    ("link","2"),
    ("miienemys","4f"),
    ("tantan","57"),
    ("diddy","27"),
    ("jack","52"),
    ("sheik","10"),
    ("purin","c"),
    ("simon","43"),
    ("pichu","13"),
    ("koopag","4d"),
    ("reflet","38"),
    ("donkey","1"),
    ("rosetta","33"),
    ("elight","5b"),
    ("plizardon","26"),
    ("base_append_head","51"),
    ("brave","53"),
    ("zelda","11"),
    ("none","ffffffff"),
    ("duckhunt","3b"),
    ("rockman","31"),
    ("samus","3"),
    ("pfushigisou","25"),
    ("cloud","3e"),
    ("packun","51"),
    ("light","74"),
    ("younglink","17"),
    ("wiifit","32"),
    ("buddy","54"),
    ("robot","2d"),
    ("littlemac","34"),
    ("trail","5d"),
    ("samusd","4"),
    ("richter","44"),
    ("pzenigame","24"),
    ("snake","22"),
    ("table_ex_term","76"),
    ("edge","59"),
    ("yoshi","5"),
    ("ganon","18"),
    ("ridley","42"),
    ("ken","3d"),
    ("mewtwo","19"),
    ("wolf","2f"),
    ("roy","1a"),
    ("base_append_num","d"),
    ("falco","14"),
    ("base_head","0"),
    ("toonlink","2e"),
    ("zenigame","6f"),
    ("murabito","30"),
    ("miienemyf","4e"),
    ("captain","b"),
    ("kamui","3f"),
    ("nana","4c"),
    ("szerosuit","20"),
    ("ptrainer","72"),
    ("popo","4b"),
    ("fushigisou","70"),
    ("pikachu","8"),
    ("gekkouga","35"),
    ("other_tail","50"),
    ("lucas","28"),
    ("fox","7"),
    ("palutena","36"),
    ("dedede","2a"),
    ("ice_climber","6e"),
    ("wario","21"),
    ("koopa","f"),
    ("daisy","e"),
    ("flame","73"),
    ("lucina","16"),
    ("mario","0"),
    ("chrom","1b"),
    ("kirby","6"),
    ("pit","1e"),
    ("eflame","5a"),
    ("lucario","2c"),
    ("marth","15"),
    ("master","56"),
    ("metaknight","1d"),
    ("peach","d"),
    ("table_ex_start","6d"),
    ("pitb","1f"),
    ("gamewatch","1c"),
    ("other_num","4"),
    ("luigi","9"),
    ("miifighter","48"),
    ("pacman","37"),
    ("random","77"),
    ("base_append_tail","5d"),
    ("miigunner","4a"),
    ("miienemyg","50"),
    ("element","75"),
    ("shizue","46"),
    ("bayonetta","40"),
    ("shulk","39"),
    ("gaogaen","47"),
    ("krool","45"),
    ("other_head","4d"),
    ("lizardon","71"),
    ("base_tail","4c"),
    ("ike","23"),
    ("ryu","3c"),
    ("mariod","12"),
    ("pikmin","2b"),
    ("base_num","4d"),
    ("koopajr","3a"),
    ("inkling","41"),
    ("ness","a"),
    ("term","5e"),
    ("pickel","58"),
    ("miiswordsman","49")
    ]);
    let lowercased=target_kind.to_lowercase();
    if let Some(hex) = kind_map.get(lowercased.as_str()){
        let int = i64::from_str_radix(hex, 16);
        return int.unwrap() as i32;
    }
    return -1;
}

// Top level struct to hold the TOML data.
#[derive(Deserialize)]
struct ConfigToml {
    kind: String,
    slots: Vec<i32>,
    param_int: Option::<Vec<param_int>>,
    param_float: Option::<Vec<param_float>>
}

#[derive(Deserialize)]
struct param_int {
    param: String,
    subparam: Option::<String>,
    value: i32
}
#[derive(Deserialize)]
struct param_float {
    param: String,
    subparam: Option::<String>,
    value: f32
}

pub unsafe fn read_config(config_file: String)
{
    let contents = match fs::read_to_string(config_file.as_str()) {
        Ok(c) => c,
        Err(_) => {
            println!("[libparam_config::data] `{}`", config_file.as_str());
            return;
        }
    };
    let data: ConfigToml = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            println!("[libparam_config::data] Unable to load data from `{}`", config_file.as_str());
            return;
        }
    };
    println!("[libparam_config::data] Found file: {}",config_file.as_str());

    let kind_i32 = get_kind_from_string(&data.kind); //data.kind
    let mut manager = PARAM_MANAGER.write();
    let mut new_param = CharacterParam {
        kind: kind_i32,
        slots: data.slots,
        ints: HashMap::new(),
        floats: HashMap::new()
    };

    print!("[libparam_config::data] Loading params for {} on slots: ",data.kind);
    for slot in &new_param.slots {
        print!("{}, ",*slot);
    } 
    println!("");
 
    if data.param_int.is_some(){
        for param in data.param_int.unwrap() {
            let subparam_string = match param.subparam {
                Some(h) => h,
                None => String::from("")
            };
            let subparam = match subparam_string.as_str() {
                "" => 0,
                _ => hash40(subparam_string.as_str()),
            };

            let index = (hash40(param.param.as_str()),subparam);
            new_param.ints.insert(index,param.value);

            println!("{},{}: {}",param.param,subparam_string,param.value);
        } 
    }
    if data.param_float.is_some(){
        for param in data.param_float.unwrap() {
            let subparam_string = match param.subparam {
                Some(h) => h,
                None => String::from("")
            };
            let subparam = match subparam_string.as_str() {
                "" => 0,
                _ => hash40(subparam_string.as_str()),
            };

            let index = (hash40(param.param.as_str()),subparam);
            new_param.floats.insert(index,param.value);

            println!("{},{}: {}",param.param,subparam_string,param.value);
        } 
    }
    println!("");

    manager.push(new_param);
}

pub fn find_folders() ->bool {
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
                        read_config(format!("{}/{}", folder.as_str(),IDENTIFIER));
                        folders_found = true;
                    }
                }
            }
        }
        return folders_found;
    }
}

pub fn install() -> bool {
    //print_kinds();
    return find_folders();
}