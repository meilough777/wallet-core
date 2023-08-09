use crate::entry::{BitcoinEntry, PlaceHolderProto};
use bitcoin::address::{Payload, WitnessVersion};
use bitcoin::key::TweakedPublicKey;
use bitcoin::{Address, ScriptBuf, VarInt, WPubkeyHash};
use secp256k1::hashes::Hash;
use secp256k1::XOnlyPublicKey;
use std::borrow::Cow;
use std::str::FromStr;
use tw_coin_entry::coin_context::CoinContext;
use tw_coin_entry::coin_entry::{PublicKeyBytes, SignatureBytes};
use tw_proto::Bitcoin::Proto::{self as BitcoinProto, UnspentTransaction};

// Legacy implementations used for `Bitcoin.proto` backwards-compatibility.
impl BitcoinEntry {
    #[inline]
    fn compile_legacy(
        &self,
        _coin: &dyn CoinContext,
        _input: PlaceHolderProto<'static>,
        _signatures: Vec<SignatureBytes>,
        _public_keys: Vec<PublicKeyBytes>,
    ) -> PlaceHolderProto<'static> {
        todo!()
    }

    #[inline]
    fn plan_builder_legacy(&self) -> Option<LegacyPlanBuilder> {
        None
    }
}

pub struct LegacyPlanBuilder;

impl LegacyPlanBuilder {
    fn plan<'a>(
        &self,
        _coin: &dyn CoinContext,
        proto: BitcoinProto::SigningInput<'a>,
    ) -> BitcoinProto::TransactionPlan<'a> {
        let mut inputs = proto.utxo.to_vec();
        inputs.sort_by(|a, b| a.amount.partial_cmp(&b.amount).unwrap());

        let mut total_selected_amount = 0;
        let mut remaining = proto.amount as u64;
        let selected: Vec<UnspentTransaction<'_>> = inputs
            .into_iter()
            .take_while(|input| {
                remaining = remaining.saturating_sub(input.amount as u64);
                total_selected_amount += input.amount as u64;

                remaining != 0
            })
            .collect();

        if remaining != 0 {
            // Return error
            todo!()
        }

        let mut weight = 0;
        let mut witness_weight = 0;

        let scale_factor = 1;
        for input in &selected {
            weight += scale_factor
                * (
                    // Outpoint
                    32 + 4
                // Sequence
                + 4
                + VarInt(input.script.as_ref().len() as u64).len()
                + input.script.len()
                );

            if !input.spendingScript.is_empty() {
                witness_weight += 1;
                weight += input.spendingScript.len();
            }
        }

        let change = total_selected_amount - proto.amount as u64;

        BitcoinProto::TransactionPlan {
            amount: proto.amount as i64,
            available_amount: total_selected_amount as i64,
            fee: 0,
            change: change as i64,
            utxos: selected,
            // Used for Zcash
            branch_id: Cow::default(),
            error: tw_proto::Common::Proto::SigningError::OK,
            // Used for other chain(s).
            output_op_return: Cow::default(),
            // Used for other chain(s).
            preblockhash: Cow::default(),
            // Used for other chain(s).
            preblockheight: 0,
        }
    }
}

fn convert_address_to_script_pubkey(address: &str) -> ScriptBuf {
    let address = address
        .parse::<Address<_>>()
        .unwrap()
        .require_network(bitcoin::Network::Bitcoin)
        .unwrap();

    let script_pubkey = match address.address_type().unwrap() {
        bitcoin::AddressType::P2pkh => {
            if let Payload::PubkeyHash(ref hash) = address.payload {
                ScriptBuf::new_p2pkh(hash)
            } else {
                panic!()
            }
        },
        bitcoin::AddressType::P2sh => {
            if let Payload::ScriptHash(ref hash) = address.payload {
                ScriptBuf::new_p2sh(hash)
            } else {
                panic!()
            }
        },
        bitcoin::AddressType::P2wpkh => {
            if let Payload::WitnessProgram(program) = address.payload {
                // TODO
                assert_eq!(program.version(), WitnessVersion::V0);
                let slice = program.program().as_bytes();
                let wpubkey_hash = WPubkeyHash::from_slice(slice).unwrap();
                ScriptBuf::new_v0_p2wpkh(&wpubkey_hash)
            } else {
                panic!()
            }
        },
        bitcoin::AddressType::P2wsh => todo!(),
        bitcoin::AddressType::P2tr => {
            if let Payload::WitnessProgram(program) = address.payload {
                // TODO
                assert_eq!(program.version(), WitnessVersion::V1);
                let slice = program.program().as_bytes();
                let xonly = XOnlyPublicKey::from_slice(slice).unwrap();
                // TODO: another way to do this?
                let output_key = TweakedPublicKey::dangerous_assume_tweaked(xonly);
                ScriptBuf::new_v1_p2tr_tweaked(output_key)
            } else {
                panic!()
            }
        },
        _ => todo!(),
    };

    script_pubkey
}