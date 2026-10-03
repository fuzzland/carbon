use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d5980f08d5bca4769")]
pub struct SetBoostAuthorityEvent {
    pub timestamp: i64,
    pub admin: solana_pubkey::Pubkey,
    pub old_boost_authority: solana_pubkey::Pubkey,
    pub new_boost_authority: solana_pubkey::Pubkey,
}
