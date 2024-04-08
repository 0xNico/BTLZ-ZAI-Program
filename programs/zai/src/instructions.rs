// src/instructions.rs
use anchor_lang::prelude::*;
use crate::{Player, PremiumItemType, ZaiError};
use std::str::FromStr;

// CONST
const SERVER_PUBLIC_KEY: &str = "zaigDCJxJdzSh6ETPosxWfVrzemjLnMJxxqhZCfdEqU";

// func0 - create_player
pub fn create_player(ctx: Context<CreatePlayer>, active_class: u8, active_weapon: u8) -> Result<()> {
    // Validate the active_class is one of the allowed values
    if ![101, 102, 103].contains(&active_class) {
        return Err(error!(ZaiError::InvalidClass));
    }

    let player_account = &mut ctx.accounts.player_account;
    let clock = Clock::get()?; // Use ? for error handling

    // Set fields for the new player
    player_account.player_id = *ctx.accounts.signer.key; // Assign signer's pubkey
    player_account.level = 1;
    player_account.xp = 0;
    player_account.chests = 0;
    player_account.active_class = active_class; // Now validated to match weapon
    player_account.active_weapon = active_weapon; // Confirmed to match class
    player_account.joined = clock.unix_timestamp;
    
    msg!("Player account {} created for player {} with class {} and weapon {}.", player_account.player_id, player_account.player_id, player_account.active_class, player_account.active_weapon);
    Ok(())
}
// func0 - create_player - ACC.
#[derive(Accounts)]
pub struct CreatePlayer<'info> {
    #[account(init, payer = signer, space = Player::LEN, seeds = [b"player", signer.key().as_ref()], bump)]
    pub player_account: Account<'info, Player>, // This is the PDA account for player data
    #[account(mut)]
    pub signer: Signer<'info>, // This is the player creating the account
    pub system_program: Program<'info, System>,
}
// func0 - create_player - END.

// func1 - change_default_class
pub fn change_default_class(ctx: Context<ChangeDefaultClass>, new_class: u8) -> Result<()> {
    let player = &mut ctx.accounts.player_account;

    // Check if the new class is different from the current class
    if player.active_class == new_class {
        return Err(error!(ZaiError::ClassChangeToSameNotAllowed));
    }

    // Validate the new class is one of the allowed values
    if ![101, 102, 103].contains(&new_class) {
        return Err(error!(ZaiError::InvalidClass));
    }

    // Update the player's class and weapon to the new class
    player.active_class = new_class;
    player.active_weapon = new_class; // Assuming the weapon should match the class

    msg!("Player {} class changed to {}.", player.player_id, player.active_class);
    Ok(())
}
// func1 - change_default_class - ACC.
#[derive(Accounts)]
pub struct ChangeDefaultClass<'info> {
    #[account(mut)]
    pub player_account: Account<'info, Player>,
    // Ensure that the signer is the player attempting to change their class
    #[account(mut, constraint = player_account.player_id == *signer.key)]
    pub signer: Signer<'info>,
}
// func1 - change_default_class - END.

// func2 - modify_player_xp
pub fn modify_player_xp(ctx: Context<ModifyPlayerXp>, xp_change: i64) -> Result<()> {
    let server_pubkey = Pubkey::from_str(SERVER_PUBLIC_KEY).unwrap();
    require_keys_eq!(ctx.accounts.admin.key(), server_pubkey, ZaiError::Unauthorized);

    let player = &mut ctx.accounts.player_account;
    // Implement logic to safely add xp_change to player.xp considering overflows or underflows
    player.xp = player.xp.checked_add(xp_change).ok_or(ZaiError::XpOverflow)?;
    
    msg!("Player {} XP modified by {}.", player.player_id, xp_change);
    Ok(())
}
//func2 - modify_player_xp - ACC.
#[derive(Accounts)]
pub struct ModifyPlayerXp<'info> {
    #[account(mut)]
    pub player_account: Account<'info, Player>,
    /// CHECK: This is only checked for authorization purposes, not dereferenced
    pub admin: Signer<'info>,
}
//func2 - modify_player_xp - END.

//func3 - increase_player_level
pub fn increase_player_level(ctx: Context<IncreasePlayerLevel>) -> Result<()> {
    let player = &mut ctx.accounts.player_account;
    // Ensure the player is not already at or above the level cap
    if player.level >= Player::LEVEL_CAP {
        return Err(ZaiError::LevelCapReached.into());
    }
    
    // Calculate the required XP for the next level using the exponential curve
    let required_xp_for_next_level = (4250.0 * f64::exp(0.055 * (player.level as f64))).round() as i64;

    // Check if the player has enough XP to level up
    if player.xp < required_xp_for_next_level {
        return Err(ZaiError::NotEnoughXp.into());
    }

    // Level up
    player.level += 1;
    // Subtract the required XP for leveling up from the player's current XP
    player.xp -= required_xp_for_next_level;

    // Attempt to give a chest with a 10% chance
    let current_slot = Clock::get()?.slot;
    let player_specific_data = player.player_id.to_bytes(); // Assuming player_id is a Pubkey
    let randomness_seed = current_slot.wrapping_add(u64::from_le_bytes(player_specific_data[0..8].try_into().unwrap())); // Simple mix of slot and player_id
    let chest_chance = randomness_seed % 10; // This gives us a value from 0 to 9

    if chest_chance == 0 { // 10% chance
        player.chests += 1;
        msg!("Congratulations! You've received a chest for leveling up.");
    }

    // Log the required XP for the next level and the player's new XP after leveling up
    msg!("Required XP for next level: {}", required_xp_for_next_level);
    msg!("Player's XP after leveling up: {}", player.xp);

    Ok(())
}

//func3 - increase_player_level - ACC.
#[derive(Accounts)]
pub struct IncreasePlayerLevel<'info> {
    #[account(mut, constraint = player_account.level < Player::LEVEL_CAP && player_account.xp >= (4250.0 * f64::exp(0.055 * (player_account.level as f64))).round() as i64)]
    pub player_account: Account<'info, Player>,
    // Ensure that the signer is the player attempting to level up
    #[account(mut, constraint = player_account.player_id == *signer.key)]
    pub signer: Signer<'info>,
}
//func3 - END.

// func4 - equip_premium_item - START.
pub fn equip_premium_item(ctx: Context<EquipPremiumItem>, item_type: PremiumItemType, item_id: u8) -> Result<()> {
    let server_pubkey = Pubkey::from_str(SERVER_PUBLIC_KEY).unwrap();
    // Ensure only the admin can execute this function
    require_keys_eq!(ctx.accounts.admin.key(), server_pubkey, ZaiError::Unauthorized);

    let player = &mut ctx.accounts.player_account;

    match item_type {
        PremiumItemType::Class => {
            // Ensure the new class is not one of the default free classes
            if [101, 102, 103].contains(&item_id) {
                return Err(error!(ZaiError::InvalidPremiumClass));
            }
            player.active_class = item_id;
        },
        PremiumItemType::Weapon => {
            // Ensure the new weapon is not one of the default free weapons
            if [101, 102, 103].contains(&item_id) { 
                return Err(error!(ZaiError::InvalidPremiumWeapon));
            }
            player.active_weapon = item_id;
        },
    }

    msg!("Player {} equipped premium item {} of type {:?}.", player.player_id, item_id, item_type);
    Ok(())
}

// func4 - equip_premium_item - ACC.
#[derive(Accounts)]
pub struct EquipPremiumItem<'info> {
    #[account(mut)]
    pub player_account: Account<'info, Player>,
    /// CHECK: This is only checked for authorization purposes, not dereferenced
    pub admin: Signer<'info>,
}
// func4 - END.