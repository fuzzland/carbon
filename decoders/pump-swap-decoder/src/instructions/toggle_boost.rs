use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x75a1a04adf897663")]
pub struct ToggleBoost {
    pub enabled: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ToggleBoostInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ToggleBoost {
    type ArrangedAccounts = ToggleBoostInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let admin = next_account(&mut iter)?;
        let global_config = next_account(&mut iter)?;

        Some(ToggleBoostInstructionAccounts {
            admin,
            global_config,
        })
    }
}
