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
            if kind == -1 {
                self.has_all = true;
            }
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

pub fn get_fighter_kind_from_string(target_kind: &str) -> i32 {
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
    return -2;
}
pub fn get_weapon_kind_from_string(target_kind: &str) -> i32 {
    let kind_map = HashMap::from([
        ("none","ffffffff"),
        ("term","267"),
        ("pit_bow","90"),
        ("jack_bus","1f3"),
        ("link_bow","1e"),
        ("ness_poo","64"),
        ("pitb_bow","e4"),
        ("ryu_sack","15d"),
        ("snake_c4","197"),
        ("buddy_pad","1ff"),
        ("diddy_gun","a2"),
        ("dolly_cap","20a"),
        ("edge_fire","246"),
        ("ike_sword","b3"),
        ("jack_fire","1ed"),
        ("jack_mona","1ef"),
        ("jack_wing","1f0"),
        ("kirby_hat","3f"),
        ("link_navy","20"),
        ("ness_yoyo","65"),
        ("pit_horse","93"),
        ("purin_cap","d5"),
        ("roy_sword","16b"),
        ("samus_gun","2a"),
        ("simon_axe","1ae"),
        ("brave_blue","1fe"),
        ("buddy_bird","201"),
        ("buddy_horn","207"),
        ("cloud_wave","16c"),
        ("dolly_fire","20b"),
        ("dolly_wave","208"),
        ("edge_flash","247"),
        ("fox_arwing","5c"),
        ("jack_doyle","1ec"),
        ("jack_fire2","1ee"),
        ("mario_pump","17"),
        ("master_axe","20c"),
        ("master_bow","20d"),
        ("ness_paula","63"),
        ("pacman_esa","13e"),
        ("pickel_axe","23b"),
        ("robot_beam","b9"),
        ("robot_gyro","b7"),
        ("samus_bomb","26"),
        ("samusd_gun","33"),
        ("shizue_pot","1d1"),
        ("shulk_riki","135"),
        ("simon_whip","1ad"),
        ("snake_rpg7","191"),
        ("yoshi_star","38"),
        ("zelda_dein","6f"),
        ("brave_crash","1fc"),
        ("brave_flash","1fa"),
        ("brave_sleep","1fd"),
        ("brave_spark","1f6"),
        ("buddy_piece","205"),
        ("chrom_sword","1b6"),
        ("dedede_mask","b1"),
        ("dedede_star","ac"),
        ("dolly_burst","209"),
        ("edge_flare1","248"),
        ("edge_flare2","249"),
        ("elight_beam","256"),
        ("fox_blaster","59"),
        ("fox_reticle","5d"),
        ("ganon_beast","82"),
        ("ganon_sword","81"),
        ("ken_hadoken","15e"),
        ("kirby_stone","47"),
        ("krool_crown","1a1"),
        ("lucina_mask","e3"),
        ("luigi_dokan","54"),
        ("mario_cappy","1b"),
        ("mario_dokan","1a"),
        ("pichu_cloud","df"),
        ("pickel_fire","243"),
        ("pickel_melt","241"),
        ("pickel_pick","23c"),
        ("pickel_rail","232"),
        ("pickel_wing","22f"),
        ("pikmin_win1","9f"),
        ("pikmin_win2","a0"),
        ("pikmin_win3","a1"),
        ("pit_chariot","92"),
        ("popo_condor","179"),
        ("popo_rubber","17b"),
        ("reflet_book","118"),
        ("richter_axe","1bf"),
        ("ryu_hadoken","15b"),
        ("samus_cshot","25"),
        ("samus_gbeam","2d"),
        ("samus_laser","28"),
        ("samusd_bomb","2f"),
        ("sheik_fusin","75"),
        ("sheik_knife","76"),
        ("shulk_fiora","136"),
        ("simon_cross","1af"),
        ("simon_stake","1b5"),
        ("simon_whip2","1b2"),
        ("tantan_beam","21c"),
        ("tantan_ring","21b"),
        ("wolf_wolfen","cc"),
        ("buddy_bullet","204"),
        ("daisy_kassar","6a"),
        ("dedede_gordo","ad"),
        ("demon_demonp","25a"),
        ("duckhunt_can","121"),
        ("falco_arwing","87"),
        ("fox_illusion","5b"),
        ("ganon_ganond","83"),
        ("gaogaen_rope","1ba"),
        ("kirby_hammer","3c"),
        ("kirby_simple","44"),
        ("koopa_breath","6d"),
        ("koopa_koopag","6e"),
        ("koopajr_kart","130"),
        ("lucas_bonnie","16a"),
        ("lucas_needle","168"),
        ("mario_mantle","16"),
        ("master_baton","212"),
        ("master_spear","210"),
        ("master_sword","211"),
        ("ness_pk_fire","60"),
        ("packun_mario","1ea"),
        ("pacman_fairy","144"),
        ("peach_kassar","67"),
        ("pichu_vortex","e0"),
        ("pickel_crack","230"),
        ("pickel_fence","242"),
        ("pickel_forge","238"),
        ("pickel_plate","233"),
        ("pickel_stone","234"),
        ("pickel_stuff","236"),
        ("pickel_sword","23a"),
        ("pickel_table","235"),
        ("pit_bowarrow","91"),
        ("popo_iceberg","17d"),
        ("popo_iceshot","17c"),
        ("reflet_chrom","11d"),
        ("richter_whip","1be"),
        ("rosetta_ring","e8"),
        ("rosetta_tico","e6"),
        ("samus_laser2","29"),
        ("samusd_cshot","2e"),
        ("samusd_gbeam","36"),
        ("samusd_laser","31"),
        ("sheik_needle","73"),
        ("shizue_broom","1e0"),
        ("shizue_swing","1e4"),
        ("shizue_timmy","1dc"),
        ("shizue_tommy","1db"),
        ("shizue_weeds","1d3"),
        ("shulk_dunban","134"),
        ("simon_coffin","1b0"),
        ("snake_cypher","196"),
        ("snake_nikita","194"),
        ("tantan_ally1","21d"),
        ("tantan_ally2","21e"),
        ("tantan_ally3","21f"),
        ("tantan_ally4","220"),
        ("tantan_ally5","221"),
        ("tantan_ally6","222"),
        ("toonlink_bow","c2"),
        ("toonlink_pig","c8"),
        ("wario_garlic","8b"),
        ("wiifit_towel","f0"),
        ("wiifit_wiibo","ef"),
        ("wolf_blaster","c9"),
        ("wolf_reticle","cd"),
        ("yoshi_tamago","39"),
        ("zelda_dein_s","70"),
        ("bayonetta_bat","171"),
        ("brave_tornado","1f8"),
        ("buddy_bigbird","202"),
        ("buddy_partner","200"),
        ("buddy_strings","206"),
        ("daisy_kinopio","6b"),
        ("dedede_shrine","ae"),
        ("demon_blaster","259"),
        ("diddy_bunshin","a7"),
        ("diddy_peanuts","a4"),
        ("duckhunt_clay","120"),
        ("eflame_esword","24c"),
        ("element_diver","1d0"),
        ("elight_esword","251"),
        ("elight_meteor","255"),
        ("falco_blaster","84"),
        ("falco_reticle","88"),
        ("gamewatch_oil","7b"),
        ("gaogaen_rope2","1bd"),
        ("gekkouga_moon","13a"),
        ("inkling_brush","18a"),
        ("inkling_squid","187"),
        ("jack_windummy","1f2"),
        ("jack_wirerope","1f1"),
        ("kirby_reserve","43"),
        ("koopag_breath","6d"),
        ("link_bowarrow","1f"),
        ("link_parasail","24"),
        ("lucas_pk_fire","162"),
    ]);
    let lowercased=target_kind.to_lowercase();
    if let Some(hex) = kind_map.get(lowercased.as_str()){
        let int = i64::from_str_radix(hex, 16);
        return int.unwrap() as i32;
    }
    return -2;
}

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
    kind: Option::<String>,
    slots: Option::<Vec<i32>>,
}
#[derive(Deserialize)]
struct param_float {
    param: String,
    subparam: Option::<String>,
    value: f32,
    kind: Option::<String>,
    slots: Option::<Vec<i32>>,
}

pub unsafe fn hash_str_to_u64(param: &str) -> u64
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
/* 
pub unsafe fn GetOrCreate(kind: i32, slots: Vec<i32>) -> CharacterParam {
    let mut manager = PARAM_MANAGER.write();

    let some_param = manager.get_param(kind, slots);
    if some_param.is_none(){
        let mut new_param = CharacterParam {
            kind: kind,
            slots: slots,
            ints: HashMap::new(),
            floats: HashMap::new()
        };
        manager.push(new_param);
        return new_param
    }
    else{
        return *(some_param.unwrap())
    }
    
}*/
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
    let mut mainKind = String::from("");
    let mut mainSlots = Vec::new();
    if data.kind.is_some(){
        mainKind = data.kind.unwrap();
        mainSlots = data.slots.unwrap();
    }
    let mut manager = PARAM_MANAGER.write();
/* 
    if data.param_int.is_some(){
        for param in data.param_int.unwrap() {
            let subparam = 0;
            let subparam_string = match param.subparam {
                Some(h) => h,
                None => String::from("")
            };
            let subparam_str = subparam_string.as_str();

            let index = (hash_str_to_u64(param.param.as_str()),hash_str_to_u64(subparam_str));

            if mainKind != "" {
                param.kind = Some(mainKind);
                param.slots = Some(mainSlots);
            }
            new_param.ints.insert(index,param.value);

            println!("{}({}): {}",param.param,subparam_str,param.value);
        } 
    }*/
    if data.param_float.is_some(){
        for param in data.param_float.unwrap() {
            let mut kind = mainKind.clone();
            let mut slots = mainSlots.clone();
            if param.kind.is_some() {
                param.kind.unwrap().clone();
            }
            if param.slots.is_some(){
                slots = param.slots.unwrap();
            }

            let isFighter = !(kind.contains("_") && kind != "ice_climber");
            let kind_i32 = if isFighter {get_fighter_kind_from_string(&kind)} else {get_weapon_kind_from_string(&kind)};
            if kind_i32 == -2 {
                print!("[libparam_config::data] {} is an invalid fighter",kind);
                continue;
            }

            let subparam = 0;
            let subparam_string = match param.subparam {
                Some(h) => h,
                None => String::from("")
            };
            let subparam_str = subparam_string.as_str();

            let index = (hash_str_to_u64(param.param.as_str()),hash_str_to_u64(subparam_str));

            manager.update_float(kind_i32,slots,index,param.value);

            println!("{}({}): {}",param.param,subparam_str,param.value);
        } 
    }
    println!("");
    //print!("[libparam_config::data] Loading params for {} on slots: ",data.kind);
    for p in &manager.params {
        let character = p.kind;
        print!("Character: {character} Slots: ");
        for s in &p.slots {
            print!("{},",*s);
        }
        println!("");
        for i in &p.ints {
            let param = i.0.0;
            let subparam = i.0.1;
            let value = i.1;
            println!("{param}({subparam}): {value}");
        }
        for f in &p.floats {
            let param = f.0.0;
            let subparam = f.0.1;
            let value = f.1;
            println!("{param}({subparam}): {value}");
        }
    } 
    //manager.push(new_param);
}
/* 
use arcropolis_api;
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
*/