#![feature(proc_macro_hygiene)]
use super::*;
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

pub static FIGHTER_TABLE: Lazy<Mutex<HashMap<&str,&str>>> = Lazy::new(|| {Mutex::new(HashMap::new())});
pub fn build_fighter_table() {
    FIGHTER_TABLE.lock().unwrap().insert("sonic","29");
    FIGHTER_TABLE.lock().unwrap().insert("demon","5c");
    FIGHTER_TABLE.lock().unwrap().insert("dolly","55");
    FIGHTER_TABLE.lock().unwrap().insert("link","2");
    FIGHTER_TABLE.lock().unwrap().insert("miienemys","4f");
    FIGHTER_TABLE.lock().unwrap().insert("tantan","57");
    FIGHTER_TABLE.lock().unwrap().insert("diddy","27");
    FIGHTER_TABLE.lock().unwrap().insert("jack","52");
    FIGHTER_TABLE.lock().unwrap().insert("sheik","10");
    FIGHTER_TABLE.lock().unwrap().insert("purin","c");
    FIGHTER_TABLE.lock().unwrap().insert("simon","43");
    FIGHTER_TABLE.lock().unwrap().insert("pichu","13");
    FIGHTER_TABLE.lock().unwrap().insert("koopag","4d");
    FIGHTER_TABLE.lock().unwrap().insert("reflet","38");
    FIGHTER_TABLE.lock().unwrap().insert("donkey","1");
    FIGHTER_TABLE.lock().unwrap().insert("rosetta","33");
    FIGHTER_TABLE.lock().unwrap().insert("elight","5b");
    FIGHTER_TABLE.lock().unwrap().insert("plizardon","26");
    FIGHTER_TABLE.lock().unwrap().insert("base_append_head","51");
    FIGHTER_TABLE.lock().unwrap().insert("brave","53");
    FIGHTER_TABLE.lock().unwrap().insert("zelda","11");
    FIGHTER_TABLE.lock().unwrap().insert("none","ffffffff");
    FIGHTER_TABLE.lock().unwrap().insert("duckhunt","3b");
    FIGHTER_TABLE.lock().unwrap().insert("rockman","31");
    FIGHTER_TABLE.lock().unwrap().insert("samus","3");
    FIGHTER_TABLE.lock().unwrap().insert("pfushigisou","25");
    FIGHTER_TABLE.lock().unwrap().insert("cloud","3e");
    FIGHTER_TABLE.lock().unwrap().insert("packun","51");
    FIGHTER_TABLE.lock().unwrap().insert("light","74");
    FIGHTER_TABLE.lock().unwrap().insert("younglink","17");
    FIGHTER_TABLE.lock().unwrap().insert("wiifit","32");
    FIGHTER_TABLE.lock().unwrap().insert("buddy","54");
    FIGHTER_TABLE.lock().unwrap().insert("robot","2d");
    FIGHTER_TABLE.lock().unwrap().insert("littlemac","34");
    FIGHTER_TABLE.lock().unwrap().insert("trail","5d");
    FIGHTER_TABLE.lock().unwrap().insert("samusd","4");
    FIGHTER_TABLE.lock().unwrap().insert("richter","44");
    FIGHTER_TABLE.lock().unwrap().insert("pzenigame","24");
    FIGHTER_TABLE.lock().unwrap().insert("snake","22");
    FIGHTER_TABLE.lock().unwrap().insert("table_ex_term","76");
    FIGHTER_TABLE.lock().unwrap().insert("edge","59");
    FIGHTER_TABLE.lock().unwrap().insert("yoshi","5");
    FIGHTER_TABLE.lock().unwrap().insert("ganon","18");
    FIGHTER_TABLE.lock().unwrap().insert("ridley","42");
    FIGHTER_TABLE.lock().unwrap().insert("ken","3d");
    FIGHTER_TABLE.lock().unwrap().insert("mewtwo","19");
    FIGHTER_TABLE.lock().unwrap().insert("wolf","2f");
    FIGHTER_TABLE.lock().unwrap().insert("roy","1a");
    FIGHTER_TABLE.lock().unwrap().insert("base_append_num","d");
    FIGHTER_TABLE.lock().unwrap().insert("falco","14");
    FIGHTER_TABLE.lock().unwrap().insert("base_head","0");
    FIGHTER_TABLE.lock().unwrap().insert("toonlink","2e");
    FIGHTER_TABLE.lock().unwrap().insert("zenigame","6f");
    FIGHTER_TABLE.lock().unwrap().insert("murabito","30");
    FIGHTER_TABLE.lock().unwrap().insert("miienemyf","4e");
    FIGHTER_TABLE.lock().unwrap().insert("captain","b");
    FIGHTER_TABLE.lock().unwrap().insert("kamui","3f");
    FIGHTER_TABLE.lock().unwrap().insert("nana","4c");
    FIGHTER_TABLE.lock().unwrap().insert("szerosuit","20");
    FIGHTER_TABLE.lock().unwrap().insert("ptrainer","72");
    FIGHTER_TABLE.lock().unwrap().insert("popo","4b");
    FIGHTER_TABLE.lock().unwrap().insert("fushigisou","70");
    FIGHTER_TABLE.lock().unwrap().insert("pikachu","8");
    FIGHTER_TABLE.lock().unwrap().insert("gekkouga","35");
    FIGHTER_TABLE.lock().unwrap().insert("other_tail","50");
    FIGHTER_TABLE.lock().unwrap().insert("lucas","28");
    FIGHTER_TABLE.lock().unwrap().insert("fox","7");
    FIGHTER_TABLE.lock().unwrap().insert("palutena","36");
    FIGHTER_TABLE.lock().unwrap().insert("dedede","2a");
    FIGHTER_TABLE.lock().unwrap().insert("ice_climber","6e");
    FIGHTER_TABLE.lock().unwrap().insert("wario","21");
    FIGHTER_TABLE.lock().unwrap().insert("koopa","f");
    FIGHTER_TABLE.lock().unwrap().insert("daisy","e");
    FIGHTER_TABLE.lock().unwrap().insert("flame","73");
    FIGHTER_TABLE.lock().unwrap().insert("lucina","16");
    FIGHTER_TABLE.lock().unwrap().insert("mario","0");
    FIGHTER_TABLE.lock().unwrap().insert("chrom","1b");
    FIGHTER_TABLE.lock().unwrap().insert("kirby","6");
    FIGHTER_TABLE.lock().unwrap().insert("pit","1e");
    FIGHTER_TABLE.lock().unwrap().insert("eflame","5a");
    FIGHTER_TABLE.lock().unwrap().insert("lucario","2c");
    FIGHTER_TABLE.lock().unwrap().insert("marth","15");
    FIGHTER_TABLE.lock().unwrap().insert("master","56");
    FIGHTER_TABLE.lock().unwrap().insert("metaknight","1d");
    FIGHTER_TABLE.lock().unwrap().insert("peach","d");
    FIGHTER_TABLE.lock().unwrap().insert("table_ex_start","6d");
    FIGHTER_TABLE.lock().unwrap().insert("pitb","1f");
    FIGHTER_TABLE.lock().unwrap().insert("gamewatch","1c");
    FIGHTER_TABLE.lock().unwrap().insert("other_num","4");
    FIGHTER_TABLE.lock().unwrap().insert("luigi","9");
    FIGHTER_TABLE.lock().unwrap().insert("miifighter","48");
    FIGHTER_TABLE.lock().unwrap().insert("pacman","37");
    FIGHTER_TABLE.lock().unwrap().insert("random","77");
    FIGHTER_TABLE.lock().unwrap().insert("base_append_tail","5d");
    FIGHTER_TABLE.lock().unwrap().insert("miigunner","4a");
    FIGHTER_TABLE.lock().unwrap().insert("miienemyg","50");
    FIGHTER_TABLE.lock().unwrap().insert("element","75");
    FIGHTER_TABLE.lock().unwrap().insert("shizue","46");
    FIGHTER_TABLE.lock().unwrap().insert("bayonetta","40");
    FIGHTER_TABLE.lock().unwrap().insert("shulk","39");
    FIGHTER_TABLE.lock().unwrap().insert("gaogaen","47");
    FIGHTER_TABLE.lock().unwrap().insert("krool","45");
    FIGHTER_TABLE.lock().unwrap().insert("other_head","4d");
    FIGHTER_TABLE.lock().unwrap().insert("lizardon","71");
    FIGHTER_TABLE.lock().unwrap().insert("base_tail","4c");
    FIGHTER_TABLE.lock().unwrap().insert("ike","23");
    FIGHTER_TABLE.lock().unwrap().insert("ryu","3c");
    FIGHTER_TABLE.lock().unwrap().insert("mariod","12");
    FIGHTER_TABLE.lock().unwrap().insert("pikmin","2b");
    FIGHTER_TABLE.lock().unwrap().insert("base_num","4d");
    FIGHTER_TABLE.lock().unwrap().insert("koopajr","3a");
    FIGHTER_TABLE.lock().unwrap().insert("inkling","41");
    FIGHTER_TABLE.lock().unwrap().insert("ness","a");
    FIGHTER_TABLE.lock().unwrap().insert("pickel","58");
    FIGHTER_TABLE.lock().unwrap().insert("miiswordsman","49");
    FIGHTER_TABLE.lock().unwrap().insert("term","5e");
    FIGHTER_TABLE.lock().unwrap().insert("all","5e");
}
pub fn get_fighter_kind_from_string(target_kind: &str) -> i32 {
    
    let lowercased=target_kind.to_lowercase();
    if let Some(hex) = FIGHTER_TABLE.lock().unwrap().get(lowercased.as_str()){
        let int = i64::from_str_radix(hex, 16);
        return int.unwrap() as i32;
    }
    return 999;
}

pub static WEAPON_TABLE: Lazy<Mutex<HashMap<&str,&str>>> = Lazy::new(|| {Mutex::new(HashMap::new())
});
pub fn build_weapon_table() {
        WEAPON_TABLE.lock().unwrap().insert("bayonetta_bat","171");
        WEAPON_TABLE.lock().unwrap().insert("bayonetta_gomorrah","172");
        WEAPON_TABLE.lock().unwrap().insert("bayonetta_hair","173");
        WEAPON_TABLE.lock().unwrap().insert("bayonetta_specialn_bullet","16e");
        WEAPON_TABLE.lock().unwrap().insert("bayonetta_wickedweavearm","16f");
        WEAPON_TABLE.lock().unwrap().insert("bayonetta_wickedweaveleg","170");
        WEAPON_TABLE.lock().unwrap().insert("brave_blue","1fe");
        WEAPON_TABLE.lock().unwrap().insert("brave_crash","1fc");
        WEAPON_TABLE.lock().unwrap().insert("brave_deathball","1fb");
        WEAPON_TABLE.lock().unwrap().insert("brave_explosion","1f9");
        WEAPON_TABLE.lock().unwrap().insert("brave_fireball","1f5");
        WEAPON_TABLE.lock().unwrap().insert("brave_flash","1fa");
        WEAPON_TABLE.lock().unwrap().insert("brave_lightning","1f7");
        WEAPON_TABLE.lock().unwrap().insert("brave_sleep","1fd");
        WEAPON_TABLE.lock().unwrap().insert("brave_spark","1f6");
        WEAPON_TABLE.lock().unwrap().insert("brave_tornado","1f8");
        WEAPON_TABLE.lock().unwrap().insert("buddy_bigbird","202");
        WEAPON_TABLE.lock().unwrap().insert("buddy_bigbirdbase","203");
        WEAPON_TABLE.lock().unwrap().insert("buddy_bird","201");
        WEAPON_TABLE.lock().unwrap().insert("buddy_bullet","204");
        WEAPON_TABLE.lock().unwrap().insert("buddy_horn","207");
        WEAPON_TABLE.lock().unwrap().insert("buddy_pad","1ff");
        WEAPON_TABLE.lock().unwrap().insert("buddy_partner","200");
        WEAPON_TABLE.lock().unwrap().insert("buddy_piece","205");
        WEAPON_TABLE.lock().unwrap().insert("buddy_strings","206");
        WEAPON_TABLE.lock().unwrap().insert("captain_bluefalcon","57");
        WEAPON_TABLE.lock().unwrap().insert("captain_falconpunch","58");
        WEAPON_TABLE.lock().unwrap().insert("chrom_sword","1b6");
        WEAPON_TABLE.lock().unwrap().insert("cloud_wave","16c");
        WEAPON_TABLE.lock().unwrap().insert("daisy_kassar","6a");
        WEAPON_TABLE.lock().unwrap().insert("daisy_kinopio","6b");
        WEAPON_TABLE.lock().unwrap().insert("daisy_kinopiospore","6c");
        WEAPON_TABLE.lock().unwrap().insert("dedede_gordo","ad");
        WEAPON_TABLE.lock().unwrap().insert("dedede_jethammer","aa");
        WEAPON_TABLE.lock().unwrap().insert("dedede_mask","b1");
        WEAPON_TABLE.lock().unwrap().insert("dedede_missile","b2");
        WEAPON_TABLE.lock().unwrap().insert("dedede_newdededehammer","b0");
        WEAPON_TABLE.lock().unwrap().insert("dedede_shrine","ae");
        WEAPON_TABLE.lock().unwrap().insert("dedede_star","ac");
        WEAPON_TABLE.lock().unwrap().insert("dedede_star_missile","ab");
        WEAPON_TABLE.lock().unwrap().insert("dedede_waddledee","af");
        WEAPON_TABLE.lock().unwrap().insert("demon_blaster","259");
        WEAPON_TABLE.lock().unwrap().insert("demon_blasterchest","25b");
        WEAPON_TABLE.lock().unwrap().insert("demon_blasterhead","25c");
        WEAPON_TABLE.lock().unwrap().insert("demon_blasterwing","25d");
        WEAPON_TABLE.lock().unwrap().insert("demon_demonp","25a");
        WEAPON_TABLE.lock().unwrap().insert("diddy_barreljet","a3");
        WEAPON_TABLE.lock().unwrap().insert("diddy_barreljets","a6");
        WEAPON_TABLE.lock().unwrap().insert("diddy_bunshin","a7");
        WEAPON_TABLE.lock().unwrap().insert("diddy_dkbarrel","a8");
        WEAPON_TABLE.lock().unwrap().insert("diddy_explosion","a5");
        WEAPON_TABLE.lock().unwrap().insert("diddy_gun","a2");
        WEAPON_TABLE.lock().unwrap().insert("diddy_lock_on_cursor","a9");
        WEAPON_TABLE.lock().unwrap().insert("diddy_peanuts","a4");
        WEAPON_TABLE.lock().unwrap().insert("dolly_burst","209");
        WEAPON_TABLE.lock().unwrap().insert("dolly_cap","20a");
        WEAPON_TABLE.lock().unwrap().insert("dolly_fire","20b");
        WEAPON_TABLE.lock().unwrap().insert("dolly_wave","208");
        WEAPON_TABLE.lock().unwrap().insert("donkey_dkbarrel","1c");
        WEAPON_TABLE.lock().unwrap().insert("duckhunt_can","121");
        WEAPON_TABLE.lock().unwrap().insert("duckhunt_clay","120");
        WEAPON_TABLE.lock().unwrap().insert("duckhunt_finalbird","128");
        WEAPON_TABLE.lock().unwrap().insert("duckhunt_finalcan","12a");
        WEAPON_TABLE.lock().unwrap().insert("duckhunt_finaldog","127");
        WEAPON_TABLE.lock().unwrap().insert("duckhunt_finalduck","124");
        WEAPON_TABLE.lock().unwrap().insert("duckhunt_finalenemy","126");
        WEAPON_TABLE.lock().unwrap().insert("duckhunt_finalgrass","129");
        WEAPON_TABLE.lock().unwrap().insert("duckhunt_finalgunman","125");
        WEAPON_TABLE.lock().unwrap().insert("duckhunt_grass","12b");
        WEAPON_TABLE.lock().unwrap().insert("duckhunt_gunman","11e");
        WEAPON_TABLE.lock().unwrap().insert("duckhunt_gunmanbullet","11f");
        WEAPON_TABLE.lock().unwrap().insert("duckhunt_kurofukuhat","123");
        WEAPON_TABLE.lock().unwrap().insert("duckhunt_reticle","122");
        WEAPON_TABLE.lock().unwrap().insert("edge_background","24b");
        WEAPON_TABLE.lock().unwrap().insert("edge_fire","246");
        WEAPON_TABLE.lock().unwrap().insert("edge_flare1","248");
        WEAPON_TABLE.lock().unwrap().insert("edge_flare2","249");
        WEAPON_TABLE.lock().unwrap().insert("edge_flaredummy","24a");
        WEAPON_TABLE.lock().unwrap().insert("edge_flash","247");
        WEAPON_TABLE.lock().unwrap().insert("eflame_blazepillar","24e");
        WEAPON_TABLE.lock().unwrap().insert("eflame_esword","24c");
        WEAPON_TABLE.lock().unwrap().insert("eflame_firepillar","24d");
        WEAPON_TABLE.lock().unwrap().insert("eflame_windummy","24f");
        WEAPON_TABLE.lock().unwrap().insert("eflame_winsword","250");
        WEAPON_TABLE.lock().unwrap().insert("element_changer","1cf");
        WEAPON_TABLE.lock().unwrap().insert("element_diver","1d0");
        WEAPON_TABLE.lock().unwrap().insert("elight_beam","256");
        WEAPON_TABLE.lock().unwrap().insert("elight_bunshin","252");
        WEAPON_TABLE.lock().unwrap().insert("elight_esword","251");
        WEAPON_TABLE.lock().unwrap().insert("elight_exprosiveshot","253");
        WEAPON_TABLE.lock().unwrap().insert("elight_meteor","255");
        WEAPON_TABLE.lock().unwrap().insert("elight_spreadbullet","254");
        WEAPON_TABLE.lock().unwrap().insert("elight_windummy","257");
        WEAPON_TABLE.lock().unwrap().insert("elight_winsword","258");
        WEAPON_TABLE.lock().unwrap().insert("enemy_yellowdevil_beam","1c7");
        WEAPON_TABLE.lock().unwrap().insert("falco_arwing","87");
        WEAPON_TABLE.lock().unwrap().insert("falco_arwingshot","89");
        WEAPON_TABLE.lock().unwrap().insert("falco_blaster","84");
        WEAPON_TABLE.lock().unwrap().insert("falco_blaster_bullet","85");
        WEAPON_TABLE.lock().unwrap().insert("falco_illusion","86");
        WEAPON_TABLE.lock().unwrap().insert("falco_reticle","88");
        WEAPON_TABLE.lock().unwrap().insert("fox_arwing","5c");
        WEAPON_TABLE.lock().unwrap().insert("fox_arwingshot","5e");
        WEAPON_TABLE.lock().unwrap().insert("fox_blaster","59");
        WEAPON_TABLE.lock().unwrap().insert("fox_blaster_bullet","5a");
        WEAPON_TABLE.lock().unwrap().insert("fox_illusion","5b");
        WEAPON_TABLE.lock().unwrap().insert("fox_reticle","5d");
        WEAPON_TABLE.lock().unwrap().insert("gamewatch_bomb","80");
        WEAPON_TABLE.lock().unwrap().insert("gamewatch_breath","7e");
        WEAPON_TABLE.lock().unwrap().insert("gamewatch_entry","7f");
        WEAPON_TABLE.lock().unwrap().insert("gamewatch_food","77");
        WEAPON_TABLE.lock().unwrap().insert("gamewatch_normal_weapon","7d");
        WEAPON_TABLE.lock().unwrap().insert("gamewatch_octopus","7c");
        WEAPON_TABLE.lock().unwrap().insert("gamewatch_oil","7b");
        WEAPON_TABLE.lock().unwrap().insert("gamewatch_panel","7a");
        WEAPON_TABLE.lock().unwrap().insert("gamewatch_parachute","79");
        WEAPON_TABLE.lock().unwrap().insert("gamewatch_rescue","78");
        WEAPON_TABLE.lock().unwrap().insert("ganon_beast","82");
        WEAPON_TABLE.lock().unwrap().insert("ganon_ganond","83");
        WEAPON_TABLE.lock().unwrap().insert("ganon_sword","81");
        WEAPON_TABLE.lock().unwrap().insert("gaogaen_championbelt","1bb");
        WEAPON_TABLE.lock().unwrap().insert("gaogaen_monsterball","1bc");
        WEAPON_TABLE.lock().unwrap().insert("gaogaen_rope","1ba");
        WEAPON_TABLE.lock().unwrap().insert("gaogaen_rope2","1bd");
        WEAPON_TABLE.lock().unwrap().insert("gekkouga_bunshin","13c");
        WEAPON_TABLE.lock().unwrap().insert("gekkouga_gekkougas","13d");
        WEAPON_TABLE.lock().unwrap().insert("gekkouga_monsterball","13b");
        WEAPON_TABLE.lock().unwrap().insert("gekkouga_moon","13a");
        WEAPON_TABLE.lock().unwrap().insert("gekkouga_shuriken","137");
        WEAPON_TABLE.lock().unwrap().insert("gekkouga_tatami","139");
        WEAPON_TABLE.lock().unwrap().insert("gekkouga_water","138");
        WEAPON_TABLE.lock().unwrap().insert("ike_sword","b3");
        WEAPON_TABLE.lock().unwrap().insert("inkling_blaster","18b");
        WEAPON_TABLE.lock().unwrap().insert("inkling_brush","18a");
        WEAPON_TABLE.lock().unwrap().insert("inkling_copy_inklinggun","18f");
        WEAPON_TABLE.lock().unwrap().insert("inkling_copy_inklingtank","190");
        WEAPON_TABLE.lock().unwrap().insert("inkling_inkbullet","185");
        WEAPON_TABLE.lock().unwrap().insert("inkling_megaphonelaser","18e");
        WEAPON_TABLE.lock().unwrap().insert("inkling_roller","188");
        WEAPON_TABLE.lock().unwrap().insert("inkling_rollerink","189");
        WEAPON_TABLE.lock().unwrap().insert("inkling_slosher","18c");
        WEAPON_TABLE.lock().unwrap().insert("inkling_splash","18d");
        WEAPON_TABLE.lock().unwrap().insert("inkling_splashbomb","186");
        WEAPON_TABLE.lock().unwrap().insert("inkling_squid","187");
        WEAPON_TABLE.lock().unwrap().insert("jack_background","1f4");
        WEAPON_TABLE.lock().unwrap().insert("jack_bus","1f3");
        WEAPON_TABLE.lock().unwrap().insert("jack_doyle","1ec");
        WEAPON_TABLE.lock().unwrap().insert("jack_fire","1ed");
        WEAPON_TABLE.lock().unwrap().insert("jack_fire2","1ee");
        WEAPON_TABLE.lock().unwrap().insert("jack_mona","1ef");
        WEAPON_TABLE.lock().unwrap().insert("jack_windummy","1f2");
        WEAPON_TABLE.lock().unwrap().insert("jack_wing","1f0");
        WEAPON_TABLE.lock().unwrap().insert("jack_wirerope","1f1");
        WEAPON_TABLE.lock().unwrap().insert("kamui_dragonhand","175");
        WEAPON_TABLE.lock().unwrap().insert("kamui_ryusensya","174");
        WEAPON_TABLE.lock().unwrap().insert("kamui_spearhand","176");
        WEAPON_TABLE.lock().unwrap().insert("kamui_waterdragon","177");
        WEAPON_TABLE.lock().unwrap().insert("kamui_waterstream","178");
        WEAPON_TABLE.lock().unwrap().insert("ken_hadoken","15e");
        WEAPON_TABLE.lock().unwrap().insert("ken_shinkuhadoken","15f");
        WEAPON_TABLE.lock().unwrap().insert("ken_shinryuken","160");
        WEAPON_TABLE.lock().unwrap().insert("kirby_finalcutter","4b");
        WEAPON_TABLE.lock().unwrap().insert("kirby_finalcuttershot","3d");
        WEAPON_TABLE.lock().unwrap().insert("kirby_hammer","3c");
        WEAPON_TABLE.lock().unwrap().insert("kirby_hat","3f");
        WEAPON_TABLE.lock().unwrap().insert("kirby_miipartshead","45");
        WEAPON_TABLE.lock().unwrap().insert("kirby_reserve","43");
        WEAPON_TABLE.lock().unwrap().insert("kirby_rosettaticomissile","46");
        WEAPON_TABLE.lock().unwrap().insert("kirby_simple","44");
        WEAPON_TABLE.lock().unwrap().insert("kirby_simple2l","49");
        WEAPON_TABLE.lock().unwrap().insert("kirby_simple2r","48");
        WEAPON_TABLE.lock().unwrap().insert("kirby_starmissile","3e");
        WEAPON_TABLE.lock().unwrap().insert("kirby_stone","47");
        WEAPON_TABLE.lock().unwrap().insert("kirby_ultrasword","40");
        WEAPON_TABLE.lock().unwrap().insert("kirby_ultraswordhat","41");
        WEAPON_TABLE.lock().unwrap().insert("kirby_warpstar","42");
        WEAPON_TABLE.lock().unwrap().insert("kirby_windummy","4a");
        WEAPON_TABLE.lock().unwrap().insert("koopag_breath","1b9");
        WEAPON_TABLE.lock().unwrap().insert("koopajr_batten","133");
        WEAPON_TABLE.lock().unwrap().insert("koopajr_cannonball","12f");
        WEAPON_TABLE.lock().unwrap().insert("koopajr_hammer","12c");
        WEAPON_TABLE.lock().unwrap().insert("koopajr_kart","130");
        WEAPON_TABLE.lock().unwrap().insert("koopajr_magichand","12e");
        WEAPON_TABLE.lock().unwrap().insert("koopajr_picopicohammer","12d");
        WEAPON_TABLE.lock().unwrap().insert("koopajr_remainclown","131");
        WEAPON_TABLE.lock().unwrap().insert("koopajr_shadowmario","132");
        WEAPON_TABLE.lock().unwrap().insert("koopa_breath","6d");
        WEAPON_TABLE.lock().unwrap().insert("koopa_koopag","6e");
        WEAPON_TABLE.lock().unwrap().insert("krool_backpack","1a0");
        WEAPON_TABLE.lock().unwrap().insert("krool_blunderbuss","1a3");
        WEAPON_TABLE.lock().unwrap().insert("krool_crown","1a1");
        WEAPON_TABLE.lock().unwrap().insert("krool_ironball","1a4");
        WEAPON_TABLE.lock().unwrap().insert("krool_piratehat","1a2");
        WEAPON_TABLE.lock().unwrap().insert("krool_spitball","1a5");
        WEAPON_TABLE.lock().unwrap().insert("link_ancient_bow","22");
        WEAPON_TABLE.lock().unwrap().insert("link_ancient_bowarrow","23");
        WEAPON_TABLE.lock().unwrap().insert("link_boomerang","1d");
        WEAPON_TABLE.lock().unwrap().insert("link_bow","1e");
        WEAPON_TABLE.lock().unwrap().insert("link_bowarrow","1f");
        WEAPON_TABLE.lock().unwrap().insert("link_navy","20");
        WEAPON_TABLE.lock().unwrap().insert("link_parasail","24");
        WEAPON_TABLE.lock().unwrap().insert("link_sword_beam","21");
        WEAPON_TABLE.lock().unwrap().insert("littlemac_championbelt","f6");
        WEAPON_TABLE.lock().unwrap().insert("littlemac_doclouis","f3");
        WEAPON_TABLE.lock().unwrap().insert("littlemac_littlemacg","f7");
        WEAPON_TABLE.lock().unwrap().insert("littlemac_sweatlittlemac","f4");
        WEAPON_TABLE.lock().unwrap().insert("littlemac_throwsweat","f5");
        WEAPON_TABLE.lock().unwrap().insert("lucario_auraball","b4");
        WEAPON_TABLE.lock().unwrap().insert("lucario_lucariom","b6");
        WEAPON_TABLE.lock().unwrap().insert("lucario_qigong","b5");
        WEAPON_TABLE.lock().unwrap().insert("lucas_bonnie","16a");
        WEAPON_TABLE.lock().unwrap().insert("lucas_doseitable","167");
        WEAPON_TABLE.lock().unwrap().insert("lucas_himohebi","165");
        WEAPON_TABLE.lock().unwrap().insert("lucas_himohebi2","166");
        WEAPON_TABLE.lock().unwrap().insert("lucas_kumatora","169");
        WEAPON_TABLE.lock().unwrap().insert("lucas_needle","168");
        WEAPON_TABLE.lock().unwrap().insert("lucas_pk_fire","162");
        WEAPON_TABLE.lock().unwrap().insert("lucas_pk_freeze","161");
        WEAPON_TABLE.lock().unwrap().insert("lucas_pk_starstorm","164");
        WEAPON_TABLE.lock().unwrap().insert("lucas_pk_thunder","163");
        WEAPON_TABLE.lock().unwrap().insert("lucina_mask","e3");
        WEAPON_TABLE.lock().unwrap().insert("luigi_dokan","54");
        WEAPON_TABLE.lock().unwrap().insert("luigi_fireball","53");
        WEAPON_TABLE.lock().unwrap().insert("luigi_obakyumu","55");
        WEAPON_TABLE.lock().unwrap().insert("luigi_plunger","56");
        WEAPON_TABLE.lock().unwrap().insert("mariod_capsuleblock","db");
        WEAPON_TABLE.lock().unwrap().insert("mariod_drcapsule","d7");
        WEAPON_TABLE.lock().unwrap().insert("mariod_drmantle","d8");
        WEAPON_TABLE.lock().unwrap().insert("mariod_huge_capsule","da");
        WEAPON_TABLE.lock().unwrap().insert("mariod_stethoscope","d9");
        WEAPON_TABLE.lock().unwrap().insert("mario_cappy","1b");
        WEAPON_TABLE.lock().unwrap().insert("mario_dokan","1a");
        WEAPON_TABLE.lock().unwrap().insert("mario_fireball","15");
        WEAPON_TABLE.lock().unwrap().insert("mario_huge_flame","19");
        WEAPON_TABLE.lock().unwrap().insert("mario_mantle","16");
        WEAPON_TABLE.lock().unwrap().insert("mario_pump","17");
        WEAPON_TABLE.lock().unwrap().insert("mario_pump_water","18");
        WEAPON_TABLE.lock().unwrap().insert("master_arrow1","20e");
        WEAPON_TABLE.lock().unwrap().insert("master_arrow2","20f");
        WEAPON_TABLE.lock().unwrap().insert("master_axe","20c");
        WEAPON_TABLE.lock().unwrap().insert("master_background","213");
        WEAPON_TABLE.lock().unwrap().insert("master_baton","212");
        WEAPON_TABLE.lock().unwrap().insert("master_bow","20d");
        WEAPON_TABLE.lock().unwrap().insert("master_spear","210");
        WEAPON_TABLE.lock().unwrap().insert("master_sword","211");
        WEAPON_TABLE.lock().unwrap().insert("master_sword2","214");
        WEAPON_TABLE.lock().unwrap().insert("master_swordflare","215");
        WEAPON_TABLE.lock().unwrap().insert("metaknight_bunshin","8e");
        WEAPON_TABLE.lock().unwrap().insert("metaknight_fourwings","8f");
        WEAPON_TABLE.lock().unwrap().insert("metaknight_mantle","8d");
        WEAPON_TABLE.lock().unwrap().insert("mewtwo_bindball","156");
        WEAPON_TABLE.lock().unwrap().insert("mewtwo_escapeairdummy","15a");
        WEAPON_TABLE.lock().unwrap().insert("mewtwo_mewtwom","157");
        WEAPON_TABLE.lock().unwrap().insert("mewtwo_psychobreak","159");
        WEAPON_TABLE.lock().unwrap().insert("mewtwo_search","158");
        WEAPON_TABLE.lock().unwrap().insert("mewtwo_shadowball","155");
        WEAPON_TABLE.lock().unwrap().insert("miienemyg_attackairf_bullet","1c8");
        WEAPON_TABLE.lock().unwrap().insert("miienemyg_rapidshot_bullet","1c9");
        WEAPON_TABLE.lock().unwrap().insert("miifighter_hat","0");
        WEAPON_TABLE.lock().unwrap().insert("miifighter_ironball","1");
        WEAPON_TABLE.lock().unwrap().insert("miigunner_attackairf_bullet","12");
        WEAPON_TABLE.lock().unwrap().insert("miigunner_bottomshoot","10");
        WEAPON_TABLE.lock().unwrap().insert("miigunner_flamepillar","d");
        WEAPON_TABLE.lock().unwrap().insert("miigunner_fullthrottle","14");
        WEAPON_TABLE.lock().unwrap().insert("miigunner_grenadelauncher","c");
        WEAPON_TABLE.lock().unwrap().insert("miigunner_groundbomb","11");
        WEAPON_TABLE.lock().unwrap().insert("miigunner_gunnercharge","9");
        WEAPON_TABLE.lock().unwrap().insert("miigunner_hat","7");
        WEAPON_TABLE.lock().unwrap().insert("miigunner_laser","13");
        WEAPON_TABLE.lock().unwrap().insert("miigunner_miimissile","a");
        WEAPON_TABLE.lock().unwrap().insert("miigunner_rapidshot_bullet","8");
        WEAPON_TABLE.lock().unwrap().insert("miigunner_stealthbomb","e");
        WEAPON_TABLE.lock().unwrap().insert("miigunner_stealthbomb_s","f");
        WEAPON_TABLE.lock().unwrap().insert("miigunner_supermissile","b");
        WEAPON_TABLE.lock().unwrap().insert("miiswordsman_chakram","4");
        WEAPON_TABLE.lock().unwrap().insert("miiswordsman_hat","2");
        WEAPON_TABLE.lock().unwrap().insert("miiswordsman_lightshuriken","3");
        WEAPON_TABLE.lock().unwrap().insert("miiswordsman_tornadoshot","5");
        WEAPON_TABLE.lock().unwrap().insert("miiswordsman_wave","6");
        WEAPON_TABLE.lock().unwrap().insert("murabito_balloon","100");
        WEAPON_TABLE.lock().unwrap().insert("murabito_beetle","10e");
        WEAPON_TABLE.lock().unwrap().insert("murabito_bowling_ball","fa");
        WEAPON_TABLE.lock().unwrap().insert("murabito_bullet","fd");
        WEAPON_TABLE.lock().unwrap().insert("murabito_butterflynet","ff");
        WEAPON_TABLE.lock().unwrap().insert("murabito_clayrocket","101");
        WEAPON_TABLE.lock().unwrap().insert("murabito_firework","fb");
        WEAPON_TABLE.lock().unwrap().insert("murabito_flowerpot","f8");
        WEAPON_TABLE.lock().unwrap().insert("murabito_furniture","10c");
        WEAPON_TABLE.lock().unwrap().insert("murabito_helmet","107");
        WEAPON_TABLE.lock().unwrap().insert("murabito_house","10b");
        WEAPON_TABLE.lock().unwrap().insert("murabito_moneybag","10d");
        WEAPON_TABLE.lock().unwrap().insert("murabito_seed","102");
        WEAPON_TABLE.lock().unwrap().insert("murabito_slingshot","fc");
        WEAPON_TABLE.lock().unwrap().insert("murabito_sprinkling_water","106");
        WEAPON_TABLE.lock().unwrap().insert("murabito_sprout","103");
        WEAPON_TABLE.lock().unwrap().insert("murabito_stump","105");
        WEAPON_TABLE.lock().unwrap().insert("murabito_timmy","10a");
        WEAPON_TABLE.lock().unwrap().insert("murabito_tommy","109");
        WEAPON_TABLE.lock().unwrap().insert("murabito_tomnook","108");
        WEAPON_TABLE.lock().unwrap().insert("murabito_tree","104");
        WEAPON_TABLE.lock().unwrap().insert("murabito_umbrella","f9");
        WEAPON_TABLE.lock().unwrap().insert("murabito_weeds","fe");
        WEAPON_TABLE.lock().unwrap().insert("ness_paula","63");
        WEAPON_TABLE.lock().unwrap().insert("ness_pk_fire","60");
        WEAPON_TABLE.lock().unwrap().insert("ness_pk_flash","5f");
        WEAPON_TABLE.lock().unwrap().insert("ness_pk_starstorm","62");
        WEAPON_TABLE.lock().unwrap().insert("ness_pk_thunder","61");
        WEAPON_TABLE.lock().unwrap().insert("ness_poo","64");
        WEAPON_TABLE.lock().unwrap().insert("ness_yoyo","65");
        WEAPON_TABLE.lock().unwrap().insert("ness_yoyo_head","66");
        WEAPON_TABLE.lock().unwrap().insert("none","ffffffff");
        WEAPON_TABLE.lock().unwrap().insert("packun_bosspackun","1eb");
        WEAPON_TABLE.lock().unwrap().insert("packun_mario","1ea");
        WEAPON_TABLE.lock().unwrap().insert("packun_poisonbreath","1e9");
        WEAPON_TABLE.lock().unwrap().insert("packun_spikeball","1e8");
        WEAPON_TABLE.lock().unwrap().insert("pacman_artisticpoint","143");
        WEAPON_TABLE.lock().unwrap().insert("pacman_bigpacman","142");
        WEAPON_TABLE.lock().unwrap().insert("pacman_esa","13e");
        WEAPON_TABLE.lock().unwrap().insert("pacman_fairy","144");
        WEAPON_TABLE.lock().unwrap().insert("pacman_firehydrant","140");
        WEAPON_TABLE.lock().unwrap().insert("pacman_firehydrant_water","141");
        WEAPON_TABLE.lock().unwrap().insert("pacman_trampoline","13f");
        WEAPON_TABLE.lock().unwrap().insert("palutena_autoaimbullet","112");
        WEAPON_TABLE.lock().unwrap().insert("palutena_autoreticle","113");
        WEAPON_TABLE.lock().unwrap().insert("palutena_beam","116");
        WEAPON_TABLE.lock().unwrap().insert("palutena_blackhole","115");
        WEAPON_TABLE.lock().unwrap().insert("palutena_explosiveflame","110");
        WEAPON_TABLE.lock().unwrap().insert("palutena_explosiveflame_reserve","111");
        WEAPON_TABLE.lock().unwrap().insert("palutena_gate","117");
        WEAPON_TABLE.lock().unwrap().insert("palutena_godwing","10f");
        WEAPON_TABLE.lock().unwrap().insert("palutena_reflectionboard","114");
        WEAPON_TABLE.lock().unwrap().insert("peach_kassar","67");
        WEAPON_TABLE.lock().unwrap().insert("peach_kinopio","68");
        WEAPON_TABLE.lock().unwrap().insert("peach_kinopiospore","69");
        WEAPON_TABLE.lock().unwrap().insert("pfushigisou_leafcutter","183");
        WEAPON_TABLE.lock().unwrap().insert("pfushigisou_seed","182");
        WEAPON_TABLE.lock().unwrap().insert("pfushigisou_vine","184");
        WEAPON_TABLE.lock().unwrap().insert("pichu_cloud","df");
        WEAPON_TABLE.lock().unwrap().insert("pichu_dengeki","dd");
        WEAPON_TABLE.lock().unwrap().insert("pichu_dengekidama","dc");
        WEAPON_TABLE.lock().unwrap().insert("pichu_kaminari","de");
        WEAPON_TABLE.lock().unwrap().insert("pichu_monsterball","e1");
        WEAPON_TABLE.lock().unwrap().insert("pichu_specialupdummy","e2");
        WEAPON_TABLE.lock().unwrap().insert("pichu_vortex","e0");
        WEAPON_TABLE.lock().unwrap().insert("pickel_axe","23b");
        WEAPON_TABLE.lock().unwrap().insert("pickel_building","237");
        WEAPON_TABLE.lock().unwrap().insert("pickel_crack","230");
        WEAPON_TABLE.lock().unwrap().insert("pickel_entryobject","23f");
        WEAPON_TABLE.lock().unwrap().insert("pickel_fence","242");
        WEAPON_TABLE.lock().unwrap().insert("pickel_fire","243");
        WEAPON_TABLE.lock().unwrap().insert("pickel_fishingrod","240");
        WEAPON_TABLE.lock().unwrap().insert("pickel_forge","238");
        WEAPON_TABLE.lock().unwrap().insert("pickel_maskfinal","244");
        WEAPON_TABLE.lock().unwrap().insert("pickel_melt","241");
        WEAPON_TABLE.lock().unwrap().insert("pickel_pick","23c");
        WEAPON_TABLE.lock().unwrap().insert("pickel_plate","233");
        WEAPON_TABLE.lock().unwrap().insert("pickel_pushfinal","245");
        WEAPON_TABLE.lock().unwrap().insert("pickel_pushobject","23e");
        WEAPON_TABLE.lock().unwrap().insert("pickel_rail","232");
        WEAPON_TABLE.lock().unwrap().insert("pickel_scarier","239");
        WEAPON_TABLE.lock().unwrap().insert("pickel_shovel","23d");
        WEAPON_TABLE.lock().unwrap().insert("pickel_stone","234");
        WEAPON_TABLE.lock().unwrap().insert("pickel_stuff","236");
        WEAPON_TABLE.lock().unwrap().insert("pickel_sword","23a");
        WEAPON_TABLE.lock().unwrap().insert("pickel_table","235");
        WEAPON_TABLE.lock().unwrap().insert("pickel_trolley","231");
        WEAPON_TABLE.lock().unwrap().insert("pickel_wing","22f");
        WEAPON_TABLE.lock().unwrap().insert("pikachu_cloud","4f");
        WEAPON_TABLE.lock().unwrap().insert("pikachu_dengeki","4d");
        WEAPON_TABLE.lock().unwrap().insert("pikachu_dengekidama","4c");
        WEAPON_TABLE.lock().unwrap().insert("pikachu_kaminari","4e");
        WEAPON_TABLE.lock().unwrap().insert("pikachu_monsterball","51");
        WEAPON_TABLE.lock().unwrap().insert("pikachu_specialupdummy","52");
        WEAPON_TABLE.lock().unwrap().insert("pikachu_vortex","50");
        WEAPON_TABLE.lock().unwrap().insert("pikmin_dolfin","9e");
        WEAPON_TABLE.lock().unwrap().insert("pikmin_pikmin","9d");
        WEAPON_TABLE.lock().unwrap().insert("pikmin_win1","9f");
        WEAPON_TABLE.lock().unwrap().insert("pikmin_win2","a0");
        WEAPON_TABLE.lock().unwrap().insert("pikmin_win3","a1");
        WEAPON_TABLE.lock().unwrap().insert("pitb_bow","e4");
        WEAPON_TABLE.lock().unwrap().insert("pitb_bowarrow","e5");
        WEAPON_TABLE.lock().unwrap().insert("pit_bow","90");
        WEAPON_TABLE.lock().unwrap().insert("pit_bowarrow","91");
        WEAPON_TABLE.lock().unwrap().insert("pit_chariot","92");
        WEAPON_TABLE.lock().unwrap().insert("pit_chariotsight","94");
        WEAPON_TABLE.lock().unwrap().insert("pit_horse","93");
        WEAPON_TABLE.lock().unwrap().insert("plizardon_breath","ce");
        WEAPON_TABLE.lock().unwrap().insert("plizardon_daimonji","d0");
        WEAPON_TABLE.lock().unwrap().insert("plizardon_explosion","cf");
        WEAPON_TABLE.lock().unwrap().insert("popo_blizzard","17a");
        WEAPON_TABLE.lock().unwrap().insert("popo_condor","179");
        WEAPON_TABLE.lock().unwrap().insert("popo_iceberg","17d");
        WEAPON_TABLE.lock().unwrap().insert("popo_iceberg_hit","17f");
        WEAPON_TABLE.lock().unwrap().insert("popo_iceberg_wind","17e");
        WEAPON_TABLE.lock().unwrap().insert("popo_iceshot","17c");
        WEAPON_TABLE.lock().unwrap().insert("popo_rubber","17b");
        WEAPON_TABLE.lock().unwrap().insert("popo_whitebear","180");
        WEAPON_TABLE.lock().unwrap().insert("ptrainer_mball","1cb");
        WEAPON_TABLE.lock().unwrap().insert("ptrainer_pfushigisou","1cd");
        WEAPON_TABLE.lock().unwrap().insert("ptrainer_plizardon","1ce");
        WEAPON_TABLE.lock().unwrap().insert("ptrainer_ptrainer","1ca");
        WEAPON_TABLE.lock().unwrap().insert("ptrainer_pzenigame","1cc");
        WEAPON_TABLE.lock().unwrap().insert("purin_cap","d5");
        WEAPON_TABLE.lock().unwrap().insert("purin_monsterball","d6");
        WEAPON_TABLE.lock().unwrap().insert("pzenigame_water","181");
        WEAPON_TABLE.lock().unwrap().insert("reflet_book","118");
        WEAPON_TABLE.lock().unwrap().insert("reflet_chrom","11d");
        WEAPON_TABLE.lock().unwrap().insert("reflet_elwind","11b");
        WEAPON_TABLE.lock().unwrap().insert("reflet_gigafire","11c");
        WEAPON_TABLE.lock().unwrap().insert("reflet_thunder","11a");
        WEAPON_TABLE.lock().unwrap().insert("reflet_window","119");
        WEAPON_TABLE.lock().unwrap().insert("richter_axe","1bf");
        WEAPON_TABLE.lock().unwrap().insert("richter_coffin","1c1");
        WEAPON_TABLE.lock().unwrap().insert("richter_cross","1c0");
        WEAPON_TABLE.lock().unwrap().insert("richter_crystal","1c2");
        WEAPON_TABLE.lock().unwrap().insert("richter_stake","1c6");
        WEAPON_TABLE.lock().unwrap().insert("richter_whip","1be");
        WEAPON_TABLE.lock().unwrap().insert("richter_whip2","1c3");
        WEAPON_TABLE.lock().unwrap().insert("richter_whiphand","1c4");
        WEAPON_TABLE.lock().unwrap().insert("richter_whipwire","1c5");
        WEAPON_TABLE.lock().unwrap().insert("ridley_breath","1b7");
        WEAPON_TABLE.lock().unwrap().insert("ridley_gunship","1b8");
        WEAPON_TABLE.lock().unwrap().insert("robot_beam","b9");
        WEAPON_TABLE.lock().unwrap().insert("robot_final_beam","ba");
        WEAPON_TABLE.lock().unwrap().insert("robot_gyro","b7");
        WEAPON_TABLE.lock().unwrap().insert("robot_gyro_holder","b8");
        WEAPON_TABLE.lock().unwrap().insert("robot_hominglaser","bf");
        WEAPON_TABLE.lock().unwrap().insert("robot_homingtarget","c0");
        WEAPON_TABLE.lock().unwrap().insert("robot_hugebeam","bb");
        WEAPON_TABLE.lock().unwrap().insert("robot_mainlaser","be");
        WEAPON_TABLE.lock().unwrap().insert("robot_narrowbeam","bc");
        WEAPON_TABLE.lock().unwrap().insert("robot_widebeam","bd");
        WEAPON_TABLE.lock().unwrap().insert("rockman_airshooter","154");
        WEAPON_TABLE.lock().unwrap().insert("rockman_blackhole","14b");
        WEAPON_TABLE.lock().unwrap().insert("rockman_bruce","150");
        WEAPON_TABLE.lock().unwrap().insert("rockman_chargeshot","149");
        WEAPON_TABLE.lock().unwrap().insert("rockman_crashbomb","146");
        WEAPON_TABLE.lock().unwrap().insert("rockman_forte","151");
        WEAPON_TABLE.lock().unwrap().insert("rockman_hardknuckle","14a");
        WEAPON_TABLE.lock().unwrap().insert("rockman_leafshield","148");
        WEAPON_TABLE.lock().unwrap().insert("rockman_leftarm","152");
        WEAPON_TABLE.lock().unwrap().insert("rockman_rightarm","153");
        WEAPON_TABLE.lock().unwrap().insert("rockman_rockbuster","145");
        WEAPON_TABLE.lock().unwrap().insert("rockman_rockmandash","14d");
        WEAPON_TABLE.lock().unwrap().insert("rockman_rockmanexe","14e");
        WEAPON_TABLE.lock().unwrap().insert("rockman_rockmanx","14c");
        WEAPON_TABLE.lock().unwrap().insert("rockman_rushcoil","147");
        WEAPON_TABLE.lock().unwrap().insert("rockman_shootingstarrockman","14f");
        WEAPON_TABLE.lock().unwrap().insert("rosetta_meteor","eb");
        WEAPON_TABLE.lock().unwrap().insert("rosetta_pointer","e9");
        WEAPON_TABLE.lock().unwrap().insert("rosetta_powerstar","ea");
        WEAPON_TABLE.lock().unwrap().insert("rosetta_ring","e8");
        WEAPON_TABLE.lock().unwrap().insert("rosetta_starpiece","e7");
        WEAPON_TABLE.lock().unwrap().insert("rosetta_tico","e6");
        WEAPON_TABLE.lock().unwrap().insert("roy_sword","16b");
        WEAPON_TABLE.lock().unwrap().insert("ryu_hadoken","15b");
        WEAPON_TABLE.lock().unwrap().insert("ryu_sack","15d");
        WEAPON_TABLE.lock().unwrap().insert("ryu_shinkuhadoken","15c");
        WEAPON_TABLE.lock().unwrap().insert("samusd_bomb","2f");
        WEAPON_TABLE.lock().unwrap().insert("samusd_bunshin","37");
        WEAPON_TABLE.lock().unwrap().insert("samusd_cshot","2e");
        WEAPON_TABLE.lock().unwrap().insert("samusd_gbeam","36");
        WEAPON_TABLE.lock().unwrap().insert("samusd_gun","33");
        WEAPON_TABLE.lock().unwrap().insert("samusd_laser","31");
        WEAPON_TABLE.lock().unwrap().insert("samusd_laser2","32");
        WEAPON_TABLE.lock().unwrap().insert("samusd_missile","30");
        WEAPON_TABLE.lock().unwrap().insert("samusd_supermissile","34");
        WEAPON_TABLE.lock().unwrap().insert("samusd_transportation","35");
        WEAPON_TABLE.lock().unwrap().insert("samus_bomb","26");
        WEAPON_TABLE.lock().unwrap().insert("samus_cshot","25");
        WEAPON_TABLE.lock().unwrap().insert("samus_gbeam","2d");
        WEAPON_TABLE.lock().unwrap().insert("samus_gun","2a");
        WEAPON_TABLE.lock().unwrap().insert("samus_laser","28");
        WEAPON_TABLE.lock().unwrap().insert("samus_laser2","29");
        WEAPON_TABLE.lock().unwrap().insert("samus_missile","27");
        WEAPON_TABLE.lock().unwrap().insert("samus_supermissile","2b");
        WEAPON_TABLE.lock().unwrap().insert("samus_transportation","2c");
        WEAPON_TABLE.lock().unwrap().insert("sheik_fusin","75");
        WEAPON_TABLE.lock().unwrap().insert("sheik_knife","76");
        WEAPON_TABLE.lock().unwrap().insert("sheik_needle","73");
        WEAPON_TABLE.lock().unwrap().insert("sheik_needlehave","74");
        WEAPON_TABLE.lock().unwrap().insert("shizue_balloon","1d7");
        WEAPON_TABLE.lock().unwrap().insert("shizue_broom","1e0");
        WEAPON_TABLE.lock().unwrap().insert("shizue_bucket","1e2");
        WEAPON_TABLE.lock().unwrap().insert("shizue_bullet","1d6");
        WEAPON_TABLE.lock().unwrap().insert("shizue_butterflynet","1d8");
        WEAPON_TABLE.lock().unwrap().insert("shizue_clayrocket","1e5");
        WEAPON_TABLE.lock().unwrap().insert("shizue_cracker","1e1");
        WEAPON_TABLE.lock().unwrap().insert("shizue_fishingline","1e7");
        WEAPON_TABLE.lock().unwrap().insert("shizue_fishingrod","1e6");
        WEAPON_TABLE.lock().unwrap().insert("shizue_furniture","1dd");
        WEAPON_TABLE.lock().unwrap().insert("shizue_moneybag","1de");
        WEAPON_TABLE.lock().unwrap().insert("shizue_office","1d9");
        WEAPON_TABLE.lock().unwrap().insert("shizue_picopicohammer","1df");
        WEAPON_TABLE.lock().unwrap().insert("shizue_pompon","1e3");
        WEAPON_TABLE.lock().unwrap().insert("shizue_pot","1d1");
        WEAPON_TABLE.lock().unwrap().insert("shizue_slingshot","1d5");
        WEAPON_TABLE.lock().unwrap().insert("shizue_swing","1e4");
        WEAPON_TABLE.lock().unwrap().insert("shizue_timmy","1dc");
        WEAPON_TABLE.lock().unwrap().insert("shizue_tommy","1db");
        WEAPON_TABLE.lock().unwrap().insert("shizue_tomnook","1da");
        WEAPON_TABLE.lock().unwrap().insert("shizue_trafficsign","1d4");
        WEAPON_TABLE.lock().unwrap().insert("shizue_umbrella","1d2");
        WEAPON_TABLE.lock().unwrap().insert("shizue_weeds","1d3");
        WEAPON_TABLE.lock().unwrap().insert("shulk_dunban","134");
        WEAPON_TABLE.lock().unwrap().insert("shulk_fiora","136");
        WEAPON_TABLE.lock().unwrap().insert("shulk_riki","135");
        WEAPON_TABLE.lock().unwrap().insert("simon_axe","1ae");
        WEAPON_TABLE.lock().unwrap().insert("simon_coffin","1b0");
        WEAPON_TABLE.lock().unwrap().insert("simon_cross","1af");
        WEAPON_TABLE.lock().unwrap().insert("simon_crystal","1b1");
        WEAPON_TABLE.lock().unwrap().insert("simon_stake","1b5");
        WEAPON_TABLE.lock().unwrap().insert("simon_whip","1ad");
        WEAPON_TABLE.lock().unwrap().insert("simon_whip2","1b2");
        WEAPON_TABLE.lock().unwrap().insert("simon_whiphand","1b3");
        WEAPON_TABLE.lock().unwrap().insert("simon_whipwire","1b4");
        WEAPON_TABLE.lock().unwrap().insert("snake_c4","197");
        WEAPON_TABLE.lock().unwrap().insert("snake_c4_switch","198");
        WEAPON_TABLE.lock().unwrap().insert("snake_cypher","196");
        WEAPON_TABLE.lock().unwrap().insert("snake_flare_grenades","19a");
        WEAPON_TABLE.lock().unwrap().insert("snake_grenade","199");
        WEAPON_TABLE.lock().unwrap().insert("snake_lock_on_cursor","19d");
        WEAPON_TABLE.lock().unwrap().insert("snake_lock_on_cursor_ready","19e");
        WEAPON_TABLE.lock().unwrap().insert("snake_missile","19f");
        WEAPON_TABLE.lock().unwrap().insert("snake_nikita","194");
        WEAPON_TABLE.lock().unwrap().insert("snake_nikita_missile","195");
        WEAPON_TABLE.lock().unwrap().insert("snake_reticle","19b");
        WEAPON_TABLE.lock().unwrap().insert("snake_reticle_cursor","19c");
        WEAPON_TABLE.lock().unwrap().insert("snake_rpg7","191");
        WEAPON_TABLE.lock().unwrap().insert("snake_trenchmortar","192");
        WEAPON_TABLE.lock().unwrap().insert("snake_trenchmortar_bullet","193");
        WEAPON_TABLE.lock().unwrap().insert("sonic_chaosemerald","d4");
        WEAPON_TABLE.lock().unwrap().insert("sonic_gimmickjump","d2");
        WEAPON_TABLE.lock().unwrap().insert("sonic_homingtarget","d1");
        WEAPON_TABLE.lock().unwrap().insert("sonic_supersonic","d3");
        WEAPON_TABLE.lock().unwrap().insert("szerosuit_gunship","99");
        WEAPON_TABLE.lock().unwrap().insert("szerosuit_laser","9b");
        WEAPON_TABLE.lock().unwrap().insert("szerosuit_paralyzer","97");
        WEAPON_TABLE.lock().unwrap().insert("szerosuit_paralyzer_bullet","95");
        WEAPON_TABLE.lock().unwrap().insert("szerosuit_reticle","9a");
        WEAPON_TABLE.lock().unwrap().insert("szerosuit_samusp","98");
        WEAPON_TABLE.lock().unwrap().insert("szerosuit_whip","96");
        WEAPON_TABLE.lock().unwrap().insert("szerosuit_whip2","9c");
        WEAPON_TABLE.lock().unwrap().insert("tantan_ally1","21d");
        WEAPON_TABLE.lock().unwrap().insert("tantan_ally2","21e");
        WEAPON_TABLE.lock().unwrap().insert("tantan_ally3","21f");
        WEAPON_TABLE.lock().unwrap().insert("tantan_ally4","220");
        WEAPON_TABLE.lock().unwrap().insert("tantan_ally5","221");
        WEAPON_TABLE.lock().unwrap().insert("tantan_ally6","222");
        WEAPON_TABLE.lock().unwrap().insert("tantan_allyarm1","223");
        WEAPON_TABLE.lock().unwrap().insert("tantan_allyarm2","224");
        WEAPON_TABLE.lock().unwrap().insert("tantan_allyarm3","225");
        WEAPON_TABLE.lock().unwrap().insert("tantan_allyarm5","226");
        WEAPON_TABLE.lock().unwrap().insert("tantan_allyarm6","227");
        WEAPON_TABLE.lock().unwrap().insert("tantan_allyarmbullet1","228");
        WEAPON_TABLE.lock().unwrap().insert("tantan_allyarmbullet2","229");
        WEAPON_TABLE.lock().unwrap().insert("tantan_allyarmbullet3","22a");
        WEAPON_TABLE.lock().unwrap().insert("tantan_beam","21c");
        WEAPON_TABLE.lock().unwrap().insert("tantan_gongfinal","22c");
        WEAPON_TABLE.lock().unwrap().insert("tantan_punch1","218");
        WEAPON_TABLE.lock().unwrap().insert("tantan_punch2","219");
        WEAPON_TABLE.lock().unwrap().insert("tantan_punch3","21a");
        WEAPON_TABLE.lock().unwrap().insert("tantan_ring","21b");
        WEAPON_TABLE.lock().unwrap().insert("tantan_spiralleft","216");
        WEAPON_TABLE.lock().unwrap().insert("tantan_spiralleftloupe","22d");
        WEAPON_TABLE.lock().unwrap().insert("tantan_spiralright","217");
        WEAPON_TABLE.lock().unwrap().insert("tantan_spiralrightloupe","22e");
        WEAPON_TABLE.lock().unwrap().insert("tantan_spiralsimple","22b");
        WEAPON_TABLE.lock().unwrap().insert("term","267");
        WEAPON_TABLE.lock().unwrap().insert("toonlink_boomerang","c1");
        WEAPON_TABLE.lock().unwrap().insert("toonlink_bow","c2");
        WEAPON_TABLE.lock().unwrap().insert("toonlink_bowarrow","c3");
        WEAPON_TABLE.lock().unwrap().insert("toonlink_fairy","c6");
        WEAPON_TABLE.lock().unwrap().insert("toonlink_hookshot","c4");
        WEAPON_TABLE.lock().unwrap().insert("toonlink_hookshot_hand","c5");
        WEAPON_TABLE.lock().unwrap().insert("toonlink_pig","c8");
        WEAPON_TABLE.lock().unwrap().insert("toonlink_takt","c7");
        WEAPON_TABLE.lock().unwrap().insert("wario_garlic","8b");
        WEAPON_TABLE.lock().unwrap().insert("wario_wariobike","8a");
        WEAPON_TABLE.lock().unwrap().insert("wario_warioman","8c");
        WEAPON_TABLE.lock().unwrap().insert("wiifit_balanceboard","ee");
        WEAPON_TABLE.lock().unwrap().insert("wiifit_hulahoop","ec");
        WEAPON_TABLE.lock().unwrap().insert("wiifit_silhouette","f1");
        WEAPON_TABLE.lock().unwrap().insert("wiifit_silhouettel","f2");
        WEAPON_TABLE.lock().unwrap().insert("wiifit_sunbullet","ed");
        WEAPON_TABLE.lock().unwrap().insert("wiifit_towel","f0");
        WEAPON_TABLE.lock().unwrap().insert("wiifit_wiibo","ef");
        WEAPON_TABLE.lock().unwrap().insert("wolf_blaster","c9");
        WEAPON_TABLE.lock().unwrap().insert("wolf_blaster_bullet","ca");
        WEAPON_TABLE.lock().unwrap().insert("wolf_illusion","cb");
        WEAPON_TABLE.lock().unwrap().insert("wolf_reticle","cd");
        WEAPON_TABLE.lock().unwrap().insert("wolf_wolfen","cc");
        WEAPON_TABLE.lock().unwrap().insert("yoshi_star","38");
        WEAPON_TABLE.lock().unwrap().insert("yoshi_tamago","39");
        WEAPON_TABLE.lock().unwrap().insert("yoshi_yoshibg01","3b");
        WEAPON_TABLE.lock().unwrap().insert("yoshi_yoshimob","3a");
        WEAPON_TABLE.lock().unwrap().insert("younglink_boomerang","1a6");
        WEAPON_TABLE.lock().unwrap().insert("younglink_bow","1a7");
        WEAPON_TABLE.lock().unwrap().insert("younglink_bowarrow","1a8");
        WEAPON_TABLE.lock().unwrap().insert("younglink_hookshot","1a9");
        WEAPON_TABLE.lock().unwrap().insert("younglink_hookshot_hand","1aa");
        WEAPON_TABLE.lock().unwrap().insert("younglink_milk","1ac");
        WEAPON_TABLE.lock().unwrap().insert("younglink_navy","1ab");
        WEAPON_TABLE.lock().unwrap().insert("zelda_dein","6f");
        WEAPON_TABLE.lock().unwrap().insert("zelda_dein_s","70");
        WEAPON_TABLE.lock().unwrap().insert("zelda_phantom","71");
        WEAPON_TABLE.lock().unwrap().insert("zelda_triforce","72");
}
pub fn get_weapon_kind_from_string(target_kind: &str) -> i32 {
    
    let lowercased=target_kind.to_lowercase();
    if let Some(hex) = WEAPON_TABLE.lock().unwrap().get(lowercased.as_str()){
        let int = i64::from_str_radix(hex, 16);
        let result = (int.unwrap()*-1) as i32;
        //println!("{} > {}",target_kind,result);
        return result;
    }
    return -999;
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
/* 
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
}*/

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
        build_fighter_table();
        build_weapon_table();
        find_folders()
    })
    .unwrap()
    .join();

    return folder_thread.unwrap();
}