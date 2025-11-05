use smash_rs::*;

#[repr(C)]
pub struct CollisionLogScuffed {
    pub x00: *const u64,
    pub x08: *const u64,
    pub location: smash_rs::cpp::simd::Vector3,
    pub x20: u8,
    pub x21: u8,
    pub x22: u8,
    pub x23: u8,
    pub opponent_object_id: u32,
    pub opponent_object_category: u8,
    pub x29: u8,
    pub x2A: u8,
    pub x2B: u8,
    pub x2C: u8,
    pub x2D: u8,
    pub x2E: u8,
    pub collision_kind: u8,
    pub receiver_part_id: u8,
    pub collider_part_id: u8,
    pub receiver_id: u8,
    pub collider_id: u8,
}