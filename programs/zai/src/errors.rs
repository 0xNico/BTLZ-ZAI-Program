use anchor_lang::prelude::*;

#[error_code]
pub enum ZaiError {
    #[msg("Invalid class selected. Valid options are 101, 102, or 103.")]
    InvalidClass,

    #[msg("Active weapon must match the active class.")]
    WeaponClassMismatch,

    #[msg("Changing to the same class is not allowed.")]
    ClassChangeToSameNotAllowed,

    #[msg("Unauthorized attempt to modify player account- Server did not sign.")]
    Unauthorized,

    #[msg("XP modification resulted in overflow.")]
    XpOverflow,

    #[msg("Player has reached the level cap.")]
    LevelCapReached,

    #[msg("Player does not have enough XP to level up.")]
    NotEnoughXp,

    #[msg("Attempted to switch to default class.")]
    InvalidPremiumClass,

    #[msg("Attempted to switch to default weapon.")]
    InvalidPremiumWeapon,
}
