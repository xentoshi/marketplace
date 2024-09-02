use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
pub mod error;

use instructions::*;

declare_id!("3nWjmxxNfat1kftF8RyEdeDvNSEmK1WTtety4xS141r2");


#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.init(name, fee, &ctx.bumps)
    }

    pub fn list(ctx: Context<List>, _name: String, price: u64) -> Result<()> {
        ctx.accounts.create_list(&ctx.bumps, price)
    }
    pub fn unlist(ctx: Context<Delist>, name: String) -> Result<()> {
        ctx.accounts.delist(name)
    }

    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.send_sol()?;
        ctx.accounts.send_nft()?;
        ctx.accounts.close_mint_vault()
    }

}