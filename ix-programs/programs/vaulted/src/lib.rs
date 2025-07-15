#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;
use anchor_spl::token::{self as spl_token};

declare_id!("ACgeeZmREHqAXoSgvCjot191Y4Vbpzuguw1pHgVeDzDv");

pub mod errors;

pub mod state;


pub const USDC_DECIMALS: u8 = 6;

#[program]
pub mod vaulted {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn mint_usdv(ctx: Context<state::MintUsdv>, amount: u64) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);
        
        let mint_key = ctx.accounts.usdv_mint.key();
        let seeds: &[&[u8]] = &[
            b"vault",
            mint_key.as_ref(),
            &[ctx.bumps.vault_pda],
        ];

        // Transfer USDC from user to vault
        spl_token::transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                spl_token::TransferChecked {
                    from: ctx.accounts.depositor_ata.to_account_info(),
                    mint: ctx.accounts.usdc_mint.to_account_info(),
                    to: ctx.accounts.vault_ata.to_account_info(),
                    authority: ctx.accounts.depositor.to_account_info(),
                },
            ),
            amount,
            USDC_DECIMALS,
        )?;

        // Mint USDv to user
        spl_token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                spl_token::MintTo {
                    mint: ctx.accounts.usdv_mint.to_account_info(),
                    to: ctx.accounts.receiver_ata.to_account_info(),
                    authority: ctx.accounts.vault_pda.to_account_info(),
                },
                &[seeds],
            ),
            amount,
        )
    }

    pub fn burn_usdv(ctx: Context<BurnUsdv>, amount: u64) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);
        
        let mint_key = ctx.accounts.usdv_mint.key();
        let seeds: &[&[u8]] = &[
            b"vault",
            mint_key.as_ref(),
            &[ctx.bumps.vault_pda],
        ];

        // Burn USDv from user
        spl_token::burn(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                spl_token::Burn {
                    mint: ctx.accounts.usdv_mint.to_account_info(),
                    from: ctx.accounts.burner_ata.to_account_info(),
                    authority: ctx.accounts.burner.to_account_info(),
                },
            ),
            amount,
        )?;

        // Transfer USDC from vault to user
        spl_token::transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                spl_token::TransferChecked {
                    from: ctx.accounts.vault_ata.to_account_info(),
                    mint: ctx.accounts.usdc_mint.to_account_info(),
                    to: ctx.accounts.receiver_ata.to_account_info(),
                    authority: ctx.accounts.vault_pda.to_account_info(),
                },
                &[seeds],
            ),
            amount,
            USDC_DECIMALS,
        )
    }
}

