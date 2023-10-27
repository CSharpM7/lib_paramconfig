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

const IDENTIFIER: &str = "config_param.toml";

pub fn get_fighter_kind_from_string(target_kind: &str) -> i32 {
    let kind_map = HashMap::from([
    //("all","-1"),
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
    return 999;
}
pub fn get_weapon_kind_from_string(target_kind: &str) -> i32 {
    let kind_map = HashMap::from([
        ("bayonetta_bat","171"),
        ("bayonetta_gomorrah","172"),
        ("bayonetta_hair","173"),
        ("bayonetta_specialn_bullet","16e"),
        ("bayonetta_wickedweavearm","16f"),
        ("bayonetta_wickedweaveleg","170"),
        ("brave_blue","1fe"),
        ("brave_crash","1fc"),
        ("brave_deathball","1fb"),
        ("brave_explosion","1f9"),
        ("brave_fireball","1f5"),
        ("brave_flash","1fa"),
        ("brave_lightning","1f7"),
        ("brave_sleep","1fd"),
        ("brave_spark","1f6"),
        ("brave_tornado","1f8"),
        ("buddy_bigbird","202"),
        ("buddy_bigbirdbase","203"),
        ("buddy_bird","201"),
        ("buddy_bullet","204"),
        ("buddy_horn","207"),
        ("buddy_pad","1ff"),
        ("buddy_partner","200"),
        ("buddy_piece","205"),
        ("buddy_strings","206"),
        ("captain_bluefalcon","57"),
        ("captain_falconpunch","58"),
        ("chrom_sword","1b6"),
        ("cloud_wave","16c"),
        ("daisy_kassar","6a"),
        ("daisy_kinopio","6b"),
        ("daisy_kinopiospore","6c"),
        ("dedede_gordo","ad"),
        ("dedede_jethammer","aa"),
        ("dedede_mask","b1"),
        ("dedede_missile","b2"),
        ("dedede_newdededehammer","b0"),
        ("dedede_shrine","ae"),
        ("dedede_star","ac"),
        ("dedede_star_missile","ab"),
        ("dedede_waddledee","af"),
        ("demon_blaster","259"),
        ("demon_blasterchest","25b"),
        ("demon_blasterhead","25c"),
        ("demon_blasterwing","25d"),
        ("demon_demonp","25a"),
        ("diddy_barreljet","a3"),
        ("diddy_barreljets","a6"),
        ("diddy_bunshin","a7"),
        ("diddy_dkbarrel","a8"),
        ("diddy_explosion","a5"),
        ("diddy_gun","a2"),
        ("diddy_lock_on_cursor","a9"),
        ("diddy_peanuts","a4"),
        ("dolly_burst","209"),
        ("dolly_cap","20a"),
        ("dolly_fire","20b"),
        ("dolly_wave","208"),
        ("donkey_dkbarrel","1c"),
        ("duckhunt_can","121"),
        ("duckhunt_clay","120"),
        ("duckhunt_finalbird","128"),
        ("duckhunt_finalcan","12a"),
        ("duckhunt_finaldog","127"),
        ("duckhunt_finalduck","124"),
        ("duckhunt_finalenemy","126"),
        ("duckhunt_finalgrass","129"),
        ("duckhunt_finalgunman","125"),
        ("duckhunt_grass","12b"),
        ("duckhunt_gunman","11e"),
        ("duckhunt_gunmanbullet","11f"),
        ("duckhunt_kurofukuhat","123"),
        ("duckhunt_reticle","122"),
        ("edge_background","24b"),
        ("edge_fire","246"),
        ("edge_flare1","248"),
        ("edge_flare2","249"),
        ("edge_flaredummy","24a"),
        ("edge_flash","247"),
        ("eflame_blazepillar","24e"),
        ("eflame_esword","24c"),
        ("eflame_firepillar","24d"),
        ("eflame_windummy","24f"),
        ("eflame_winsword","250"),
        ("element_changer","1cf"),
        ("element_diver","1d0"),
        ("elight_beam","256"),
        ("elight_bunshin","252"),
        ("elight_esword","251"),
        ("elight_exprosiveshot","253"),
        ("elight_meteor","255"),
        ("elight_spreadbullet","254"),
        ("elight_windummy","257"),
        ("elight_winsword","258"),
        ("enemy_yellowdevil_beam","1c7"),
        ("falco_arwing","87"),
        ("falco_arwingshot","89"),
        ("falco_blaster","84"),
        ("falco_blaster_bullet","85"),
        ("falco_illusion","86"),
        ("falco_reticle","88"),
        ("fox_arwing","5c"),
        ("fox_arwingshot","5e"),
        ("fox_blaster","59"),
        ("fox_blaster_bullet","5a"),
        ("fox_illusion","5b"),
        ("fox_reticle","5d"),
        ("gamewatch_bomb","80"),
        ("gamewatch_breath","7e"),
        ("gamewatch_entry","7f"),
        ("gamewatch_food","77"),
        ("gamewatch_normal_weapon","7d"),
        ("gamewatch_octopus","7c"),
        ("gamewatch_oil","7b"),
        ("gamewatch_panel","7a"),
        ("gamewatch_parachute","79"),
        ("gamewatch_rescue","78"),
        ("ganon_beast","82"),
        ("ganon_ganond","83"),
        ("ganon_sword","81"),
        ("gaogaen_championbelt","1bb"),
        ("gaogaen_monsterball","1bc"),
        ("gaogaen_rope","1ba"),
        ("gaogaen_rope2","1bd"),
        ("gekkouga_bunshin","13c"),
        ("gekkouga_gekkougas","13d"),
        ("gekkouga_monsterball","13b"),
        ("gekkouga_moon","13a"),
        ("gekkouga_shuriken","137"),
        ("gekkouga_tatami","139"),
        ("gekkouga_water","138"),
        ("ike_sword","b3"),
        ("inkling_blaster","18b"),
        ("inkling_brush","18a"),
        ("inkling_copy_inklinggun","18f"),
        ("inkling_copy_inklingtank","190"),
        ("inkling_inkbullet","185"),
        ("inkling_megaphonelaser","18e"),
        ("inkling_roller","188"),
        ("inkling_rollerink","189"),
        ("inkling_slosher","18c"),
        ("inkling_splash","18d"),
        ("inkling_splashbomb","186"),
        ("inkling_squid","187"),
        ("jack_background","1f4"),
        ("jack_bus","1f3"),
        ("jack_doyle","1ec"),
        ("jack_fire","1ed"),
        ("jack_fire2","1ee"),
        ("jack_mona","1ef"),
        ("jack_windummy","1f2"),
        ("jack_wing","1f0"),
        ("jack_wirerope","1f1"),
        ("kamui_dragonhand","175"),
        ("kamui_ryusensya","174"),
        ("kamui_spearhand","176"),
        ("kamui_waterdragon","177"),
        ("kamui_waterstream","178"),
        ("ken_hadoken","15e"),
        ("ken_shinkuhadoken","15f"),
        ("ken_shinryuken","160"),
        ("kirby_finalcutter","4b"),
        ("kirby_finalcuttershot","3d"),
        ("kirby_hammer","3c"),
        ("kirby_hat","3f"),
        ("kirby_miipartshead","45"),
        ("kirby_reserve","43"),
        ("kirby_rosettaticomissile","46"),
        ("kirby_simple","44"),
        ("kirby_simple2l","49"),
        ("kirby_simple2r","48"),
        ("kirby_starmissile","3e"),
        ("kirby_stone","47"),
        ("kirby_ultrasword","40"),
        ("kirby_ultraswordhat","41"),
        ("kirby_warpstar","42"),
        ("kirby_windummy","4a"),
        ("koopag_breath","1b9"),
        ("koopajr_batten","133"),
        ("koopajr_cannonball","12f"),
        ("koopajr_hammer","12c"),
        ("koopajr_kart","130"),
        ("koopajr_magichand","12e"),
        ("koopajr_picopicohammer","12d"),
        ("koopajr_remainclown","131"),
        ("koopajr_shadowmario","132"),
        ("koopa_breath","6d"),
        ("koopa_koopag","6e"),
        ("krool_backpack","1a0"),
        ("krool_blunderbuss","1a3"),
        ("krool_crown","1a1"),
        ("krool_ironball","1a4"),
        ("krool_piratehat","1a2"),
        ("krool_spitball","1a5"),
        ("link_ancient_bow","22"),
        ("link_ancient_bowarrow","23"),
        ("link_boomerang","1d"),
        ("link_bow","1e"),
        ("link_bowarrow","1f"),
        ("link_navy","20"),
        ("link_parasail","24"),
        ("link_sword_beam","21"),
        ("littlemac_championbelt","f6"),
        ("littlemac_doclouis","f3"),
        ("littlemac_littlemacg","f7"),
        ("littlemac_sweatlittlemac","f4"),
        ("littlemac_throwsweat","f5"),
        ("lucario_auraball","b4"),
        ("lucario_lucariom","b6"),
        ("lucario_qigong","b5"),
        ("lucas_bonnie","16a"),
        ("lucas_doseitable","167"),
        ("lucas_himohebi","165"),
        ("lucas_himohebi2","166"),
        ("lucas_kumatora","169"),
        ("lucas_needle","168"),
        ("lucas_pk_fire","162"),
        ("lucas_pk_freeze","161"),
        ("lucas_pk_starstorm","164"),
        ("lucas_pk_thunder","163"),
        ("lucina_mask","e3"),
        ("luigi_dokan","54"),
        ("luigi_fireball","53"),
        ("luigi_obakyumu","55"),
        ("luigi_plunger","56"),
        ("mariod_capsuleblock","db"),
        ("mariod_drcapsule","d7"),
        ("mariod_drmantle","d8"),
        ("mariod_huge_capsule","da"),
        ("mariod_stethoscope","d9"),
        ("mario_cappy","1b"),
        ("mario_dokan","1a"),
        ("mario_fireball","15"),
        ("mario_huge_flame","19"),
        ("mario_mantle","16"),
        ("mario_pump","17"),
        ("mario_pump_water","18"),
        ("master_arrow1","20e"),
        ("master_arrow2","20f"),
        ("master_axe","20c"),
        ("master_background","213"),
        ("master_baton","212"),
        ("master_bow","20d"),
        ("master_spear","210"),
        ("master_sword","211"),
        ("master_sword2","214"),
        ("master_swordflare","215"),
        ("metaknight_bunshin","8e"),
        ("metaknight_fourwings","8f"),
        ("metaknight_mantle","8d"),
        ("mewtwo_bindball","156"),
        ("mewtwo_escapeairdummy","15a"),
        ("mewtwo_mewtwom","157"),
        ("mewtwo_psychobreak","159"),
        ("mewtwo_search","158"),
        ("mewtwo_shadowball","155"),
        ("miienemyg_attackairf_bullet","1c8"),
        ("miienemyg_rapidshot_bullet","1c9"),
        ("miifighter_hat","0"),
        ("miifighter_ironball","1"),
        ("miigunner_attackairf_bullet","12"),
        ("miigunner_bottomshoot","10"),
        ("miigunner_flamepillar","d"),
        ("miigunner_fullthrottle","14"),
        ("miigunner_grenadelauncher","c"),
        ("miigunner_groundbomb","11"),
        ("miigunner_gunnercharge","9"),
        ("miigunner_hat","7"),
        ("miigunner_laser","13"),
        ("miigunner_miimissile","a"),
        ("miigunner_rapidshot_bullet","8"),
        ("miigunner_stealthbomb","e"),
        ("miigunner_stealthbomb_s","f"),
        ("miigunner_supermissile","b"),
        ("miiswordsman_chakram","4"),
        ("miiswordsman_hat","2"),
        ("miiswordsman_lightshuriken","3"),
        ("miiswordsman_tornadoshot","5"),
        ("miiswordsman_wave","6"),
        ("murabito_balloon","100"),
        ("murabito_beetle","10e"),
        ("murabito_bowling_ball","fa"),
        ("murabito_bullet","fd"),
        ("murabito_butterflynet","ff"),
        ("murabito_clayrocket","101"),
        ("murabito_firework","fb"),
        ("murabito_flowerpot","f8"),
        ("murabito_furniture","10c"),
        ("murabito_helmet","107"),
        ("murabito_house","10b"),
        ("murabito_moneybag","10d"),
        ("murabito_seed","102"),
        ("murabito_slingshot","fc"),
        ("murabito_sprinkling_water","106"),
        ("murabito_sprout","103"),
        ("murabito_stump","105"),
        ("murabito_timmy","10a"),
        ("murabito_tommy","109"),
        ("murabito_tomnook","108"),
        ("murabito_tree","104"),
        ("murabito_umbrella","f9"),
        ("murabito_weeds","fe"),
        ("ness_paula","63"),
        ("ness_pk_fire","60"),
        ("ness_pk_flash","5f"),
        ("ness_pk_starstorm","62"),
        ("ness_pk_thunder","61"),
        ("ness_poo","64"),
        ("ness_yoyo","65"),
        ("ness_yoyo_head","66"),
        ("none","ffffffff"),
        ("packun_bosspackun","1eb"),
        ("packun_mario","1ea"),
        ("packun_poisonbreath","1e9"),
        ("packun_spikeball","1e8"),
        ("pacman_artisticpoint","143"),
        ("pacman_bigpacman","142"),
        ("pacman_esa","13e"),
        ("pacman_fairy","144"),
        ("pacman_firehydrant","140"),
        ("pacman_firehydrant_water","141"),
        ("pacman_trampoline","13f"),
        ("palutena_autoaimbullet","112"),
        ("palutena_autoreticle","113"),
        ("palutena_beam","116"),
        ("palutena_blackhole","115"),
        ("palutena_explosiveflame","110"),
        ("palutena_explosiveflame_reserve","111"),
        ("palutena_gate","117"),
        ("palutena_godwing","10f"),
        ("palutena_reflectionboard","114"),
        ("peach_kassar","67"),
        ("peach_kinopio","68"),
        ("peach_kinopiospore","69"),
        ("pfushigisou_leafcutter","183"),
        ("pfushigisou_seed","182"),
        ("pfushigisou_vine","184"),
        ("pichu_cloud","df"),
        ("pichu_dengeki","dd"),
        ("pichu_dengekidama","dc"),
        ("pichu_kaminari","de"),
        ("pichu_monsterball","e1"),
        ("pichu_specialupdummy","e2"),
        ("pichu_vortex","e0"),
        ("pickel_axe","23b"),
        ("pickel_building","237"),
        ("pickel_crack","230"),
        ("pickel_entryobject","23f"),
        ("pickel_fence","242"),
        ("pickel_fire","243"),
        ("pickel_fishingrod","240"),
        ("pickel_forge","238"),
        ("pickel_maskfinal","244"),
        ("pickel_melt","241"),
        ("pickel_pick","23c"),
        ("pickel_plate","233"),
        ("pickel_pushfinal","245"),
        ("pickel_pushobject","23e"),
        ("pickel_rail","232"),
        ("pickel_scarier","239"),
        ("pickel_shovel","23d"),
        ("pickel_stone","234"),
        ("pickel_stuff","236"),
        ("pickel_sword","23a"),
        ("pickel_table","235"),
        ("pickel_trolley","231"),
        ("pickel_wing","22f"),
        ("pikachu_cloud","4f"),
        ("pikachu_dengeki","4d"),
        ("pikachu_dengekidama","4c"),
        ("pikachu_kaminari","4e"),
        ("pikachu_monsterball","51"),
        ("pikachu_specialupdummy","52"),
        ("pikachu_vortex","50"),
        ("pikmin_dolfin","9e"),
        ("pikmin_pikmin","9d"),
        ("pikmin_win1","9f"),
        ("pikmin_win2","a0"),
        ("pikmin_win3","a1"),
        ("pitb_bow","e4"),
        ("pitb_bowarrow","e5"),
        ("pit_bow","90"),
        ("pit_bowarrow","91"),
        ("pit_chariot","92"),
        ("pit_chariotsight","94"),
        ("pit_horse","93"),
        ("plizardon_breath","ce"),
        ("plizardon_daimonji","d0"),
        ("plizardon_explosion","cf"),
        ("popo_blizzard","17a"),
        ("popo_condor","179"),
        ("popo_iceberg","17d"),
        ("popo_iceberg_hit","17f"),
        ("popo_iceberg_wind","17e"),
        ("popo_iceshot","17c"),
        ("popo_rubber","17b"),
        ("popo_whitebear","180"),
        ("ptrainer_mball","1cb"),
        ("ptrainer_pfushigisou","1cd"),
        ("ptrainer_plizardon","1ce"),
        ("ptrainer_ptrainer","1ca"),
        ("ptrainer_pzenigame","1cc"),
        ("purin_cap","d5"),
        ("purin_monsterball","d6"),
        ("pzenigame_water","181"),
        ("reflet_book","118"),
        ("reflet_chrom","11d"),
        ("reflet_elwind","11b"),
        ("reflet_gigafire","11c"),
        ("reflet_thunder","11a"),
        ("reflet_window","119"),
        ("richter_axe","1bf"),
        ("richter_coffin","1c1"),
        ("richter_cross","1c0"),
        ("richter_crystal","1c2"),
        ("richter_stake","1c6"),
        ("richter_whip","1be"),
        ("richter_whip2","1c3"),
        ("richter_whiphand","1c4"),
        ("richter_whipwire","1c5"),
        ("ridley_breath","1b7"),
        ("ridley_gunship","1b8"),
        ("robot_beam","b9"),
        ("robot_final_beam","ba"),
        ("robot_gyro","b7"),
        ("robot_gyro_holder","b8"),
        ("robot_hominglaser","bf"),
        ("robot_homingtarget","c0"),
        ("robot_hugebeam","bb"),
        ("robot_mainlaser","be"),
        ("robot_narrowbeam","bc"),
        ("robot_widebeam","bd"),
        ("rockman_airshooter","154"),
        ("rockman_blackhole","14b"),
        ("rockman_bruce","150"),
        ("rockman_chargeshot","149"),
        ("rockman_crashbomb","146"),
        ("rockman_forte","151"),
        ("rockman_hardknuckle","14a"),
        ("rockman_leafshield","148"),
        ("rockman_leftarm","152"),
        ("rockman_rightarm","153"),
        ("rockman_rockbuster","145"),
        ("rockman_rockmandash","14d"),
        ("rockman_rockmanexe","14e"),
        ("rockman_rockmanx","14c"),
        ("rockman_rushcoil","147"),
        ("rockman_shootingstarrockman","14f"),
        ("rosetta_meteor","eb"),
        ("rosetta_pointer","e9"),
        ("rosetta_powerstar","ea"),
        ("rosetta_ring","e8"),
        ("rosetta_starpiece","e7"),
        ("rosetta_tico","e6"),
        ("roy_sword","16b"),
        ("ryu_hadoken","15b"),
        ("ryu_sack","15d"),
        ("ryu_shinkuhadoken","15c"),
        ("samusd_bomb","2f"),
        ("samusd_bunshin","37"),
        ("samusd_cshot","2e"),
        ("samusd_gbeam","36"),
        ("samusd_gun","33"),
        ("samusd_laser","31"),
        ("samusd_laser2","32"),
        ("samusd_missile","30"),
        ("samusd_supermissile","34"),
        ("samusd_transportation","35"),
        ("samus_bomb","26"),
        ("samus_cshot","25"),
        ("samus_gbeam","2d"),
        ("samus_gun","2a"),
        ("samus_laser","28"),
        ("samus_laser2","29"),
        ("samus_missile","27"),
        ("samus_supermissile","2b"),
        ("samus_transportation","2c"),
        ("sheik_fusin","75"),
        ("sheik_knife","76"),
        ("sheik_needle","73"),
        ("sheik_needlehave","74"),
        ("shizue_balloon","1d7"),
        ("shizue_broom","1e0"),
        ("shizue_bucket","1e2"),
        ("shizue_bullet","1d6"),
        ("shizue_butterflynet","1d8"),
        ("shizue_clayrocket","1e5"),
        ("shizue_cracker","1e1"),
        ("shizue_fishingline","1e7"),
        ("shizue_fishingrod","1e6"),
        ("shizue_furniture","1dd"),
        ("shizue_moneybag","1de"),
        ("shizue_office","1d9"),
        ("shizue_picopicohammer","1df"),
        ("shizue_pompon","1e3"),
        ("shizue_pot","1d1"),
        ("shizue_slingshot","1d5"),
        ("shizue_swing","1e4"),
        ("shizue_timmy","1dc"),
        ("shizue_tommy","1db"),
        ("shizue_tomnook","1da"),
        ("shizue_trafficsign","1d4"),
        ("shizue_umbrella","1d2"),
        ("shizue_weeds","1d3"),
        ("shulk_dunban","134"),
        ("shulk_fiora","136"),
        ("shulk_riki","135"),
        ("simon_axe","1ae"),
        ("simon_coffin","1b0"),
        ("simon_cross","1af"),
        ("simon_crystal","1b1"),
        ("simon_stake","1b5"),
        ("simon_whip","1ad"),
        ("simon_whip2","1b2"),
        ("simon_whiphand","1b3"),
        ("simon_whipwire","1b4"),
        ("snake_c4","197"),
        ("snake_c4_switch","198"),
        ("snake_cypher","196"),
        ("snake_flare_grenades","19a"),
        ("snake_grenade","199"),
        ("snake_lock_on_cursor","19d"),
        ("snake_lock_on_cursor_ready","19e"),
        ("snake_missile","19f"),
        ("snake_nikita","194"),
        ("snake_nikita_missile","195"),
        ("snake_reticle","19b"),
        ("snake_reticle_cursor","19c"),
        ("snake_rpg7","191"),
        ("snake_trenchmortar","192"),
        ("snake_trenchmortar_bullet","193"),
        ("sonic_chaosemerald","d4"),
        ("sonic_gimmickjump","d2"),
        ("sonic_homingtarget","d1"),
        ("sonic_supersonic","d3"),
        ("szerosuit_gunship","99"),
        ("szerosuit_laser","9b"),
        ("szerosuit_paralyzer","97"),
        ("szerosuit_paralyzer_bullet","95"),
        ("szerosuit_reticle","9a"),
        ("szerosuit_samusp","98"),
        ("szerosuit_whip","96"),
        ("szerosuit_whip2","9c"),
        ("tantan_ally1","21d"),
        ("tantan_ally2","21e"),
        ("tantan_ally3","21f"),
        ("tantan_ally4","220"),
        ("tantan_ally5","221"),
        ("tantan_ally6","222"),
        ("tantan_allyarm1","223"),
        ("tantan_allyarm2","224"),
        ("tantan_allyarm3","225"),
        ("tantan_allyarm5","226"),
        ("tantan_allyarm6","227"),
        ("tantan_allyarmbullet1","228"),
        ("tantan_allyarmbullet2","229"),
        ("tantan_allyarmbullet3","22a"),
        ("tantan_beam","21c"),
        ("tantan_gongfinal","22c"),
        ("tantan_punch1","218"),
        ("tantan_punch2","219"),
        ("tantan_punch3","21a"),
        ("tantan_ring","21b"),
        ("tantan_spiralleft","216"),
        ("tantan_spiralleftloupe","22d"),
        ("tantan_spiralright","217"),
        ("tantan_spiralrightloupe","22e"),
        ("tantan_spiralsimple","22b"),
        ("term","267"),
        ("toonlink_boomerang","c1"),
        ("toonlink_bow","c2"),
        ("toonlink_bowarrow","c3"),
        ("toonlink_fairy","c6"),
        ("toonlink_hookshot","c4"),
        ("toonlink_hookshot_hand","c5"),
        ("toonlink_pig","c8"),
        ("toonlink_takt","c7"),
        ("wario_garlic","8b"),
        ("wario_wariobike","8a"),
        ("wario_warioman","8c"),
        ("wiifit_balanceboard","ee"),
        ("wiifit_hulahoop","ec"),
        ("wiifit_silhouette","f1"),
        ("wiifit_silhouettel","f2"),
        ("wiifit_sunbullet","ed"),
        ("wiifit_towel","f0"),
        ("wiifit_wiibo","ef"),
        ("wolf_blaster","c9"),
        ("wolf_blaster_bullet","ca"),
        ("wolf_illusion","cb"),
        ("wolf_reticle","cd"),
        ("wolf_wolfen","cc"),
        ("yoshi_star","38"),
        ("yoshi_tamago","39"),
        ("yoshi_yoshibg01","3b"),
        ("yoshi_yoshimob","3a"),
        ("younglink_boomerang","1a6"),
        ("younglink_bow","1a7"),
        ("younglink_bowarrow","1a8"),
        ("younglink_hookshot","1a9"),
        ("younglink_hookshot_hand","1aa"),
        ("younglink_milk","1ac"),
        ("younglink_navy","1ab"),
        ("zelda_dein","6f"),
        ("zelda_dein_s","70"),
        ("zelda_phantom","71"),
        ("zelda_triforce","72")
    ]);
    let lowercased=target_kind.to_lowercase();
    if let Some(hex) = kind_map.get(lowercased.as_str()){
        let int = i64::from_str_radix(hex, 16);
        return (int.unwrap()*-1) as i32;
    }
    return -999;
}

lazy_static! {
    static ref HOOK_ARTICLES: RwLock<bool> = RwLock::new(false);
    static ref HOOK_PARAMS: RwLock<bool> = RwLock::new(false);
}

pub fn can_Hook_Articles() -> bool {
    return *HOOK_ARTICLES.read();
}
pub fn can_Hook_Params() -> bool {
    return *HOOK_PARAMS.read();
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
            else {
            }
            let subparam_str = subparam_string.as_str();

            let index = (hash_str_to_u64(param.param.as_str()),
            hash_str_to_u64(subparam_str)
            );

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
    return find_folders();
}