use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CAgent, L2CValue},
        hash40
    },
    smash_script::*,
    smashline::{*, Priority::*}
};
use smash::app::sv_animcmd::IS_RANDOM;

static mut alStack80: bool = false;

pub const FIGHTER_MARIO_STATUS_KIND_SHOULDER_END: i32 = 0x1E3;
pub const FIGHTER_MARIO_STATUS_KIND_SHOULDER_FALL: i32 = 0x1E4;
pub const FIGHTER_MARIO_STATUS_KIND_SHOULDER_PASS: i32 = 0x1E5;
pub const FIGHTER_MARIO_STATUS_KIND_SHOULDER_START: i32 = 0x1E6;
pub const FIGHTER_MARIO_STATUS_KIND_SHOULDER_TURN: i32 = 0x1E7;
pub const FIGHTER_MARIO_STATUS_KIND_SHOULDER_WAIT: i32 = 0x1E8;
pub const FIGHTER_MARIO_STATUS_KIND_SHOULDER_WALK: i32 = 0x1E9;
pub const FIGHTER_MARIO_STATUS_KIND_THROW_F_B: i32 = 0x1EA;
pub const FIGHTER_MARIO_STATUS_KIND_THROW_F_F: i32 = 0x1EB;
pub const FIGHTER_MARIO_STATUS_KIND_THROW_F_LW: i32 = 0x1EC;
pub const FIGHTER_MARIO_STATUS_KIND_THROW_F_HI: i32 = 0x1ED;
pub const FIGHTER_MARIO_STATUS_KIND_SHOULDER_LANDING: i32 = 0x1EE;

pub const MARIO_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x68d630;

pub const FIGHTER_MARIO_STATUS_SHOULDER_START_FLAG_SHOULDER_WAIT: i32 = 0x2100000C;

unsafe extern "C" fn fighter_on(fighter: &mut L2CFighterCommon) {
    fighter.global_table[0x45].assign(&L2CValue::I32(FIGHTER_MARIO_STATUS_KIND_SHOULDER_START));
}


#[skyline::hook(offset = MARIO_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn mario_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;

    if MotionModule::motion_kind(boma) == hash40("catch_cut"){
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
    } 
    original!()(vtable, fighter)
}


pub unsafe extern "C" fn FUN_7100021af0(result: bool,fighter: &mut L2CFighterCommon) {
    //let temp;
    if CancelModule::is_enable_cancel(fighter.module_accessor) == false {
        //top
        if MotionModule::is_end(fighter.module_accessor) == false || fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
            if MotionModule::is_end(fighter.module_accessor) == false {
            } else {
                if fighter.global_table[0x16] == *SITUATION_KIND_AIR {
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                    alStack80 = true;
                    return();
                }
            }
            if StatusModule::is_changing(fighter.module_accessor) == false{
                if fighter.global_table[0x17] == *SITUATION_KIND_GROUND {
                    if fighter.global_table[0x16] == *SITUATION_KIND_AIR {
                        if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
                            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                        } else {
                            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);		
                            
                        }
                    }
                }
                if fighter.global_table[0x17] != *SITUATION_KIND_GROUND {
                    if fighter.global_table[0x16] == *SITUATION_KIND_GROUND {
                        if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
                            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                        } else {
                            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);		
                        
                        }
                    }
                }
            } else {
                if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
                    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                } else {
                    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);		
                    
                }
            }
            alStack80 = false;
            return();
        }	
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    } else {
        if fighter.global_table[0x16] != *SITUATION_KIND_GROUND || fighter.sub_wait_ground_check_common(false.into()).get_bool() == false{
            if fighter.global_table[0x16] == *SITUATION_KIND_AIR {
                if fighter.sub_air_check_fall_common().get_bool(){
                    alStack80 = false;
                    return();
                }
            }	
            if MotionModule::is_end(fighter.module_accessor) == false || fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
                if MotionModule::is_end(fighter.module_accessor) == false {
                } else {
                  if fighter.global_table[0x16] == *SITUATION_KIND_AIR {
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                    alStack80 = true;
                    return();
                  }
                }
                if StatusModule::is_changing(fighter.module_accessor) == false{
                    if fighter.global_table[0x17] == *SITUATION_KIND_GROUND {
                        if fighter.global_table[0x16] == *SITUATION_KIND_AIR {
                            if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
                                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                            } else {
                                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);		
                                
                            }
                        }
                    }
                    if fighter.global_table[0x17] != *SITUATION_KIND_GROUND {
                        if fighter.global_table[0x16] == *SITUATION_KIND_GROUND {
                            if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
                                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                            } else {
                                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);		
                                
                            }
                        }
                    }
                  
                } else {
                    if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
                        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                    } else {
                        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);		
                        
                    }
                }
                alStack80 = false;
                return();
            }	
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    alStack80 = true;
    return();
}

pub unsafe extern "C" fn FUN_7100023390(mut param_1: bool, fighter: &mut L2CFighterCommon, param_3: bool) {
    if param_3 == true {
        if WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_TURN_WORK_FLOAT_TURN_FRAME) > 0.0 {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_TURN_FLAG_TURN) == true {
                println!("turning");
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_TURN_FLAG_TURN);
                PostureModule::reverse_lr(fighter.module_accessor);
        	}
        }
    } else {
    	WorkModule::add_float(fighter.module_accessor, -1.0, *FIGHTER_STATUS_TURN_WORK_FLOAT_TURN_FRAME);
    }
    param_1 = false;
}

pub unsafe extern "C" fn FUN_7100023820(param_1: bool, fighter: &mut L2CFighterCommon){
    let mut var2 = false;
    let mut numvar;
    if !ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        if fighter.global_table[0x20].get_i32() == *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON {
            var2 = true;
        } 
        numvar = false;
    } else {
        numvar = fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND;
    }
    if numvar == false{
        if WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_stick_y")) > ControlModule::get_stick_y(fighter.module_accessor){
        } else {
            if fighter.global_table[0x1d].get_i32() < WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("jump_flick_y")){
                numvar = false;
            } else {
                numvar = fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND;
            }
            if numvar{
                /* 
                if ControlModule::is_enable_flick_jump(fighter.module_accessor){
                  fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SHOULDER_JUMP_SQUAT.into(), true.into());
                  alStack80 = true;
                  return();
                }
                */
            }
        }
        alStack80 = false;
        
    } else{
        /*
        fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SHOULDER_JUMP_SQUAT_B.into(), true.into());
        alStack80 = true;
        */
    }
    return();
}

pub unsafe extern "C" fn FUN_7100022520(param_1: bool, fighter: &mut L2CFighterCommon) {
    let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE) as u32;
    let capture_boma = sv_battle_object::module_accessor(capture_id);
    
    println!("step 1");
    if ControlModule::get_clatter_time(capture_boma, 0) <= 0.0 {
      alStack80 = false;
      return();
    }
      
    println!("step 2");
    let alStack128 = (fighter.global_table[0x21].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_F);
    let alStack144 = (fighter.global_table[0x21].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_B);
    let alStack160 = (fighter.global_table[0x21].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_HI);
    let alStack176 = (fighter.global_table[0x21].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_LW);

    println!("{}", fighter.global_table[0x21].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_F);

    let alStack208 = *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S | *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N | *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI | *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW;

    let alStack224 = (fighter.global_table[0x21].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N);

    let xpos = ControlModule::get_stick_x(fighter.module_accessor);
    let ypos = ControlModule::get_stick_y(fighter.module_accessor);

    if (xpos*PostureModule::lr(fighter.module_accessor) > 0.5 && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)) == false{
        if (xpos*PostureModule::lr(fighter.module_accessor) < -0.5 && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)) == false{
            if (ypos > 0.5 && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)) == false{
                if (ypos < -0.5 && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)) == false{
                    if (xpos*PostureModule::lr(fighter.module_accessor) < 0.5 && xpos*PostureModule::lr(fighter.module_accessor) > -0.5 && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)) == false {
                        alStack80 = false;
                    } else {
                        println!("fthrow");
                        fighter.change_status(FIGHTER_MARIO_STATUS_KIND_THROW_F_F.into(), true.into());
                        alStack80 = true;
                    }
                } else {
                    println!("lwthrow");
                    fighter.change_status(FIGHTER_MARIO_STATUS_KIND_THROW_F_LW.into(), true.into());
                    alStack80 = true;
                }
            } else {
                println!("hithrow");
                fighter.change_status(FIGHTER_MARIO_STATUS_KIND_THROW_F_HI.into(), true.into());
                alStack80 = true;
                
        } else {
            println!("bthrow");
            fighter.change_status(FIGHTER_MARIO_STATUS_KIND_THROW_F_B.into(), true.into());
            alStack80 = true;
        }
    } else {
        fighter.change_status(FIGHTER_MARIO_STATUS_KIND_THROW_F_F.into(), true.into());
        alStack80 = true;
    }
    return();
    
}

pub unsafe extern "C" fn FUN_7100021780(fighter: &mut L2CFighterCommon){
    let thingy = (WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND));
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    println!("attack attempt");
    if status_kind == FIGHTER_MARIO_STATUS_KIND_THROW_F_F{
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("throw_f_f"), 0.0, 1.0, false, 0.0, false, false);
    } else if status_kind == FIGHTER_MARIO_STATUS_KIND_THROW_F_B{
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("throw_f_b"), 0.0, 1.0, false, 0.0, false, false);
    } else if status_kind == FIGHTER_MARIO_STATUS_KIND_THROW_F_LW{
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("throw_f_lw"), 0.0, 1.0, false, 0.0, false, false);
    } else if status_kind == FIGHTER_MARIO_STATUS_KIND_THROW_F_HI{
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("throw_f_hi"), 0.0, 1.0, false, 0.0, false, false);
    } else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("throw_f_f"), 0.0, 1.0, false, 0.0, false, false);
    }


    //MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(thingy), 0.0, 1.0, false, 0.0, false, false);
    return();
}

pub unsafe extern "C" fn shoulderend_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

pub unsafe extern "C" fn shoulderend_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor) == false {
        if fighter.global_table[0x16] != *SITUATION_KIND_GROUND{
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    return 0.into();
}

pub unsafe extern "C" fn shoulderend_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

pub unsafe extern "C" fn shoulderfall_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

pub unsafe extern "C" fn shoulderfall_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    FUN_7100022520(false,fighter);
    if alStack80 != true{
        if fighter.global_table[0x16] != *SITUATION_KIND_GROUND{
            if MotionModule::is_end(fighter.module_accessor){
                if MotionModule::motion_kind(fighter.module_accessor) != hash40("shoulder_fall") {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("shoulder_fall"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
            return 0.into();
        }
        fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SHOULDER_LANDING.into(), false.into());
    }
    return 1.into();

}

pub unsafe extern "C" fn shoulderfall_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_AIR), 
        *FIGHTER_KINETIC_TYPE_MOTION_FALL, 
        GROUND_CORRECT_KIND_AIR.into(), 
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_FALL_FLOAT,
        0
    );

    let aLStack200 = *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_TREADED_KIND_NO_REAC | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON;
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor, 
        false, 
        *FIGHTER_TREADED_KIND_NO_REAC, 
        false, 
        false, 
        true, 
        0, 
        0, 
        0, 
        0
    );

    return 0.into();
}

pub unsafe extern "C" fn shoulderpass_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

unsafe fn get_table_value(table: *mut smash2::lib::L2CTable, key: &str) -> smash2::lib::L2CValue {
    let hash = if key.starts_with("0x") {
        smash2::phx::Hash40::from_hex_str(key).unwrap()
    } else {
        smash2::phx::hash40(key)
    };
    (*table).get_map(hash).unwrap().clone()
}

unsafe extern "C" fn check_dmg(fighter: &mut L2CFighterCommon, param2: &L2CValue) -> L2CValue {
    let table = param2.get_table() as *mut smash2::lib::L2CTable;

    let damage = get_table_value(table, "damage_").try_float().unwrap();
    let damage_add = get_table_value(table, "damage_add_").try_float().unwrap();
    let damage_add_reaction = get_table_value(table, "damage_add_reaction_").try_float().unwrap();
    println!(
        "Damage: {}\nDamageAdd: {}\nDamageAddReac: {}",
        damage,
        damage_add,
        damage_add_reaction
    );

    let object_id = get_table_value(table, "0x10d723eebb").try_integer().unwrap() as u32;
    let boma = sv_battle_object::module_accessor(object_id);

    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DEAD, false);

    0.into()
}

pub unsafe extern "C" fn shoulderpass_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    println!("we are exiting");
    if fighter.global_table[0xb].get_i32() != FIGHTER_MARIO_STATUS_KIND_THROW_F_F{
        if fighter.global_table[0xb].get_i32() != FIGHTER_MARIO_STATUS_KIND_THROW_F_B{
            if fighter.global_table[0xb].get_i32() != FIGHTER_MARIO_STATUS_KIND_THROW_F_HI{
                if fighter.global_table[0xb].get_i32() != FIGHTER_MARIO_STATUS_KIND_THROW_F_LW{
                    if fighter.global_table[0xb].get_i32() != FIGHTER_MARIO_STATUS_KIND_SHOULDER_START{
                        return 0.into();
                    }
                    if fighter.global_table[0xb].get_i32() != FIGHTER_MARIO_STATUS_KIND_SHOULDER_WAIT{
                        return 0.into();
                    }
                    if fighter.global_table[0xb].get_i32() != FIGHTER_MARIO_STATUS_KIND_SHOULDER_WALK{
                        return 0.into();
                    }
                    if fighter.global_table[0xb].get_i32() != FIGHTER_MARIO_STATUS_KIND_SHOULDER_START{
                        return 0.into();
                    }
                    if fighter.global_table[0xb].get_i32() != FIGHTER_MARIO_STATUS_KIND_SHOULDER_PASS{
                        return 0.into();
                    }
                    if fighter.global_table[0xb].get_i32() != FIGHTER_MARIO_STATUS_KIND_SHOULDER_FALL{
                        return 0.into();
                    }
                    if fighter.global_table[0xb].get_i32() != FIGHTER_MARIO_STATUS_KIND_SHOULDER_LANDING{
                        return 0.into();
                    }
                } else {
                    WorkModule::set_int64(fighter.module_accessor, hash40("throw_f_lw") as i64,*FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                    fighter.clear_lua_stack();
                    lua_args!(fighter, *MA_MSC_SET_IGNORE_CATCHING, false);
                    smash::app::sv_module_access::capture(fighter.lua_state_agent);
                    //alStack80 = fighter.pop_lua_stack(1);
                    println!("lwthrow");
                }
            } else {
                WorkModule::set_int64(fighter.module_accessor, hash40("throw_f_hi") as i64,*FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                fighter.clear_lua_stack();
                lua_args!(fighter, *MA_MSC_SET_IGNORE_CATCHING, false);
                smash::app::sv_module_access::capture(fighter.lua_state_agent);
                //alStack80 = fighter.pop_lua_stack(1);
                println!("hithrow");
            }
        } else {
            WorkModule::set_int64(fighter.module_accessor, hash40("throw_f_b") as i64,*FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
            fighter.clear_lua_stack();
            lua_args!(fighter, *MA_MSC_SET_IGNORE_CATCHING, false);
            smash::app::sv_module_access::capture(fighter.lua_state_agent);
            //alStack80 = fighter.pop_lua_stack(1);
            println!("bthrow");
        }
    } else {
        WorkModule::set_int64(fighter.module_accessor, hash40("throw_f_f") as i64,*FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
        fighter.clear_lua_stack();
        lua_args!(fighter, *MA_MSC_SET_IGNORE_CATCHING, false);
        smash::app::sv_module_access::capture(fighter.lua_state_agent);
        //alStack80 = fighter.pop_lua_stack(1);
        println!("fthrow");
    }

    return 0.into();
}

pub unsafe extern "C" fn shoulderpass_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_flick_y(fighter.module_accessor);
    GroundModule::pass_floor(fighter.module_accessor);
    fighter.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    fighter.sub_shift_status_main(L2CValue::Ptr(shoulderpass_main_loop as *const () as _))
}

pub unsafe extern "C" fn shoulderpass_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0x10].get_i32() == FIGHTER_MARIO_STATUS_KIND_SHOULDER_START{
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("shoulder_fall"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(shoulderfall_main_loop as *const () as _))
}

pub unsafe extern "C" fn shoulderpass_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_AIR), 
        *FIGHTER_KINETIC_TYPE_MOTION_FALL, 
        GROUND_CORRECT_KIND_KEEP.into(), 
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_FALL_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE
    );

    let aLStack200 = *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_TREADED_KIND_NO_REAC | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON;
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor, 
        false, 
        *FIGHTER_TREADED_KIND_NO_REAC, 
        false, 
        false, 
        true, 
        0, 
        0, 
        0, 
        0
    );
    return 0.into();
}

pub unsafe extern "C" fn shoulderstart_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

pub unsafe extern "C" fn shoulderstart_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    LinkModule::send_event_nodes(fighter.module_accessor, *LINK_NO_CAPTURE, Hash40::new("shoulder"), 0);
    fighter.clear_lua_stack();
    lua_args!(fighter, *MA_MSC_SET_IGNORE_CATCHING, true);
    smash::app::sv_module_access::capture(fighter.lua_state_agent);
    fighter.pop_lua_stack(1);
    HitModule::set_invincible_frame_global(fighter.module_accessor, WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("throw_invincible_frame")) as i32, false, 0);
    return 0.into();
}

pub unsafe extern "C" fn shoulderstart_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("throw_f"), 0.0, 1.0, false, 0.0, false, false);
    fighter.clear_lua_stack();
    lua_args!(fighter, Hash40::new_raw(0x20cbc92683).hash, 1, *FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_01-1);
    smash::app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
    fighter.pop_lua_stack(1);
    println!("we are here");
    fighter.sub_shift_status_main(L2CValue::Ptr(shoulderstart_main_loop as *const () as _))
}

pub unsafe extern "C" fn shoulderstart_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) == false {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARIO_STATUS_SHOULDER_START_FLAG_SHOULDER_WAIT) == false{
            return 0.into();
        }
        if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
            if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_AIR {
                return 0.into();
            }
            fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SHOULDER_FALL.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SHOULDER_WAIT.into(), false.into());
        }
    } else {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() == false {
            if fighter.sub_air_check_fall_common().get_bool() == false{
                if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARIO_STATUS_SHOULDER_START_FLAG_SHOULDER_WAIT) == false{
                    return 0.into();
                }
                if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
                    if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_AIR {
                        return 0.into();
                    }
                    fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SHOULDER_FALL.into(), false.into());
                } else {
                    fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SHOULDER_WAIT.into(), false.into());
                }
                return 1.into();
            }	
        }
        FUN_7100022520(alStack80,fighter);
        if alStack80 != true{
            FUN_7100023820(alStack80,fighter);
            if alStack80{
                return 1.into();
            }
            let mut temp = *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH | *FIGHTER_PAD_CMD_CAT1_FLAG_TURN;
            //this is an intentional error
            if (fighter.global_table[0x20].get_i32() & temp) == 0 || fighter.global_table[0x16] != *SITUATION_KIND_GROUND{
                temp = *FIGHTER_PAD_CMD_CAT1_FLAG_DASH | *FIGHTER_PAD_CMD_CAT1_FLAG_WALK;
                if (fighter.global_table[0x20].get_i32() & temp) == 0 {
                    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARIO_STATUS_SHOULDER_START_FLAG_SHOULDER_WAIT) == false{
                        return 0.into();
                    }
                    if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
                        if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_AIR {
                            return 0.into();
                        }
                        fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SHOULDER_FALL.into(), false.into());
                    } else {
                        fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SHOULDER_WAIT.into(), false.into());
                    }
                    return 1.into();
                }
                if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
                    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARIO_STATUS_SHOULDER_START_FLAG_SHOULDER_WAIT) == false{
                        return 0.into();
                    }
                    if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
                        if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_AIR {
                            return 0.into();
                        }
                        fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SHOULDER_FALL.into(), false.into());
                    } else {
                        fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SHOULDER_WAIT.into(), false.into());
                    }
                    return 1.into();
                }
                fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SHOULDER_WALK.into(), true.into());
            } else {
                fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SHOULDER_TURN.into(), true.into());
            }
        }
        return 1.into();
    }
    return 1.into();
}

pub unsafe extern "C" fn shoulderstart_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_GROUND), 
        *FIGHTER_KINETIC_TYPE_MOTION_LOOP, 
        GROUND_CORRECT_KIND_GROUND_OTTOTTO.into(), 
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_FALL_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE
    );

    let aLStack200 = *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_TREADED_KIND_NO_REAC | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON;
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor, 
        true, 
        *FIGHTER_TREADED_KIND_NO_REAC, 
        false, 
        false, 
        true, 
        0, 
        0, 
        *FIGHTER_STATUS_ATTR_SCALE_KINETIC_ENERGY as u32, 
        0
    );

    return 0.into();
}

pub unsafe extern "C" fn shoulderturn_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

pub unsafe extern "C" fn shoulderturn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut temp = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_frame"));
    WorkModule::set_float(fighter.module_accessor, temp, *FIGHTER_STATUS_TURN_WORK_FLOAT_TURN_FRAME);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_TURN_FLAG_TURN);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("shoulder_turn"), 0.0, 1.0, false, 0.0, false, false);
    println!("trying to turn");
    let mut temp2 = StopModule::is_stop(fighter.module_accessor);
    if !temp2 == true {
        FUN_7100023390(temp2,fighter,true);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(shoulderturn_main_loop as *const () as _))
}

pub unsafe extern "C" fn shoulderturn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    FUN_7100022520(alStack80,fighter);
    if alStack80 == false{
        FUN_7100023820(alStack80,fighter);
        if alStack80 == false {
            if MotionModule::is_end(fighter.module_accessor) == false || fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
                if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_AIR{
                    return 0.into();
                }
                fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SHOULDER_FALL.into(), false.into());
            } else {
              fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SHOULDER_WAIT.into(), false.into());
            }
        }
    }

    return 1.into();
}

pub unsafe extern "C" fn shoulderturn_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_GROUND), 
        *FIGHTER_KINETIC_TYPE_TURN, 
        GROUND_CORRECT_KIND_GROUND.into(), 
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_FALL_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE
    );

    let aLStack200 = *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_TREADED_KIND_NO_REAC | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON;
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor, 
        false, 
        *FIGHTER_TREADED_KIND_NO_REAC, 
        false, 
        false, 
        false, 
        0, 
        0, 
        0, 
        0
    );

    return 0.into();
}

pub unsafe extern "C" fn shoulderwait_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

pub unsafe extern "C" fn shoulderwait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[10] != FIGHTER_MARIO_STATUS_KIND_SHOULDER_START {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("shoulder_wait"), 0.0, 1.0, false, 0.0, false, false);
    }

    fighter.sub_shift_status_main(L2CValue::Ptr(shoulderwait_main_loop as *const () as _))
}

pub unsafe extern "C" fn shoulderwait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut temp;
    println!("we are waiting");
    FUN_7100022520(alStack80,fighter);
    if alStack80 != true{
        FUN_7100023820(alStack80,fighter);
        if alStack80 != true {
            if ControlModule::get_stick_y(fighter.module_accessor) >WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("pass_stick_y")){
                //LAB_71000244a4:
                temp = fighter.global_table[0x1a].get_f32() * PostureModule::lr(fighter.module_accessor) <= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_stick_x"));
                if temp == false {
                    temp = false;
                } else {
                    temp = fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND;
                }
                if temp == false{
                    temp = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("walk_stick_x")) <= fighter.global_table[0x1a].get_f32() * PostureModule::lr(fighter.module_accessor);
                    println!("temp1 {} {} {}", temp, WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("walk_stick_x")), fighter.global_table[0x1a].get_f32() * PostureModule::lr(fighter.module_accessor));
                    if temp == false {
                        temp = false;
                    }else{
                        temp = fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND;
                    }
                    println!("temp2 {}", temp);
                    
                    if temp == false{
                        if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_AIR {
                            if MotionModule::is_end(fighter.module_accessor) == false {
                              
                            } else {
                                if MotionModule::motion_kind(fighter.module_accessor) != hash40("shoulder_wait"){
                                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("shoulder_wait"), 0.0, 1.0, false, 0.0, false, false);
                                }
                            }
                            return 0.into();
                        }
                        fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SHOULDER_FALL.into(), false.into());
                    } else {
                        fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SHOULDER_WALK.into(), true.into());
                    }
                } else {
                    fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SHOULDER_TURN.into(), true.into());
                }
            } else {
                temp = fighter.global_table[0x1d].get_i32() < WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("pass_flick_y"));
                
                if temp == false {
                    temp = false;
                } else {
                    if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND{
                        temp = false;
                    }
                    temp = GroundModule::is_passable_check(fighter.module_accessor);
                }
                if temp == false{
                    temp = fighter.global_table[0x1a].get_f32() * PostureModule::lr(fighter.module_accessor) <= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_stick_x"));
                    if temp == false {
                        temp = false;
                    }else{
                        temp = fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND;
                    }
                    if temp == false{
                        temp = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("walk_stick_x")) <= fighter.global_table[0x1a].get_f32() * PostureModule::lr(fighter.module_accessor);
                        if temp == false {
                            temp = false;
                        }else{
                            temp = fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND;
                        }
                        println!("temp {}", temp);
                        
                        if temp == false{
                            if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_AIR{
                                if MotionModule::is_end(fighter.module_accessor) == false{
                                  
                                } else {
                                    if MotionModule::motion_kind(fighter.module_accessor) != hash40("shoulder_wait"){
                                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("shoulder_wait"), 0.0, 1.0, false, 0.0, false, false);
                                    }
                                }
                                return 0.into();
                            }
                            fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SHOULDER_FALL.into(), false.into());
                        } else {
                            fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SHOULDER_WALK.into(), true.into());
                        }
                    } else {
                        fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SHOULDER_TURN.into(), true.into());
                    }
                    return 1.into();
                }
                fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SHOULDER_PASS.into(), true.into());
            }
        }
    }
    return 1.into();
}

pub unsafe extern "C" fn shoulderwait_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_GROUND), 
        *FIGHTER_KINETIC_TYPE_MOTION, 
        GROUND_CORRECT_KIND_GROUND_OTTOTTO.into(), 
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_FALL_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE
    );

    let aLStack200 = *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_TREADED_KIND_NO_REAC | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON;
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor, 
        true, 
        *FIGHTER_TREADED_KIND_NO_REAC, 
        false, 
        false, 
        false, 
        0, 
        0, 
        0, 
        0
    );

    return 0.into();
}

pub unsafe extern "C" fn shoulderwalk_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

pub unsafe extern "C" fn shoulderwalk_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_walk_uniq_process_main_common(hash40("shoulder_walk_fast").into(), hash40("shoulder_walk_middle").into(), hash40("shoulder_walk_slow").into(), false.into());  
    return 0.into();
}

pub unsafe extern "C" fn shoulderwalk_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_walk_uniq_process_init_common(hash40("shoulder_walk_fast").into(),hash40("shoulder_walk_middle").into(),hash40("shoulder_walk_slow").into(),false.into());
    return 0.into();
}

pub unsafe extern "C" fn shoulderwalk_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_shift_status_main(L2CValue::Ptr(shoulderwalk_main_loop as *const () as _))
}

pub unsafe extern "C" fn shoulderwalk_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    FUN_7100022520(alStack80, fighter);
    let mut temp;
    let mut tempbool;
    if alStack80 != true {
        FUN_7100023820(alStack80, fighter);
        temp = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("pass_stick_y"));
        if alStack80 != true {
            if ControlModule::get_stick_y(fighter.module_accessor) > temp {
                //loop_here
                if fighter.global_table[0x1a].get_f32()*PostureModule::lr(fighter.module_accessor) >= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("walk_stick_x")) {
                    tempbool = false;
                } else {
                    tempbool = fighter.global_table[0x16] == *SITUATION_KIND_GROUND;
                }
                if tempbool == false {
                    tempbool = fighter.global_table[0x16] == *SITUATION_KIND_AIR;
                    if tempbool == false {
                        return 0.into();
                    }
                    fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SHOULDER_FALL.into(), false.into());
                } else {
                    fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SHOULDER_WAIT.into(), true.into());
                }
            } else {
                if fighter.global_table[0x1d].get_i32() >= WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("pass_flick_y")) {
                    tempbool = false;
                } else {
                    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
                        tempbool = false;
                    } else {
                        tempbool = GroundModule::is_passable_check(fighter.module_accessor);
                    }
                }
                if tempbool == false {
                    if fighter.global_table[0x1a].get_f32()*PostureModule::lr(fighter.module_accessor) >= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("walk_stick_x")) {
                        tempbool = false;
                    } else {
                        tempbool = fighter.global_table[0x16] == *SITUATION_KIND_GROUND;
                    }
                    if tempbool == false {
                        tempbool = fighter.global_table[0x16] == *SITUATION_KIND_AIR;
                        if tempbool == false {
                            return 0.into();
                        }
                        fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SHOULDER_FALL.into(), false.into());
                    } else {
                        fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SHOULDER_WAIT.into(), true.into());
                    }
                    return 1.into();
                } else {
                    fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SHOULDER_PASS.into(), true.into());
                }
            }
        }
    }
    return 1.into();
}

pub unsafe extern "C" fn shoulderwalk_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_GROUND), 
        *FIGHTER_KINETIC_TYPE_MOTION_LOOP, 
        GROUND_CORRECT_KIND_GROUND_OTTOTTO.into(), 
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_FALL_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE
    );

    let aLStack200 = *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_TREADED_KIND_NO_REAC | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON;
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor, 
        true, 
        *FIGHTER_TREADED_KIND_NO_REAC, 
        false, 
        false, 
        false, 
        0, 
        *FIGHTER_STATUS_ATTR_SCALE_KINETIC_ENERGY as u32, 
        0, 
        0
    );

    return 0.into();
}

pub unsafe extern "C" fn throwfb_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    //fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    return 0.into();
}

pub unsafe extern "C" fn throwfb_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    FUN_7100021780(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(throwfhi_main_loop as *const () as _))
}

pub unsafe extern "C" fn throwfb_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_NONE), 
        *FIGHTER_KINETIC_TYPE_UNIQ, 
        GROUND_CORRECT_KIND_KEEP.into(), 
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_FALL_FLOAT,
        0
    );

    let aLStack200 = *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_THROW_LIFT_B | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_THROW;
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor, 
        false, 
        *FIGHTER_TREADED_KIND_NO_REAC, 
        false, 
        true, 
        false, 
        aLStack200 as u64, 
        0, 
        *FIGHTER_POWER_UP_ATTACK_BIT_THROW as u32, 
        0
    );

    return 0.into();
}

pub unsafe extern "C" fn throwff_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    //fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    return 0.into();
}

pub unsafe extern "C" fn throwff_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    FUN_7100021780(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(throwfhi_main_loop as *const () as _))
}

pub unsafe extern "C" fn throwff_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_NONE), 
        *FIGHTER_KINETIC_TYPE_UNIQ, 
        GROUND_CORRECT_KIND_KEEP.into(), 
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_FALL_FLOAT,
        0
    );

    let aLStack200 = *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_THROW_LIFT_F | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_THROW;
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor, 
        false, 
        *FIGHTER_TREADED_KIND_NO_REAC, 
        false, 
        true, 
        false, 
        aLStack200 as u64, 
        0, 
        *FIGHTER_POWER_UP_ATTACK_BIT_THROW as u32, 
        0
    );

    return 0.into();
}

pub unsafe extern "C" fn throwfhi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    //fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    return 0.into();
}

pub unsafe extern "C" fn throwfhi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    FUN_7100021780(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(throwfhi_main_loop as *const () as _))
}

pub unsafe extern "C" fn throwfhi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    FUN_7100021af0(alStack80,fighter);
    if alStack80 == true{
        return 1.into();
    } else {
        return 0.into();
    }
}

pub unsafe extern "C" fn throwfhi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_NONE), 
        *FIGHTER_KINETIC_TYPE_UNIQ, 
        GROUND_CORRECT_KIND_KEEP.into(), 
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_FALL_FLOAT,
        0
    );

    let aLStack200 = *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_THROW_LIFT_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_THROW;
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor, 
        false, 
        *FIGHTER_TREADED_KIND_NO_REAC, 
        false, 
        true, 
        false, 
        aLStack200 as u64, 
        0, 
        *FIGHTER_POWER_UP_ATTACK_BIT_THROW as u32, 
        0
    );

    return 0.into();
}

pub unsafe extern "C" fn throwflw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    //fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    return 0.into();
}

pub unsafe extern "C" fn throwflw_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    CatchModule::catch_cut(fighter.module_accessor, false, false);
    return 0.into();
}

pub unsafe extern "C" fn throwflw_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut compare = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
    let output;
    if compare != 0x9c72950bb {
        if compare != 0x9c04494a2 {
            if compare != 0xa249b58ab {
                output = 0x122daa1b33;
            } else {
                output = 0x12b3c9e354;
            }
        } else {
            output = 0x114026e3a9;
        }
    } else {
        output = 0x11474b27b0;
    }

    compare = 0xe9b37bc21;

    LinkModule::send_event_nodes_throw(fighter.module_accessor, Hash40::new_raw(output), Hash40::new_raw(compare), true, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO, *FIGHTER_STATUS_THROW_WORK_FLOAT_MOTION_RATE);
    WorkModule::set_int(fighter.module_accessor, LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE) as i32, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
    return 0.into();

}

pub unsafe extern "C" fn throwflw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    FUN_7100021780(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(throwflw_main_loop as *const () as _))
}

pub unsafe extern "C" fn throwflw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    FUN_7100021af0(alStack80,fighter);
    if alStack80 == false {
        let mut temp = WorkModule::get_param_int(fighter.module_accessor, hash40("param_private"), Hash40::new_raw(0x13a4988083).hash);
        FighterUtil::start_catch_stop(fighter.module_accessor, temp, 1.0);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_THROW_FLAG_STOP);
    }
    return 0.into();
}

pub unsafe extern "C" fn throwflw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_NONE), 
        *FIGHTER_KINETIC_TYPE_UNIQ, 
        GROUND_CORRECT_KIND_KEEP.into(), 
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_FALL_FLOAT,
        0
    );

    let aLStack200 = *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_THROW_LIFT_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_THROW;
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor, 
        false, 
        *FIGHTER_TREADED_KIND_NO_REAC, 
        false, 
        true, 
        false, 
        aLStack200 as u64, 
        0, 
        *FIGHTER_POWER_UP_ATTACK_BIT_THROW as u32, 
        0
    );

    return 0.into();
}

pub unsafe extern "C" fn shoulderlanding_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

pub unsafe extern "C" fn shoulderlanding_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("shoulder_landing"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(shoulderlanding_main_loop as *const () as _))
}

pub unsafe extern "C" fn shoulderlanding_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0x16] != *SITUATION_KIND_AIR {
        if MotionModule::is_end(fighter.module_accessor){
            if fighter.global_table[0x16] == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SHOULDER_WAIT.into(), false.into());
                return 1.into();
            }
        }
        return 0.into();
    } else {
        fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SHOULDER_FALL.into(), false.into());
        return 1.into();
    }
}

pub unsafe extern "C" fn shoulderlanding_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_GROUND), 
        *FIGHTER_KINETIC_TYPE_MOTION, 
        GROUND_CORRECT_KIND_GROUND_OTTOTTO.into(), 
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_FALL_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE
    );
    
    let aLStack200 = *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_TREADED_KIND_NO_REAC | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON;
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor, 
        true, 
        *FIGHTER_TREADED_KIND_NO_REAC, 
        false, 
        false, 
        false, 
        0, 
        0, 
        0, 
        0
    );

    return 0.into();
}

unsafe extern "C" fn game_throwfb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 13.0, 72, 52, 0, 85, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
        macros::CHECK_FINISH_CAMERA(agent, 26, 14);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.4);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 12.0, y: 1.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::REVERSE_LR(agent);
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

unsafe extern "C" fn game_throwff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.0, 60, 46, 0, 90, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::CHECK_FINISH_CAMERA(agent, 38, 19);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 12.0, y: 6.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

unsafe extern "C" fn game_throwfhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.0, 92, 53, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::CHECK_FINISH_CAMERA(agent, 1, 31);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.9);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

unsafe extern "C" fn game_throwflw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 11.0, 46, 45, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_THROW_FLAG_STOP);
        macros::CHECK_FINISH_CAMERA(agent, 4, 2);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.2);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        macros::SET_SPEED_EX(agent, 0, 1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
}

unsafe extern "C" fn game_throwf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 361, 100, 30, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_MARIO_STATUS_SHOULDER_START_FLAG_SHOULDER_WAIT);
    }
}

pub fn install() {
    skyline::install_hooks!(
        mario_opff
    );
    Agent::new("mario")
        .on_start(fighter_on)
        .status(End, FIGHTER_MARIO_STATUS_KIND_SHOULDER_END, shoulderend_end)
        .status(Main, FIGHTER_MARIO_STATUS_KIND_SHOULDER_END, shoulderend_main)
        .status(Pre, FIGHTER_MARIO_STATUS_KIND_SHOULDER_END, shoulderend_pre)

        .status(End, FIGHTER_MARIO_STATUS_KIND_SHOULDER_FALL, shoulderfall_end)
        .status(Pre, FIGHTER_MARIO_STATUS_KIND_SHOULDER_FALL, shoulderfall_pre)

        .status(End, FIGHTER_MARIO_STATUS_KIND_SHOULDER_PASS, shoulderpass_end)
        .status(Main, FIGHTER_MARIO_STATUS_KIND_SHOULDER_PASS, shoulderpass_main)
        .status(Pre, FIGHTER_MARIO_STATUS_KIND_SHOULDER_PASS, shoulderpass_pre)
        .status(Exit, FIGHTER_MARIO_STATUS_KIND_SHOULDER_PASS, shoulderpass_exit)
        .status(CheckDamage, FIGHTER_MARIO_STATUS_KIND_SHOULDER_PASS, check_dmg)

        .status(End, FIGHTER_MARIO_STATUS_KIND_SHOULDER_LANDING, shoulderlanding_end)
        .status(Main, FIGHTER_MARIO_STATUS_KIND_SHOULDER_LANDING, shoulderlanding_main)
        .status(Pre, FIGHTER_MARIO_STATUS_KIND_SHOULDER_LANDING, shoulderlanding_pre)


        .status(End, FIGHTER_MARIO_STATUS_KIND_SHOULDER_START, shoulderstart_end)
        .status(Init, FIGHTER_MARIO_STATUS_KIND_SHOULDER_START, shoulderstart_init)
        .status(Main, FIGHTER_MARIO_STATUS_KIND_SHOULDER_START, shoulderstart_main)
        .status(Pre, FIGHTER_MARIO_STATUS_KIND_SHOULDER_START, shoulderstart_pre)


        .status(End, FIGHTER_MARIO_STATUS_KIND_SHOULDER_TURN, shoulderturn_end)
        .status(Main, FIGHTER_MARIO_STATUS_KIND_SHOULDER_TURN, shoulderturn_main)
        .status(Pre, FIGHTER_MARIO_STATUS_KIND_SHOULDER_TURN, shoulderturn_pre)


        .status(End, FIGHTER_MARIO_STATUS_KIND_SHOULDER_WAIT, shoulderwait_end)
        .status(Main, FIGHTER_MARIO_STATUS_KIND_SHOULDER_WAIT, shoulderwait_main)
        .status(Pre, FIGHTER_MARIO_STATUS_KIND_SHOULDER_WAIT, shoulderwait_pre)


        .status(End, FIGHTER_MARIO_STATUS_KIND_SHOULDER_WALK, shoulderwalk_end)
        .status(Main, FIGHTER_MARIO_STATUS_KIND_SHOULDER_WALK, shoulderwalk_main)
        .status(Exec, FIGHTER_MARIO_STATUS_KIND_SHOULDER_WALK, shoulderwalk_exec)
        .status(Init, FIGHTER_MARIO_STATUS_KIND_SHOULDER_WALK, shoulderwalk_init)
        .status(Pre, FIGHTER_MARIO_STATUS_KIND_SHOULDER_WALK, shoulderwalk_pre)


        .status(End, FIGHTER_MARIO_STATUS_KIND_THROW_F_B, throwfb_end)
        .status(Init, FIGHTER_MARIO_STATUS_KIND_THROW_F_B, throwflw_init)
        .status(Main, FIGHTER_MARIO_STATUS_KIND_THROW_F_B, throwfb_main)
        .status(Pre, FIGHTER_MARIO_STATUS_KIND_THROW_F_B, throwfb_pre)

        .status(End, FIGHTER_MARIO_STATUS_KIND_THROW_F_F, throwff_end)
        .status(Init, FIGHTER_MARIO_STATUS_KIND_THROW_F_F, throwflw_init)
        .status(Main, FIGHTER_MARIO_STATUS_KIND_THROW_F_F, throwff_main)
        .status(Pre, FIGHTER_MARIO_STATUS_KIND_THROW_F_F, throwff_pre)

        .status(End, FIGHTER_MARIO_STATUS_KIND_THROW_F_LW, throwflw_end)
        .status(Main, FIGHTER_MARIO_STATUS_KIND_THROW_F_LW, throwflw_main)
        .status(Pre, FIGHTER_MARIO_STATUS_KIND_THROW_F_LW, throwflw_pre)
        .status(Init, FIGHTER_MARIO_STATUS_KIND_THROW_F_LW, throwflw_init)
        .status(Exit, FIGHTER_MARIO_STATUS_KIND_THROW_F_LW, throwflw_exit)

        .status(End, FIGHTER_MARIO_STATUS_KIND_THROW_F_HI, throwfhi_end)
        .status(Init, FIGHTER_MARIO_STATUS_KIND_THROW_F_HI, throwflw_init)
        .status(Main, FIGHTER_MARIO_STATUS_KIND_THROW_F_HI, throwfhi_main)
        .status(Pre, FIGHTER_MARIO_STATUS_KIND_THROW_F_HI, throwfhi_pre)

        .game_acmd("game_throwff", game_throwff, Default)
        .game_acmd("game_throwfb", game_throwfb, Default)
        .game_acmd("game_throwflw", game_throwfhi, Default)
        .game_acmd("game_throwfhi", game_throwflw, Default)
        .game_acmd("game_throwf", game_throwf, Default)

        //.status(End, FIGHTER_MARIO_STATUS_KIND_SHOULDER_WALK, shoulderwalk_end)
        //.status(Init, FIGHTER_MARIO_STATUS_KIND_SHOULDER_WALK, shoulderwalk_init)
        .install();
}


