use anchor_lang::prelude::*;

declare_id!("7qx8HL4Gnh7onFDuVAdDiE3SA5TerSdP3S6G8zfvwmp3");

#[program]
pub mod solana_anchor_nft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitNFT<`info> {
    /// CHECK: <comment explaining why are we blindly trusting this account>
    #[account(mut signer)]
    signer: AccountInfo<`info>,
    #[account(
        init,
        payer: signer,
        mint::decimals = 0,
        mint::authority = signer.key(),
        mint::freeze_authority = signer.key()
    )]
    mint: Account<`info, Mint>,
}
