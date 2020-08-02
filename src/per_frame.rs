use smash::lib::{lua_const::*, L2CAgent};
use smash::app::{self, lua_bind::*};
use smash::lua2cpp::L2CFighterCommon;
use crate::utils::*;

pub fn install() {
    acmd::add_custom_hooks!(sys_line_system_control_fighter_hook);
}

static X_WALL: f32 = 120.0;
static Y_CEIL: f32 = 65.0;
static Y_FLOOR: f32 = -30.0; // must make sure Y_FLOOR is less than Y_CEIL to avoid getting stuck above fake ceiling
static Y_SPEED_THRESHOLD: f32 = 4.0;


pub fn sys_line_system_control_fighter_hook(fighter: &mut L2CFighterCommon) {
    unsafe{
        let lua_state = fighter.lua_state_agent;
        let boma = app::sv_system::battle_object_module_accessor(lua_state);
        let l2c_agent = L2CAgent::new(lua_state);

        if get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
            let x_pos = PostureModule::pos_x(boma);
            let y_pos = PostureModule::pos_y(boma);
            reset_walls_ceiling(boma, x_pos, y_pos);
        }
    }
}


unsafe fn reset_walls_ceiling(boma: &mut app::BattleObjectModuleAccessor, x_pos: f32, y_pos: f32) {
    let relative_pos = GroundModule::get_center_pos(boma);
    if relative_pos.abs() >= X_WALL {
        let reset_pos_x = smash::phx::Vector2f {x: x_pos*-1.0, y: Y_CEIL};
        PostureModule::set_pos_2d(boma, &reset_pos_x);
    }
    if y_pos >= Y_CEIL {
        KineticModule::clear_speed_all(boma);
    }
    if y_pos <= Y_FLOOR && KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN).abs() >= Y_SPEED_THRESHOLD {
        let reset_pos_y = smash::phx::Vector2f {x: x_pos, y: -y_pos};
        PostureModule::set_pos_2d(boma, &reset_pos_y);
    }
}