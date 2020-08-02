use smash::lib::{self, lua_const::*};
use smash::app::{self, lua_bind::*};

pub fn get_category(boma: &mut app::BattleObjectModuleAccessor) -> i32{
    return (boma.info >> 28) as u8 as i32;
}
