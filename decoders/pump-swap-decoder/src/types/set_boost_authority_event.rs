use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SetBoostAuthorityEvent {
    pub timestamp: i64,
    pub admin: solana_pubkey::Pubkey,
    pub old_boost_authority: solana_pubkey::Pubkey,
    pub new_boost_authority: solana_pubkey::Pubkey,
}
