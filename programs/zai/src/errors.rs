use anchor_lang::prelude::*;

#[error_code]
pub enum ZaiError {
    #[msg("Invalid class selected. Valid options are 101, 102, or 103.")]
    InvalidClass,

    #[msg("Active weapon must match the active class.")]
    WeaponClassMismatch,

    #[msg("Changing to the same class is not allowed.")]
    ClassChangeToSameNotAllowed,

    #[msg("Unauthorized attempt to modify XP.")]
    Unauthorized,

    #[msg("XP modification resulted in overflow.")]
    XpOverflow,
}

