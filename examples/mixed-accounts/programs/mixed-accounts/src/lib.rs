use anchor_lang::prelude::*;
use light_sdk::{compressed_account::LightAccount, merkle_context::PackedAddressMerkleContext, light_account, light_accounts, light_program};

declare_id!("7yucc7fL3JGbyMwg4neUaenNSdySS39hbAk89Ao3t1Hz");

#[light_program]
#[program]
pub mod mixed_accounts {
    use super::*;

    pub fn with_compressed_account<'info>(
        ctx: LightContext<'_, '_, '_, 'info, WithCompressedAccount<'info>>,
        name: String,
    ) -> Result<()> {
        Ok(())
    }

    pub fn without_compressed_account<'info>(
        ctx: Context<'_, '_, '_, 'info, WithoutCompressedAccount<'info>>,
        name: String,
    ) -> Result<()> {
        Ok(())
    }
}

#[light_account]
#[derive(Clone, Debug, Default)]
pub struct MyCompressedAccount {
    foo: u64,
}

#[account]
pub struct MyRegularAccount {
    foo: u64,
}

#[light_accounts]
#[instruction(foo: u64)]
pub struct WithCompressedAccount<'info> {
    #[account(mut)]
    #[fee_payer]
    pub signer: Signer<'info>,
    #[self_program]
    pub self_program: Program<'info, crate::program::MixedAccounts>,
    /// CHECK: Checked in light-system-program.
    #[authority]
    pub cpi_signed: AccountInfo<'info>,

    #[light_account(
        init,
        seeds = [b"compressed", &foo.to_le_bytes()],
    )]
    pub my_compressed_account: LightAccount<MyCompressedAccount>,
}

#[derive(Accounts)]
#[instruction(foo: u64)]
pub struct WithoutCompressedAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        seeds = [b"compressed", &foo.to_le_bytes()],
        bump,
        payer = signer,
        space = 8 + 8,
    )]
    pub my_regular_account: Account<'info, MyRegularAccount>,
    pub system_program: Program<'info, System>,
}
