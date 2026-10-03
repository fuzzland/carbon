use solana_pubkey::Pubkey;

pub struct PumpSwapDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA");

#[cfg(test)]
mod tests {

    use crate::instructions::{
        boost_buy_and_burn::BoostBuyAndBurn, boost_buy_and_burn_event::BoostBuyAndBurnEvent,
        PumpSwapInstruction,
    };

    use super::{PumpSwapDecoder, PROGRAM_ID};
    use carbon_core::{
        borsh::BorshSerialize,
        deserialize::{ArrangeAccounts, CarbonDeserialize},
        instruction::InstructionDecoder,
    };
    use solana_instruction::{AccountMeta, Instruction};
    use solana_pubkey::Pubkey;

    fn hex_to_bytes(hex: &str) -> Vec<u8> {
        (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).expect("valid hex"))
            .collect()
    }

    fn test_accounts(n: u8) -> Vec<AccountMeta> {
        (1..=n)
            .map(|i| AccountMeta::new_readonly(Pubkey::new_from_array([i; 32]), false))
            .collect()
    }

    #[test]
    fn test_boost_buy_and_burn_deserialization() {
        let decoder = PumpSwapDecoder;
        // Real mainnet boost_buy_and_burn instruction data.
        let data = hex_to_bytes("694406af000723a22ef69e270000000068c6598fe1050000");
        let accounts = test_accounts(13);
        let ix = Instruction {
            program_id: PROGRAM_ID,
            accounts: accounts.clone(),
            data,
        };

        let decoded = decoder
            .decode_instruction(&ix)
            .expect("Invalid instruction");
        match decoded.data {
            PumpSwapInstruction::BoostBuyAndBurn(boost) => {
                assert_eq!(boost.quote_amount_in, 664_729_134);
                assert_eq!(boost.min_base_amount_burned, 0x05e1_8f59_c668);
            }
            other => {
                panic!("Expected BoostBuyAndBurn, got {:?}", other);
            }
        }

        let arranged =
            BoostBuyAndBurn::arrange_accounts(&decoded.accounts).expect("arrange accounts");
        assert_eq!(arranged.pool, accounts[0].pubkey);
        assert_eq!(arranged.authority, accounts[1].pubkey);
        assert_eq!(arranged.boost_vault, accounts[8].pubkey);
        assert_eq!(arranged.program, accounts[12].pubkey);
        assert!(BoostBuyAndBurn::arrange_accounts(&accounts[..12]).is_none());
    }

    #[test]
    fn test_boost_buy_and_burn_event_round_trip() {
        let event = BoostBuyAndBurnEvent {
            timestamp: 1_759_000_000,
            mint: Pubkey::new_from_array([1; 32]),
            bonding_curve: Pubkey::new_from_array([2; 32]),
            pool: Pubkey::new_from_array([3; 32]),
            authority: Pubkey::new_from_array([4; 32]),
            quote_amount_in_requested: 664_729_134,
            quote_amount_in_used: 664_000_000,
            base_amount_burned: 6_466_330_805_864,
            virtual_quote_reserves: -123_456_789_012_345_678_901_234,
            real_quote_reserves_after: 85_000_000_000,
            base_reserves_after: 200_000_000_000_000,
            boost_vault_remaining: 1_000_000,
        };

        let mut data = BoostBuyAndBurnEvent::DISCRIMINATOR.to_vec();
        assert_eq!(
            data,
            hex_to_bytes("e445a52e51cb9a1d3f451c16305cc2b9"),
            "self-CPI event discriminator"
        );
        event.timestamp.serialize(&mut data).unwrap();
        event.mint.to_bytes().serialize(&mut data).unwrap();
        event.bonding_curve.to_bytes().serialize(&mut data).unwrap();
        event.pool.to_bytes().serialize(&mut data).unwrap();
        event.authority.to_bytes().serialize(&mut data).unwrap();
        event
            .quote_amount_in_requested
            .serialize(&mut data)
            .unwrap();
        event.quote_amount_in_used.serialize(&mut data).unwrap();
        event.base_amount_burned.serialize(&mut data).unwrap();
        event.virtual_quote_reserves.serialize(&mut data).unwrap();
        event
            .real_quote_reserves_after
            .serialize(&mut data)
            .unwrap();
        event.base_reserves_after.serialize(&mut data).unwrap();
        event.boost_vault_remaining.serialize(&mut data).unwrap();
        assert_eq!(data.len(), 16 + 8 + 4 * 32 + 3 * 8 + 16 + 3 * 8);

        assert_eq!(
            BoostBuyAndBurnEvent::deserialize(&data).as_ref(),
            Some(&event)
        );

        let ix = Instruction {
            program_id: PROGRAM_ID,
            accounts: test_accounts(1),
            data,
        };
        let decoded = PumpSwapDecoder
            .decode_instruction(&ix)
            .expect("Invalid instruction");
        match decoded.data {
            PumpSwapInstruction::BoostBuyAndBurnEvent(decoded_event) => {
                assert_eq!(decoded_event, event);
            }
            other => {
                panic!("Expected BoostBuyAndBurnEvent, got {:?}", other);
            }
        }
    }

    #[test]
    fn test_buy_deserialization_without_track_volume() {
        let decoder = PumpSwapDecoder;
        let ix =
            carbon_test_utils::read_instruction("tests/fixtures/buy_with_track_volume_false.json")
                .expect("read fixture");

        let maybe_decoded = decoder.decode_instruction(&ix);
        let decoded = maybe_decoded.expect("Invalid instruction");
        match decoded.data {
            PumpSwapInstruction::Buy(buy) => {
                assert!(!buy.track_volume.0);
            }
            other => {
                panic!("Expected Buy, got {:?}", other);
            }
        }
    }

    #[test]
    fn test_buy_deserialization_with_track_volume() {
        let decoder = PumpSwapDecoder;
        let ix =
            carbon_test_utils::read_instruction("tests/fixtures/buy_with_track_volume_true.json")
                .expect("read fixture");

        let decoded = decoder
            .decode_instruction(&ix)
            .expect("Invalid instruction");
        match decoded.data {
            PumpSwapInstruction::Buy(buy) => {
                assert!(buy.track_volume.0);
            }
            other => {
                panic!("Expected Buy, got {:?}", other);
            }
        }
    }
}
