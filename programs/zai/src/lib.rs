use anchor_lang::prelude::*;
use crate::{errors::ZaiError, player::Player, instructions::*};

mod errors;
mod player;
mod instructions;

declare_id!("HWjAY4TNEiAQquRKmwRXMabXf1PMGp36QyQgA162XdNr");

#[program]
pub mod zai {
    use super::*;
    use crate::instructions::{create_player as cp, change_default_class as cdc, modify_player_xp as mpxp};

    // Wrapper for create_player
    pub fn create_player(ctx: Context<CreatePlayer>, active_class: u8, active_weapon: u8) -> Result<()> {
        cp(ctx, active_class, active_weapon)
    }

    // Wrapper for change_default_class
    pub fn change_default_class(ctx: Context<ChangeDefaultClass>, new_class: u8) -> Result<()> {
        cdc(ctx, new_class)
    }

    // Wrapper for modify_player_xp
    pub fn modify_player_xp(ctx: Context<ModifyPlayerXp>, xp_change: i64) -> Result<()> {
        mpxp(ctx, xp_change)
    }
}