use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod snip {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_snips = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 10000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct BaseAccount {
    pub total_snips: u64,
    pub snips_list: Vec<SnipStruct>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SnipStruct {
    pub snip_text: String,
    pub user_address: Pubkey,
}
