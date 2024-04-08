use anchor_lang::prelude::*;
use crate::{errors::ZaiError, instructions::*, player::Player, player::PremiumItemType};

mod errors;
mod instructions;
mod player;

declare_id!("HWjAY4TNEiAQquRKmwRXMabXf1PMGp36QyQgA162XdNr");

#[program]
pub mod zai {
    use super::*;
    use crate::instructions::{
        change_default_class as cdc, create_player as cp, equip_premium_item as epi,
        increase_player_level as ipl, modify_player_xp as mpxp,
    };

    // Wrapper for create_player
    pub fn create_player(
        ctx: Context<CreatePlayer>,
        active_class: u8,
        active_weapon: u8,
    ) -> Result<()> {
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

    // Wrapper for increase_player_level
    pub fn increase_player_level(ctx: Context<IncreasePlayerLevel>) -> Result<()> {
        ipl(ctx)
    }

    // Wrapper for equip_premium_item
    pub fn equip_premium_item(
        ctx: Context<EquipPremiumItem>,
        item_type: PremiumItemType,
        item_id: u8,
    ) -> Result<()> {
        epi(ctx, item_type, item_id)
    }
}
