use std::borrow::Cow;

use candid::CandidType;
use ic_stable_structures::{Bound, Storable};
use serde::{Deserialize, Serialize};

use crate::codec::ByteChunkReader;
use crate::U256;

/// Describes basic state of an EVM account.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, CandidType)]
pub struct BasicAccount {
    /// Account balance.
    pub balance: U256,
    /// Account nonce.
    pub nonce: U256,
}

/// StableDBStorage indices information
#[derive(Debug, Clone, Serialize, CandidType, Deserialize, Eq, PartialEq)]
pub struct Indices {
    /// Index of the current block
    pub pending_block: u64,
    /// Number of block to keep history
    pub history_size: u64,
}

impl Indices {
    const STORABLE_BYTE_SIZE: usize = std::mem::size_of::<u64>() * 2;
}

impl Storable for Indices {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let mut buf = Vec::with_capacity(Self::STORABLE_BYTE_SIZE);
        buf.extend_from_slice(&self.pending_block.to_be_bytes());
        buf.extend_from_slice(&self.history_size.to_be_bytes());
        buf.into()
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        let mut reader = ByteChunkReader::new(&bytes);
        let pending_block = u64::from_be_bytes(*reader.read_slice());
        let history_size = u64::from_be_bytes(*reader.read_slice());
        Self {
            pending_block,
            history_size,
        }
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: Self::STORABLE_BYTE_SIZE as _,
        is_fixed_size: true,
    };
}

/// Full information about entry
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct FullStorageValue {
    /// Data
    pub data: Vec<u8>,
    /// Number of inserts subtracted by number of removals.
    /// May be zero for the values which were removed in past before the moment they are cleaned.
    pub rc: u32,
}
