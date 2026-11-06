use anchor_lang::prelude::*;

/// Emitted on every successful deposit
#[event]
pub struct DepositEvent {
    pub user: Pubkey,
    pub config: Pubkey,
    /// Underlying tokens deposited
    pub amount: u64,
    /// Share tokens minted to the user
    pub shares: u64,
    /// Vault state AFTER the deposit
    pub total_assets: u64,
    pub total_shares: u64,
    pub timestamp: i64,
}

/// Emitted on every successful withdraw
#[event]
pub struct WithdrawEvent {
    pub user: Pubkey,
    pub config: Pubkey,
    /// Share tokens burned
    pub shares: u64,
    /// Underlying tokens sent to the user (after fee)
    pub amount: u64,
    /// Fee kept in the vault
    pub fee: u64,
    /// User's shares remaining
    pub shares_left: u64,
    pub timestamp: i64,
}
