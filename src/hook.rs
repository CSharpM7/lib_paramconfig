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
pub static mut FLOAT_OFFSET: usize = 0x4dedc0;

pub static FLOAT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x19, 0x40, 0xf9,
];

//Related to Param Edits
fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

#[skyline::hook(offset=INT_OFFSET)]
pub unsafe fn get_param_int_hook(module: u64, param_type: u64, param_hash: u64) -> i32 {
    let mut boma = *((module as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor;
    let boma_reference = &mut *boma;
    let id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let mut slot = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);

    let mut fighter_kind = utility::get_kind(boma_reference);
    if utility::get_category(boma_reference) == *BATTLE_OBJECT_CATEGORY_WEAPON {
        let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        if sv_battle_object::is_active(owner_id) {
            fighter_kind = sv_battle_object::kind(owner_id);
            slot = WorkModule::get_int(sv_battle_object::module_accessor(owner_id), *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        }
        else {
            fighter_kind = -1;
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
        let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        if sv_battle_object::is_active(owner_id) {
            fighter_kind = sv_battle_object::kind(owner_id);
            slot = WorkModule::get_int(sv_battle_object::module_accessor(owner_id), *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        }
        else {
            fighter_kind = -1;
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

pub fn install() {
	unsafe {
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, FLOAT_SEARCH_CODE) {
            FLOAT_OFFSET = offset;
        }
    }
    skyline::install_hooks!(
        get_param_int_hook,
        get_param_float_hook,
    );
	skyline::install_hook!(offset_dump);
}