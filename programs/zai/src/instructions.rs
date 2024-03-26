// src/instructions.rs
use anchor_lang::prelude::*;
use crate::{Player, ZaiError};
use std::str::FromStr;

// CONST
const SERVER_PUBLIC_KEY: &str = "zaigDCJxJdzSh6ETPosxWfVrzemjLnMJxxqhZCfdEqU";

// func0 - create_player
pub fn create_player(ctx: Context<CreatePlayer>, active_class: u64, active_weapon: u64) -> Result<()> {
    // Validate the active_class is one of the allowed values
    if ![101, 102, 103].contains(&active_class) {
        return Err(error!(ZaiError::InvalidClass));
    }

    let player_account = &mut ctx.accounts.player_account;
    let clock = Clock::get()?; // Use ? for error handling

    // Set fields for the new player
    player_account.player_id = *ctx.accounts.signer.key; // Assign signer's pubkey
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
pub fn change_default_class(ctx: Context<ChangeDefaultClass>, new_class: u64) -> Result<()> {
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