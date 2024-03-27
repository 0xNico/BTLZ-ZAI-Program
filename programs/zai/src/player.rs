use anchor_lang::prelude::*;

#[account] // 8 bytes
pub struct Player {
    pub player_id: Pubkey, //32 bytes
    pub level: u8, // 1 byte
    pub xp: i64, // 8 bytes
    pub chests: u16, // 2 bytes
    pub active_class: u8, // 1 byte
    pub active_weapon: u8, // 1 byte
    pub joined: i64, // 8 bytes
}

impl Player {
    pub const LEN: usize = 64; // 61 byte total + 3 byte buffer.
    pub const LEVEL_CAP: u8 = 100;
}
