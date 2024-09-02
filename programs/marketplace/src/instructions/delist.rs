use anchor_lang::prelude::*;
use anchor_spl::token::{
        close_account, transfer_checked, CloseAccount, Mint, Token, TokenAccount, TransferChecked,
    };

use crate::state::{Listing, Marketplace};

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Delist<'info> {
    #[account(mut)]
    pub maker: Signer<'info>, // The user who created the listing and wants to delist

    #[account(
        seeds = [b"marketplace", name.as_bytes()],
        bump = marketplace.bump
    )]
    pub marketplace: Account<'info, Marketplace>, // The marketplace account

    #[account(
        mut,
        close = maker,  // Close this account and send rent to maker
        seeds = [b"listing", marketplace.key().as_ref(), mint.key().as_ref()],
        bump = listing.bump,
        has_one = mint,
        has_one = maker,
    )]
    pub listing: Account<'info, Listing>, // The listing account to be closed

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = marketplace,
    )]
    pub vault: Account<'info, TokenAccount>, // The vault holding the listed token

    pub mint: Account<'info, Mint>, // The mint of the token being delisted

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = maker
    )]
    pub mint_ata: Account<'info, TokenAccount>, // The maker's associated token account

    pub token_program: Program<'info, Token>, // The SPL Token program
}

impl<'info> Delist<'info> {
    pub fn delist(&mut self, name: String) -> Result<()> {
        // Prepare the CPI (Cross-Program Invocation) to transfer the token back to the maker
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            to: self.mint_ata.to_account_info(),
            from: self.vault.to_account_info(),
            mint: self.mint.to_account_info(),
            authority: self.marketplace.to_account_info(),
        };

        // Create the seeds for PDA signing
        let seeds = &[b"marketplace", name.as_bytes(), &[self.marketplace.bump]];

        let signer_seeds = &[&seeds[..]];

        // Create the CPI context with signer seeds
        let cpi_ctx = CpiContext::new_with_signer(cpi_program.clone(), cpi_accounts, signer_seeds);

        // Perform the token transfer
        transfer_checked(cpi_ctx, 1, self.mint.decimals)?;

        // Prepare the CPI to close the vault account
        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.marketplace.to_account_info(),
        };

        // Create the CPI context for closing the account
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        // Close the vault account
        close_account(cpi_ctx)?;

        Ok(())
    }
}
