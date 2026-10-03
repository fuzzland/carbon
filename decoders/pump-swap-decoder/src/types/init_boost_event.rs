use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InitBoostEvent {
    pub timestamp: i64,
    pub mint: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub virtual_quote_reserves: i128,
    pub real_quote_reserves_after: u64,
}
