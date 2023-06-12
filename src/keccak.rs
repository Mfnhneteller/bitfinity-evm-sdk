use rlp::Encodable;
use sha2::Digest;
use sha3::Keccak256;

use crate::hash::Hash;
use crate::H256;

/// The KECCAK of the RLP encoding of empty data, used in empty trie node, genesis block.
/// https://docs.rs/keccak-hash/latest/keccak_hash/constant.KECCAK_NULL_RLP.html
pub const KECCAK_NULL_RLP: H256 = Hash::<ethereum_types::H256>(ethereum_types::H256([
    0x56, 0xe8, 0x1f, 0x17, 0x1b, 0xcc, 0x55, 0xa6, 0xff, 0x83, 0x45, 0xe6, 0x92, 0xc0, 0xf8, 0x6e,
    0x5b, 0x48, 0xe0, 0x1b, 0x99, 0x6c, 0xad, 0xc0, 0x01, 0x62, 0x2f, 0xb5, 0xe3, 0x63, 0xb4, 0x21,
]));

/// The KECCAK of the empty byte slice
pub const KECCAK_EMPTY: H256 = Hash::<ethereum_types::H256>(ethereum_types::H256([
    0xc5, 0xd2, 0x46, 0x01, 0x86, 0xf7, 0x23, 0x3c, 0x92, 0x7e, 0x7d, 0xb2, 0xdc, 0xc7, 0x03, 0xc0,
    0xe5, 0x00, 0xb6, 0x53, 0xca, 0x82, 0x27, 0x3b, 0x7b, 0xfa, 0xd8, 0x04, 0x5d, 0x85, 0xa4, 0x70,
]));

/// The KECCAK of the RLP encoding of empty list, used in genesis block.
/// https://docs.rs/keccak-hash/latest/keccak_hash/constant.KECCAK_EMPTY_LIST_RLP.html
pub const KECCAK_EMPTY_LIST_RLP: H256 = Hash::<ethereum_types::H256>(ethereum_types::H256([
    0x1d, 0xcc, 0x4d, 0xe8, 0xde, 0xc7, 0x5d, 0x7a, 0xab, 0x85, 0xb5, 0x67, 0xb6, 0xcc, 0xd4, 0x1a,
    0xd3, 0x12, 0x45, 0x1b, 0x94, 0x8a, 0x74, 0x13, 0xf0, 0xa1, 0x42, 0xfd, 0x40, 0xd4, 0x93, 0x47,
]));

/// Calculate the Keccak hash of an encoded rlp stream
pub fn keccak_hash_rlp<E: Encodable>(data: &E) -> H256 {
    keccak_hash(&rlp::encode(data))
}

/// Calculate the Keccak hash
pub fn keccak_hash(data: &[u8]) -> H256 {
    let mut out = [0; 32];
    let mut hash = Keccak256::new();
    hash.update(data);
    out.copy_from_slice(hash.finalize().as_slice());
    out.into()
}
