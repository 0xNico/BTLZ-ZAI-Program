use anchor_lang::prelude::*;

#[account]
pub struct Player {
    pub player_id: Pubkey,
    pub xp: i64,
    pub chests: u64,
    pub active_class: u64,
    pub active_weapon: u64,
    pub joined: i64,
}

impl Player {
    pub const LEN: usize = 88; // Total space required for the struct
}
