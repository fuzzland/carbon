use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8ce9215e845ac28f")]
pub struct InitBoost {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitBoostInstructionAccounts {
    pub pool: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub pool_base_token_account: solana_pubkey::Pubkey,
    pub pool_quote_token_account: solana_pubkey::Pubkey,
    pub boost_vault_authority: solana_pubkey::Pubkey,
    pub boost_vault: solana_pubkey::Pubkey,
    pub quote_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitBoost {
    type ArrangedAccounts = InitBoostInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let pool = next_account(&mut iter)?;
        let global_config = next_account(&mut iter)?;
        let creator = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let pool_base_token_account = next_account(&mut iter)?;
        let pool_quote_token_account = next_account(&mut iter)?;
        let boost_vault_authority = next_account(&mut iter)?;
        let boost_vault = next_account(&mut iter)?;
        let quote_token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(InitBoostInstructionAccounts {
            pool,
            global_config,
            creator,
            base_mint,
            quote_mint,
            pool_base_token_account,
            pool_quote_token_account,
            boost_vault_authority,
            boost_vault,
            quote_token_program,
            system_program,
            associated_token_program,
            event_authority,
            program,
        })
    }
}
