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

    pub fn add_snip(ctx: Context<AddSnips>, code: String, lang_string: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;

        let lang_enum = match &lang_string[..] {
            "Rust" => Language::Rust,
            "Go" => Language::Go,
            "Text" => Language::Text,
            _ => Language::Unknown,
        };

        // Build the struct.
        let item = SnipStruct {
            code: code.to_string(),
            lang: lang_enum,
            user_address: *base_account.to_account_info().key,
        };

        // Add it to the gif_list vector.
        base_account.snips_list.push(item);
        base_account.total_snips += 1;
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

#[derive(Accounts)]
pub struct AddSnips<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[account]
pub struct BaseAccount {
    pub total_snips: u64,
    pub snips_list: Vec<SnipStruct>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SnipStruct {
    pub code: String,
    pub lang: Language,
    pub user_address: Pubkey,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub enum Language {
    Rust,
    Go,
    Text,
    Unknown,
}
