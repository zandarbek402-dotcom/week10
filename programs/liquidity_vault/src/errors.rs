use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("Amount must be greater than zero")]
    ZeroAmount,
    #[msg("Vault is paused")]
    VaultPaused,
    #[msg("Deposit would exceed the vault deposit cap")]
    DepositCapExceeded,
    #[msg("Fee is too high")]
    FeeTooHigh,
    #[msg("Not enough shares")]
    InsufficientShares,
    #[msg("Calculated shares amount is zero (deposit too small)")]
    ZeroShares,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Only the vault admin can do this")]
    Unauthorized,
    #[msg("Slippage: result is below the minimum requested")]
    SlippageExceeded,
    #[msg("Not enough tokens in user account")]
    InsufficientFunds,
    #[msg("Not enough idle liquidity in the vault")]
    InsufficientLiquidity,
    #[msg("Position still has shares, withdraw them first")]
    PositionNotEmpty,
}
