use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, metadata::{
        MasterEditionAccount, Metadata, MetadataAccount,
    }, token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked}
};

use crate::state::{Listing, Marketplace};

#[derive(Accounts)]
#[instruction(name: String)]
pub struct List<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        seeds = [b"marketplace", name.as_bytes()],
        bump = marketplace.bump
    )]
    pub marketplace: Account<'info, Marketplace>,
    #[account(
        init,
        associated_token::mint = mint,
        associated_token::authority = marketplace,
        payer = maker
    )]
    pub vault: Account<'info, TokenAccount>,
    #[account(
        init,
        seeds = [b"listing", marketplace.key().as_ref(), mint.key().as_ref()],
        bump,
        space = Listing::INIT_SPACE,
        payer = maker,
    )]
    pub listing: Account<'info, Listing>,
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = maker
    )]
    pub mint_ata: Account<'info, TokenAccount>,
    pub collection: Account<'info, Mint>,
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref()
        ],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true,
    )]
    pub metadata: Account<'info, MetadataAccount>,
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition"
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub edition: Account<'info, MasterEditionAccount>,
    #[account(
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump = marketplace.rewards_bump,
    )]
    pub rewards: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = rewards,
        associated_token::authority = maker
    )]
    pub maker_rewards_ata: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>
}

impl<'info> List<'info> {
    pub fn create_list(&mut self, bumps: &ListBumps, price: u64) -> Result<()> {

        self.listing.set_inner(Listing {
            price,
            bump: bumps.listing,
            maker: self.maker.key(),
            mint: self.mint.key(),
        });

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            to: self.vault.to_account_info(),
            mint: self.mint.to_account_info(),
            authority: self.maker.to_account_info(),
            from: self.mint_ata.to_account_info()
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, 1, self.mint.decimals)?;

        Ok(())
    }
}