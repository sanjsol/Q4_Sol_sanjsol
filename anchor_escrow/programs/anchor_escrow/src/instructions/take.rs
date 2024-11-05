use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked, CloseAccount, close_account}};

use crate::state::Escrow;

#[derive(Accounts)]
pub struct Take<'info> {
  #[account(mut)]
  pub taker: Signer<'info>,
  #[account(mut)]
  pub maker: SystemAccount<'info>,
  pub mint_a: InterfaceAccount<'info, Mint>,
  pub mint_b: InterfaceAccount<'info, Mint>,
  #[account(
    init_if_needed,
    payer = taker,
    associated_token::mint = mint_a,
    associated_token::authority = taker
  )]
  pub taker_ata_a: Box<InterfaceAccount<'info, TokenAccount>>,
  #[account(
    mut,
    associated_token::mint = mint_b,
    associated_token::authority = taker
  )]
  pub taker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>,
  #[account(
    init_if_needed,
    payer = taker,
    associated_token::mint = mint_b,
    associated_token::authority = maker
  )]
  pub maker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>,
  #[account(
    mut,
    close = maker,
    has_one = maker,
    has_one = mint_a,
    has_one = mint_b,
    seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
    bump = escrow.bump
  )]
  pub escrow: Account<'info, Escrow>,
  #[account(
    mut,
    associated_token::mint = mint_a,
    associated_token::authority = escrow
  )]
  pub vault: InterfaceAccount<'info, TokenAccount>,
  pub associated_token_program: Program<'info, AssociatedToken>,
  pub token_program: Interface<'info, TokenInterface>,
  pub system_program: Program<'info, System>
}

impl<'info> Take<'info> {
  pub fn deposit(&mut self) -> Result<()> {

    let cpi_accounts = TransferChecked {
      from: self.taker_ata_b.to_account_info(),
      to: self.maker_ata_b.to_account_info(),
      mint: self.mint_b.to_account_info(),
      authority: self.taker.to_account_info()
    };

    let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

    transfer_checked(cpi_ctx, self.escrow.receive, self.mint_b.decimals)?;

    Ok(())
  }

  pub fn withdraw_and_close_vault(&mut self) -> Result<()> {
    
    let cpi_accounts = TransferChecked {
      from: self.vault.to_account_info(),
      to: self.taker_ata_a.to_account_info(),
      mint: self.mint_a.to_account_info(),
      authority: self.escrow.to_account_info()
    };

    let seeds = [b"escrow", self.maker.key.as_ref(), &self.escrow.seed.to_le_bytes()[..], &[self.escrow.bump]];

    let signer_seeds = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), cpi_accounts, signer_seeds);

    transfer_checked(cpi_ctx, self.vault.amount, self.mint_a.decimals)?;

    let cpi_accounts = CloseAccount {
      account: self.vault.to_account_info(),
      destination: self.maker.to_account_info(),
      authority: self.escrow.to_account_info()
    };

    let cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), cpi_accounts, signer_seeds);

    close_account(cpi_ctx)?;

    Ok(())
  }
}

