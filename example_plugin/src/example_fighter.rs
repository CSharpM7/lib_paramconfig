use smash::{
    app::{self, lua_bind::*, *},
    hash40,
    lib::{lua_const::*, L2CAgent, L2CValue},
    lua2cpp::*,
    phx::*,
};
use std::sync::RwLock;
use lazy_static::lazy_static;

const DEFAULT_SLOTS: [usize;3] = [0,1,2];
lazy_static! {
    pub static ref MOD_SLOTS: RwLock<Vec<usize>> = RwLock::new({
        let mut default = Vec::with_capacity(256);
        for c in DEFAULT_SLOTS {
            default.push(c);
        }
        
        default
    });
}

pub static mut CLONED_ARTICLE_KIND: i32 = 0; 

pub unsafe fn install_parameters() {
    CLONED_ARTICLE_KIND = *WEAPON_KIND_MARIO_FIREBALL; //This should be the weapon kind of whatever you are cloning. NOT the article generate ID.
    let CUSTOM_FIGHTER_KIND: i32 = *smash::lib::lua_const::FIGHTER_KIND_MARIOD;

    let slot = (*MOD_SLOTS.read().unwrap()).to_vec();
    let mut slots: Vec<i32> = Vec::with_capacity(slot.len());
    for s in slot {
        slots.push(s as i32);
    }

    //Hey, did you read the README? Make sure you aren't use `float` for fighter_param attributes like jump height, run speed, etc
    let mut param_ints: Vec<(u64,u64,i32)> = Vec::new();
    let mut param_floats: Vec<(u64,u64,f32)> = Vec::new(); 
    let mut param_attributes: Vec<(u64,u64,f32)> = Vec::new();

    param_config::disable_kirby_copy(CUSTOM_FIGHTER_KIND, slots.clone());
    param_config::disable_villager_pocket(CUSTOM_FIGHTER_KIND, slots.clone(), CLONED_ARTICLE_KIND);
    param_config::disable_villager_pocket(CUSTOM_FIGHTER_KIND, slots.clone(), CLONED_ARTICLE_KIND);

    //If using cargo skyline listen, you'll see these parameters get applied to your game during the initial boot
    param_ints.push((hash40("jump_count_max"),0 as u64, 4));
    param_attributes.push((hash40("dash_speed"),0 as u64, 2.0));
    param_attributes.push((hash40("jump_initial_y"),0 as u64, 2.0));
    param_attributes.push((hash40("jump_y"),0 as u64, 2.0));
    param_attributes.push((hash40("mini_jump_y"),0 as u64, 2.0));

    for p in param_ints {
        param_config::update_int_2(CUSTOM_FIGHTER_KIND, slots.clone(), p);
    }
    for p in param_floats {
        param_config::update_float_2(CUSTOM_FIGHTER_KIND, slots.clone(), p);
    }
    for p in param_attributes {
        param_config::update_attribute_mul_2(CUSTOM_FIGHTER_KIND, slots.clone(), p);
    }
    println!("[smashline_example_mod]: Installed Params");
}

pub fn install() {
    unsafe {
        install_parameters();
    }
}