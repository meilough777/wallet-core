use crate::{tweak_pubkey, InputContext, Recipient, TaprootScript};
use bitcoin::key::{PublicKey, TweakedPublicKey as BTweakedPublicKey};
use bitcoin::script::ScriptBuf;
use bitcoin::taproot::TapNodeHash;
use bitcoin::{OutPoint, PubkeyHash, Sequence, TxIn, Txid, Witness};

#[derive(Debug, Clone)]
pub enum TxInput {
    P2PKH(TxInputP2PKH),
    P2TRKeyPath(TxInputP2TRKeyPath),
    NonStandard { ctx: InputContext },
}

impl From<TxInputP2PKH> for TxInput {
    fn from(input: TxInputP2PKH) -> Self {
        TxInput::P2PKH(input)
    }
}

impl From<TxInputP2TRKeyPath> for TxInput {
    fn from(input: TxInputP2TRKeyPath) -> Self {
        TxInput::P2TRKeyPath(input)
    }
}

impl From<TxInput> for TxIn {
    fn from(input: TxInput) -> Self {
        let ctx = input.ctx();

        TxIn {
            previous_output: ctx.previous_output,
            script_sig: ScriptBuf::default(),
            sequence: ctx.sequence,
            witness: ctx.witness.clone(),
        }
    }
}

impl TxInput {
    pub fn ctx(&self) -> &InputContext {
        match self {
            TxInput::P2PKH(t) => &t.ctx,
            TxInput::P2TRKeyPath(t) => &t.ctx,
            TxInput::NonStandard { ctx } => ctx,
        }
    }
    pub fn satoshis(&self) -> Option<u64> {
        match self {
            TxInput::P2PKH(p) => p.ctx.value,
            TxInput::P2TRKeyPath(p) => p.ctx.value,
            TxInput::NonStandard { ctx } => ctx.value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TxInputP2PKH {
    pub(crate) ctx: InputContext,
    pub(crate) recipient: Recipient<PubkeyHash>,
}

impl TxInputP2PKH {
    // TODO: `satoshis` should be mandatory.
    pub fn new(
        txid: Txid,
        vout: u32,
        recipient: impl Into<Recipient<PubkeyHash>>,
        satoshis: Option<u64>,
    ) -> Self {
        let recipient: Recipient<PubkeyHash> = recipient.into();

        TxInputP2PKH {
            ctx: InputContext {
                previous_output: OutPoint { txid, vout },
                value: satoshis,
                script_pubkey: ScriptBuf::new_p2pkh(&recipient.pubkey_hash()),
                sequence: Sequence::default(),
                witness: Witness::default(),
            },
            recipient,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TxInputP2TRKeyPath {
    pub(crate) ctx: InputContext,
    pub(crate) recipient: Recipient<PublicKey>,
}

impl TxInputP2TRKeyPath {
    pub fn new(
        txid: Txid,
        vout: u32,
        recipient: impl Into<Recipient<PublicKey>>,
        satoshis: u64,
    ) -> Self {
        let recipient: Recipient<PublicKey> = recipient.into();

        TxInputP2TRKeyPath {
            ctx: InputContext {
                previous_output: OutPoint { txid, vout },
                value: Some(satoshis),
                // TODO: Use `new_v1_p2tr` instead?
                script_pubkey: ScriptBuf::new_v1_p2tr_tweaked(recipient.tweaked_pubkey()),
                sequence: Sequence::default(),
                witness: Witness::default(),
            },
            recipient,
        }
    }
}

pub struct TxInputP2TRScriptPath {
    pub(crate) ctx: InputContext,
    pub(crate) recipient: Recipient<TaprootScript>,
}

impl TxInputP2TRScriptPath {
    pub fn new(
        txid: Txid,
        vout: u32,
        recipient: impl Into<Recipient<TaprootScript>>,
        satoshis: u64,
    ) -> Self {
        let recipient: Recipient<TaprootScript> = recipient.into();

        TxInputP2TRScriptPath {
            ctx: InputContext {
                previous_output: OutPoint { txid, vout },
                value: Some(satoshis),
                script_pubkey: ScriptBuf::new_v1_p2tr(
                    &secp256k1::Secp256k1::new(),
                    recipient.untweaked_pubkey(),
                    Some(recipient.merkle_root()),
                ),
                sequence: Sequence::default(),
                witness: Witness::default(),
            },
            recipient,
        }
    }
}
