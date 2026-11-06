use anchor_lang::prelude::*;

pub mod constants;
pub mod token_cpi;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod math;
pub mod state;

use instructions::*;

declare_id!("264Fck2jjphRCD3vZ5uKA9zzKnzhjXsqGjpNsY2YopeZ");

/// Liquidity Vault.
/// Week 8:  skeleton (config, user PDA, share mint).
/// Week 9:  deposit — tokens in, share tokens out.
/// Week 10: CPI & withdraw — shares burned, vault returns tokens via the
///          Token program with a PDA signature.
#[program]
pub mod liquidity_vault {
    use super::*;

    /// Admin creates the vault: config PDA + share mint PDA + vault token account PDA
    pub fn initialize_vault(
        ctx: Context<InitializeVault>,
        fee_bps: u16,
        deposit_cap: u64,
    ) -> Result<()> {
        instructions::initialize_vault::handler(ctx, fee_bps, deposit_cap)
    }

    /// (optional) User creates their position PDA explicitly.
    /// `deposit` also creates it automatically on the first deposit.
    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        instructions::initialize_user::handler(ctx)
    }

    /// Deposit underlying tokens, receive shares (with slippage protection)
    pub fn deposit(ctx: Context<Deposit>, amount: u64, min_shares_out: u64) -> Result<()> {
        instructions::deposit::handler(ctx, amount, min_shares_out)
    }

    /// Read-only: shares that `amount` would mint right now
    pub fn preview_deposit(ctx: Context<PreviewDeposit>, amount: u64) -> Result<u64> {
        instructions::preview_deposit::handler(ctx, amount)
    }

    /// Burn shares (CPI, user signs) and receive underlying tokens
    /// (CPI transfer_checked, config PDA signs)
    pub fn withdraw(ctx: Context<Withdraw>, shares: u64, min_assets_out: u64) -> Result<()> {
        instructions::withdraw::handler(ctx, shares, min_assets_out)
    }

    /// Read-only: tokens (after fee) that burning `shares` would return right now
    pub fn preview_withdraw(ctx: Context<PreviewWithdraw>, shares: u64) -> Result<u64> {
        instructions::preview_withdraw::handler(ctx, shares)
    }

    /// Close an empty user position and get the rent back
    pub fn close_position(ctx: Context<ClosePosition>) -> Result<()> {
        instructions::close_position::handler(ctx)
    }

    /// Admin: change fee / cap / pause
    pub fn update_config(
        ctx: Context<UpdateConfig>,
        fee_bps: Option<u16>,
        deposit_cap: Option<u64>,
        paused: Option<bool>,
    ) -> Result<()> {
        instructions::update_config::handler(ctx, fee_bps, deposit_cap, paused)
    }
}
