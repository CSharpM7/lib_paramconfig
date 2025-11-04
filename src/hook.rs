use {
    smash::{
        hash40,
        app::{lua_bind::*, *},
        lib::lua_const::*
    }
};
use super::*;
use skyline::hooks::{
    getRegionAddress, 
    Region, 
    InlineCtx
};


const RETURN_0: u64 = 0;
const RETURN_1: u64 = 1;
const RETURN_ORIGINAL: u64 = 2;

static INT_OFFSET: usize = 0x4e53a0; // 13.0.3
static FLOAT_OFFSET: usize = 0x4e53e0; // 13.0.3
static ARTICLE_OFFSET: usize = 0x3a6670; // 13.0.3
static COPY_RESET_OFFSET: usize = 0xb96770;

const KIRBY_VTABLE_PER_FIGHTER_FRAME_OFFSET: usize = 0xb97b30;
const KIRBY_VTABLE_ON_SEARCH: usize = 0xb9d8b0+0x20; 

const VILLAGER_VTABLE_ON_SEARCH: usize = 0xdbcf60+0x20; //isabelle is the same

const ROSETTA_VTABLE_ON_SEARCH: usize = 0x10aa080+0x20;

#[skyline::hook(offset=INT_OFFSET)]
pub unsafe fn get_param_int_hook(module: u64, param_type: u64, param_hash: u64) -> i32 {
    let original_value = original!()(module, param_type, param_hash);

    let mut module_accessor = *((module as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor;
    let module_accessor_reference = &mut *module_accessor;
    let id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let mut slot = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);

    let mut fighter_kind = utility::get_kind(module_accessor_reference);
    if utility::get_category(module_accessor_reference) == *BATTLE_OBJECT_CATEGORY_WEAPON {
        fighter_kind *= -1;
        let owner_id = WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        if sv_battle_object::is_active(owner_id) {
            slot = WorkModule::get_int(sv_battle_object::module_accessor(owner_id), *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        }
    }

    if FighterParamModule::has_kind(fighter_kind)
    {
        if let Some(mult) = FighterParamModule::get_int_param_mul(fighter_kind, slot,param_type, param_hash){
            let temp = (original_value as f32) * mult;
            return temp as i32;
        }
        else if let Some(new_param) = FighterParamModule::get_int_param(fighter_kind, slot,param_type, param_hash){
            return new_param;
        }
    }

    return original_value;
}


#[skyline::hook(offset=FLOAT_OFFSET)]
pub unsafe fn get_param_float_hook(module: u64, param_type: u64, param_hash: u64) -> f32 {
    let original_value = original!()(module, param_type, param_hash);

    let mut module_accessor = *((module as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor;
    let module_accessor_reference = &mut *module_accessor;
    let id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let mut slot = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);

    let mut fighter_kind = utility::get_kind(module_accessor_reference);
    if utility::get_category(module_accessor_reference) == *BATTLE_OBJECT_CATEGORY_WEAPON {
        fighter_kind *= -1;
        let owner_id = WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        if sv_battle_object::is_active(owner_id) {
            slot = WorkModule::get_int(sv_battle_object::module_accessor(owner_id), *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        }
    }

    if FighterParamModule::has_kind(fighter_kind)
    {
        if let Some(mult) = FighterParamModule::get_attribute_mul(fighter_kind, slot,param_type, param_hash) {
            return original_value*mult;
        }
        else if let Some(new_param) = FighterParamModule::get_float_param(fighter_kind, slot,param_type, param_hash) {
            return new_param;
        }
    }

    return original_value;
}


#[skyline::hook(offset = ARTICLE_OFFSET)]
unsafe fn get_article_use_type_mask(weapon_kind: i32, entry_id: i32) -> u8 {
    if FighterParamModule::has_kind(-weapon_kind) {
        if let Some(new_type) = FighterParamModule::get_article_use_type(-weapon_kind){
            return new_type as u8;
        }
    }
    call_original!(weapon_kind, entry_id)
}

#[skyline::from_offset(COPY_RESET_OFFSET)]
fn copy_ability_reset(fighter: *mut Fighter, some_miifighter_bool: bool);

unsafe fn kirby_cant_copy(fighter: &mut Fighter) {
    let module_accessor = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(module_accessor);

    if status_kind != *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_DRINK {
        return;
    }
    if WorkModule::is_flag(module_accessor, *FIGHTER_KIRBY_STATUS_SPECIAL_N_FLAG_DRINK_WEAPON) {
        return;
    }
    if WorkModule::is_flag(module_accessor, *FIGHTER_KIRBY_STATUS_SPECIAL_N_FLAG_SPIT_END) 
    && WorkModule::get_int(module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) != *FIGHTER_KIND_KIRBY {
        let opponent_id = WorkModule::get_int64(module_accessor, *FIGHTER_KIRBY_STATUS_SPECIAL_N_WORK_INT_TARGET_TASK) as u32;
        if opponent_id != *BATTLE_OBJECT_ID_INVALID as u32 && sv_battle_object::is_active(opponent_id) {
            let opp = sv_battle_object::module_accessor(opponent_id);
            let opp_slot = WorkModule::get_int(opp, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
            let opp_kind = utility::get_kind(&mut *opp);

            if FighterParamModule::has_kind(opp_kind) {
                if !FighterParamModule::can_kirby_copy(opp_kind,opp_slot) {
                    copy_ability_reset(fighter, false);
                    let star_id = WorkModule::get_int(module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WN_STAR_TASK_ID) as u32;
                    if sv_battle_object::is_active(star_id) {
                        let star = sv_battle_object::module_accessor(star_id);
                        WorkModule::set_int(star, *FIGHTER_KIND_KIRBY, *WEAPON_KIRBY_STARMISSILE_STATUS_WORK_INT_KIND);
                    }
                }
            }
        }
    }
}

#[skyline::hook(offset = KIRBY_VTABLE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn kirby_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    if *super::IS_HOOKED_KIRBY.read() { 
        kirby_cant_copy(fighter);
    }
    original!()(vtable, fighter)
}

fn get_weapon_common_from_accessor<'a>(boma: &'a mut BattleObjectModuleAccessor) -> &'a mut smash::common::root::lua2cpp::L2CWeaponCommon {
    unsafe {
        let lua_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x190 / 8);
        std::mem::transmute(*((lua_module + 0x1D8) as *mut *mut smash::common::root::lua2cpp::L2CWeaponCommon))
    }
}

#[skyline::hook(offset = KIRBY_VTABLE_ON_SEARCH)]
unsafe extern "C" fn kirby_search(_vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let module_accessor = fighter.battle_object.module_accessor;
    let kind = fighter.battle_object.kind;
    if kind == *FIGHTER_KIND_KIRBY as u32 {
        let status_kind = StatusModule::status_kind(module_accessor);
        if [*FIGHTER_KIRBY_STATUS_KIND_MURABITO_SPECIAL_N_SEARCH,*FIGHTER_KIRBY_STATUS_KIND_SHIZUE_SPECIAL_N_SEARCH].contains(&status_kind) {
            let custom_result = villager_pocket_search(fighter, log,true);
            if custom_result != RETURN_ORIGINAL {return custom_result};
        }
        else if status_kind == *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_LOOP {
            //let custom_result = kirby_inhale_search(fighter, log);
            //if custom_result != RETURN_ORIGINAL {return custom_result};
        }
    }
    original!()(_vtable, fighter, log)
}
/*
unsafe fn check_inhale_target(fighter: &mut Fighter, object_id: u32) -> i32 {
    let object_boma = sv_battle_object::module_accessor(object_id);
    let object_cat = utility::get_category(&mut *object_boma);
    let object_kind = utility::get_kind(&mut *object_boma);
    if object_cat == *BATTLE_OBJECT_CATEGORY_WEAPON {
        let owner_id = WorkModule::get_int(object_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        let owner_boma = sv_battle_object::module_accessor(owner_id);
        let owner_kind = utility::get_kind(&mut *owner_boma);
        let owner_slot = WorkModule::get_int(owner_boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);

        //if owner_kind is found in table
        if FighterParamModule::has_kind(owner_kind) {
            if let Some(new_behavior) = FighterParamModule::get_kirby_inhale_behavior(owner_kind,owner_slot,object_kind) {
                return new_behavior;
            }
        }
    }
    return super::POCKET_BEHAVIOR_ORIGINAL;
}

unsafe fn kirby_inhale_search(fighter: &mut Fighter, log: u64, is_kirby: bool) -> u64 {
    use wubor_utils::app::*;

    let module_accessor = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(module_accessor);
    if (!is_kirby && status_kind == FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH) ||
    is_kirby
    {
        let collision_log = *(log as *const u64).add(0x10 / 0x8);
        let collision_log = collision_log as *mut CollisionLogScuffed;
        let collider_id = (*collision_log).opponent_object_id;
        if collider_id == *BATTLE_OBJECT_ID_INVALID as u32 {return RETURN_ORIGINAL;}

        let pocket_behavior = check_pocket_target(fighter,collider_id);
        if pocket_behavior == super::POCKET_BEHAVIOR_ORIGINAL {return RETURN_ORIGINAL;}
        if pocket_behavior == super::POCKET_BEHAVIOR_IGNORE {return RETURN_0;}

        //Destroy weapon//
        let weapon_boma = sv_battle_object::module_accessor(collider_id);
        let pos = *PostureModule::pos(weapon_boma);
        EffectModule::req(
            weapon_boma,
            Hash40::new("sys_erace_smoke"),
            &Vector3f{x:pos.x,y:pos.y+2.0,z:pos.z},
            &Vector3f{x:0.0,y:0.0,z:0.0},
            0.625,
            0,
            -1,
            false,
            0
        );
        
        use smash_script::*;
        let weapon = get_weapon_common_from_accessor(&mut *weapon_boma);
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        
        if pocket_behavior == super::POCKET_BEHAVIOR_IGNORE {
            return RETURN_0;
        }
        
        if pocket_behavior == super::POCKET_BEHAVIOR_MISFIRE {
            let mut next_status = *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_FAILURE;
            if is_kirby {
                if status_kind == *FIGHTER_KIRBY_STATUS_KIND_MURABITO_SPECIAL_N_SEARCH {
                    next_status = *FIGHTER_KIRBY_STATUS_KIND_MURABITO_SPECIAL_N_FAILURE;
                }
                else if status_kind == *FIGHTER_KIRBY_STATUS_KIND_SHIZUE_SPECIAL_N_SEARCH {
                    next_status = *FIGHTER_KIRBY_STATUS_KIND_SHIZUE_SPECIAL_N_FAILURE;
                }
            }
            StatusModule::change_status_request_from_script(module_accessor, next_status, false);
            return RETURN_0;
        }
    }
    return RETURN_ORIGINAL;
}
 */

#[skyline::hook(offset = VILLAGER_VTABLE_ON_SEARCH)]
pub unsafe extern "C" fn villager_search(_vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let module_accessor = fighter.battle_object.module_accessor;
    let kind = fighter.battle_object.kind;
    if kind == *FIGHTER_KIND_MURABITO as u32 
    || kind == *FIGHTER_KIND_SHIZUE as u32 {
        let custom_result = villager_pocket_search(fighter, log,false);
        if custom_result != RETURN_ORIGINAL {return custom_result};
    }
    original!()(_vtable, fighter, log)
}
unsafe fn check_pocket_target(fighter: &mut Fighter, object_id: u32) -> i32 {
    let object_boma = sv_battle_object::module_accessor(object_id);
    let object_cat = utility::get_category(&mut *object_boma);
    let object_kind = utility::get_kind(&mut *object_boma);
    if object_cat == *BATTLE_OBJECT_CATEGORY_WEAPON {
        let owner_id = WorkModule::get_int(object_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        let owner_boma = sv_battle_object::module_accessor(owner_id);
        let owner_kind = utility::get_kind(&mut *owner_boma);
        let owner_slot = WorkModule::get_int(owner_boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);

        //if owner_kind is found in table
        if FighterParamModule::has_kind(owner_kind) {
            if let Some(new_behavior) = FighterParamModule::get_villager_pocket_behavior(owner_kind,owner_slot,object_kind) {
                return new_behavior;
            }
        }
    }
    return super::POCKET_BEHAVIOR_ORIGINAL;
}

unsafe fn villager_pocket_search(fighter: &mut Fighter, log: u64, is_kirby: bool) -> u64 {
    use wubor_utils::app::*;

    let module_accessor = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(module_accessor);
    if (!is_kirby && status_kind == FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH) ||
    is_kirby
    {
        let collision_log = *(log as *const u64).add(0x10 / 0x8);
        let collision_log = collision_log as *mut CollisionLogScuffed;
        let collider_id = (*collision_log).opponent_object_id;
        if collider_id == *BATTLE_OBJECT_ID_INVALID as u32 {return RETURN_ORIGINAL;}

        let pocket_behavior = check_pocket_target(fighter,collider_id);
        if pocket_behavior == super::POCKET_BEHAVIOR_ORIGINAL {return RETURN_ORIGINAL;}
        if pocket_behavior == super::POCKET_BEHAVIOR_IGNORE {return RETURN_0;}

        //Destroy weapon//
        let weapon_boma = sv_battle_object::module_accessor(collider_id);
        let pos = *PostureModule::pos(weapon_boma);
        EffectModule::req(
            weapon_boma,
            Hash40::new("sys_erace_smoke"),
            &Vector3f{x:pos.x,y:pos.y+2.0,z:pos.z},
            &Vector3f{x:0.0,y:0.0,z:0.0},
            0.625,
            0,
            -1,
            false,
            0
        );
        
        use smash_script::*;
        let weapon = get_weapon_common_from_accessor(&mut *weapon_boma);
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        
        if pocket_behavior == super::POCKET_BEHAVIOR_IGNORE {
            return RETURN_0;
        }
        
        if pocket_behavior == super::POCKET_BEHAVIOR_MISFIRE {
            let mut next_status = *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_FAILURE;
            if is_kirby {
                if status_kind == *FIGHTER_KIRBY_STATUS_KIND_MURABITO_SPECIAL_N_SEARCH {
                    next_status = *FIGHTER_KIRBY_STATUS_KIND_MURABITO_SPECIAL_N_FAILURE;
                }
                else if status_kind == *FIGHTER_KIRBY_STATUS_KIND_SHIZUE_SPECIAL_N_SEARCH {
                    next_status = *FIGHTER_KIRBY_STATUS_KIND_SHIZUE_SPECIAL_N_FAILURE;
                }
            }
            StatusModule::change_status_request_from_script(module_accessor, next_status, false);
            return RETURN_0;
        }
    }
    return RETURN_ORIGINAL;
}

#[skyline::hook(offset = ROSETTA_VTABLE_ON_SEARCH)]
pub unsafe extern "C" fn rosetta_search(_vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let module_accessor = fighter.battle_object.module_accessor;
    let kind = fighter.battle_object.kind;
    if kind == *FIGHTER_KIND_ROSETTA as u32 {
        let custom_result = rosetta_cant_pull(fighter, log);
        if custom_result != RETURN_ORIGINAL {return custom_result};
    }
    original!()(_vtable, fighter, log)
}
unsafe fn check_pull_target(fighter: &mut Fighter, object_id: u32) -> bool {
    let object_boma = sv_battle_object::module_accessor(object_id);
    let object_cat = utility::get_category(&mut *object_boma);
    let object_kind = utility::get_kind(&mut *object_boma);
    if object_cat == *BATTLE_OBJECT_CATEGORY_WEAPON {
        let owner_id = WorkModule::get_int(object_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        let owner_boma = sv_battle_object::module_accessor(owner_id);
        let owner_kind = utility::get_kind(&mut *owner_boma);
        let owner_slot = WorkModule::get_int(owner_boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);

        //if owner_kind is found in table
        if FighterParamModule::has_kind(owner_kind) {
            if !FighterParamModule::can_rosetta_pull(owner_kind,owner_slot,object_kind) {
                return true;
            }
        }
    }
    return false;
}
unsafe fn rosetta_cant_pull(fighter: &mut Fighter, log: u64) -> u64 {
    use wubor_utils::app::*;

    let module_accessor = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(module_accessor);
    if status_kind == FIGHTER_STATUS_KIND_SPECIAL_LW
    {
        let collision_log = *(log as *const u64).add(0x10 / 0x8);
        let collision_log = collision_log as *mut CollisionLogScuffed;
        let collider_id = (*collision_log).opponent_object_id;
        if collider_id == *BATTLE_OBJECT_ID_INVALID as u32 {return RETURN_ORIGINAL;}

        if check_pull_target(fighter,collider_id) {
            let object_boma = sv_battle_object::module_accessor(collider_id);
            WorkModule::on_flag(object_boma, super::WEAPON_INSTANCE_WORK_ID_FLAG_ROSETTA_PULLED);
            return RETURN_0;
        }
    }
    return RETURN_ORIGINAL;
}

pub fn install_params() {
    super::set_hash_any();
    if super::can_hook_params() {
        println!("[libparam_config] Hooking GetParam functions");
        skyline::install_hooks!(
            get_param_int_hook,
            get_param_float_hook,
            //read_melee_mode
        );
        *super::IS_HOOKED_PARAMS.write() = true;
    }
}
pub fn install_articles() {
    if super::can_hook_articles() {
        println!("[libparam_config] Hooking Article Use Type function");
        skyline::install_hooks!(
            get_article_use_type_mask
        ); 
        *super::IS_HOOKED_ARTICLES.write() = true;
    }
}
pub fn install_kirby() {
    if super::can_hook_kirby() || super::can_hook_villager() {
        println!("[libparam_config] Hooking Kirby Frame Vtable");
        skyline::install_hooks!(
            kirby_opff,
            kirby_search,
        ); 
        *super::IS_HOOKED_KIRBY.write() = true;
    }
}
pub fn install_villager() {
    if super::can_hook_villager() {
        println!("[libparam_config] Hooking Villager and Kirby On Search Vtables");
        skyline::install_hooks!(
            villager_search,
        ); 
        *super::IS_HOOKED_VILLAGER.write() = true;

        install_kirby();
    }
}
pub fn install_rosetta() {
    if super::can_hook_rosetta() {
        println!("[libparam_config] Hooking Rosetta On Search Vtable");
        skyline::install_hooks!(
            rosetta_search
        ); 
        *super::IS_HOOKED_ROSETTA.write() = true;
    }
}