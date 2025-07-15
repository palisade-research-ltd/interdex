use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        mint::decimals = USDC_DECIMALS,
        mint::authority = vault_pda,
        mint::freeze_authority = vault_pda
    )]
    pub usdv_mint: Account<'info, Mint>,
    
    /// CHECK: This PDA is derived from seeds and used as authority for vault operations
    #[account(
        seeds = [b"vault", usdv_mint.key().as_ref()],
        bump,
    )]
    pub vault_pda: UncheckedAccount<'info>,
    
    #[account(
        init,
        payer = payer,
        token::mint = usdc_mint,
        token::authority = vault_pda,
    )]
    pub vault_ata: Account<'info, TokenAccount>,
    
    #[account(constraint = usdc_mint.decimals == USDC_DECIMALS)]
    pub usdc_mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintUsdv<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,
    
    #[account(mut, token::mint = usdc_mint, token::authority = depositor)]
    pub depositor_ata: Account<'info, TokenAccount>,
    
    #[account(mut, token::mint = usdc_mint, token::authority = vault_pda)]
    pub vault_ata: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub receiver_ata: Account<'info, TokenAccount>,
    
    pub usdc_mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub usdv_mint: Account<'info, Mint>,
    
    /// CHECK: This PDA is derived from seeds and used as authority for vault operations
    #[account(
        seeds = [b"vault", usdv_mint.key().as_ref()],
        bump
    )]
    pub vault_pda: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct BurnUsdv<'info> {
    #[account(mut)]
    pub burner: Signer<'info>,
    
    #[account(mut, token::mint = usdv_mint, token::authority = burner)]
    pub burner_ata: Account<'info, TokenAccount>,
    
    #[account(mut, token::mint = usdc_mint, token::authority = vault_pda)]
    pub vault_ata: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub receiver_ata: Account<'info, TokenAccount>,
    
    pub usdc_mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub usdv_mint: Account<'info, Mint>,
    
    /// CHECK: This PDA is derived from seeds and used as authority for vault operations
    #[account(
        seeds = [b"vault", usdv_mint.key().as_ref()],
        bump
    )]
    pub vault_pda: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token>,
}
