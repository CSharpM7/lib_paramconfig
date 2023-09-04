 use {
    smash::{
        hash40,
        app::{lua_bind::*, *},
        lib::lua_const::*
    }
};
use super::data::*;
use skyline::hooks::{
    getRegionAddress, 
    Region, 
    InlineCtx
};

//Related to Param Edits
#[skyline::hook(offset=0x3f0028, inline)]
pub unsafe fn offset_dump(ctx: &InlineCtx) {
	let text = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
	println!("Function Offset: {:#X}", ctx.registers[8].x.as_ref() - text);
}

static INT_OFFSET: usize = 0x4e5380; // 12.0.0
static FLOAT_OFFSET: usize = 0x4e53C0; // 12.0.0

#[skyline::hook(offset=INT_OFFSET)]
pub unsafe fn get_param_int_hook(module: u64, param_type: u64, param_hash: u64) -> i32 {
    let mut boma = *((module as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor;
    let boma_reference = &mut *boma;
    let id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let mut slot = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);

    let mut fighter_kind = utility::get_kind(boma_reference);
    if utility::get_category(boma_reference) == *BATTLE_OBJECT_CATEGORY_WEAPON {
        fighter_kind *= -1;
        let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        if sv_battle_object::is_active(owner_id) {
            slot = WorkModule::get_int(sv_battle_object::module_accessor(owner_id), *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        }
    }

    if FighterParamModule::has_kind(fighter_kind)
    {
        if let Some(new_param) = FighterParamModule::get_int_param(fighter_kind, slot,param_type, param_hash){
            return new_param;
        }
    }

    original!()(module, param_type, param_hash)
}


#[skyline::hook(offset=FLOAT_OFFSET)]
pub unsafe fn get_param_float_hook(module: u64, param_type: u64, param_hash: u64) -> f32 {
    let mut boma = *((module as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor;
    let boma_reference = &mut *boma;
    let id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let mut slot = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);

    let mut fighter_kind = utility::get_kind(boma_reference);
    if utility::get_category(boma_reference) == *BATTLE_OBJECT_CATEGORY_WEAPON {
        fighter_kind *= -1;
        let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        if sv_battle_object::is_active(owner_id) {
            slot = WorkModule::get_int(sv_battle_object::module_accessor(owner_id), *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        }
    }

    if FighterParamModule::has_kind(fighter_kind)
    {
        if let Some(new_param) = FighterParamModule::get_float_param(fighter_kind, slot,param_type, param_hash){
            return new_param;
        }
    }
    original!()(module, param_type, param_hash)

}


#[skyline::hook(offset = 0x3a6650)]
unsafe fn get_article_use_type_mask(weapon_kind: i32, entry_id: i32) -> u8 {
    if FighterParamModule::has_kind(weapon_kind) {
        if let Some(new_type) = FighterParamModule::get_article_use_type(weapon_kind){
            return new_type as u8;
        }
    }
    call_original!(weapon_kind, entry_id)
}

pub fn install() {
    skyline::install_hooks!(
        get_param_int_hook,
        get_param_float_hook,
    );
    if super::data::can_Hook_Articles() {
        println!("[libparam_config::main] Article use type hooked");
        skyline::install_hooks!(
            get_article_use_type_mask
        ); 
    }
}