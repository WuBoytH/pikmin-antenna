use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::Vector3f,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    smashline::*
};

#[repr(C)]
struct TroopManager {
  _x0: u64,
  max_pikmin_count: usize, // always 3
  current_pikmin_count: usize,
  pikmin_objects: *mut *mut BattleObject,
  pikmin: [*mut BattleObject; 3],
  // remainder that we don't care about
  // big shoutouts to Blujay for this one
}

#[fighter_frame_callback]
fn pikmin_frame_callback(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = fighter.global_table[0x2].get_i32();
        if fighter_kind == *FIGHTER_KIND_PIKMIN {
            if fighter.global_table[0x9].get_i32() == *FIGHTER_STATUS_KIND_ENTRY
            || sv_information::is_ready_go() {
                let troops = WorkModule::get_int64(fighter.module_accessor, 0x100000C0);
                let troopmanager = troops as *const TroopManager;
                let count = (*troopmanager).current_pikmin_count;
                let pikmin;
                let pikmin_id;
                if count > 0 {
                    pikmin = (*troopmanager).pikmin[0];
                    pikmin_id = (*pikmin).battle_object_id;
                }
                else {
                    pikmin = std::ptr::null_mut();
                    pikmin_id = *BATTLE_OBJECT_ID_INVALID as u32;
                }
                let color;
                if pikmin_id != *BATTLE_OBJECT_ID_INVALID as u32
                && sv_battle_object::is_active(pikmin_id) {
                    let variation = WorkModule::get_int((*pikmin).module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
                    let color_vec = match variation {
                        0 => Vector3f{x: 1.0, y: 0.2, z: 0.2},
                        1 => Vector3f{x: 1.0, y: 1.0, z: 0.2},
                        2 => Vector3f{x: 0.2, y: 0.2, z: 1.0},
                        3 => Vector3f{x: 0.8, y: 0.8, z: 0.8},
                        _ => Vector3f{x: 0.4, y: 0.2, z: 0.8}
                    };
                    color = color_vec;
                }
                else {
                    color = Vector3f{x: 0.2, y: 0.2, z: 0.4};
                }
                let antenna_eff = WorkModule::get_int(fighter.module_accessor, 0x100000C4) as u32;
                EffectModule::set_rgb(fighter.module_accessor, antenna_eff, color.x, color.y, color.z);
            }
        }
    }
}

pub fn install() {
    install_agent_frame_callbacks!(
        pikmin_frame_callback
    );
}