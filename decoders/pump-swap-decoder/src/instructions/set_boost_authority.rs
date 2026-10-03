use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe3954c2a8227eacd")]
pub struct SetBoostAuthority {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetBoostAuthorityInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub boost_authority: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetBoostAuthority {
    type ArrangedAccounts = SetBoostAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let admin = next_account(&mut iter)?;
        let global_config = next_account(&mut iter)?;
        let boost_authority = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(SetBoostAuthorityInstructionAccounts {
            admin,
            global_config,
            boost_authority,
            system_program,
            event_authority,
            program,
        })
    }
}
