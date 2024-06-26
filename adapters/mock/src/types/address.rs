use rollup_interface::state::BasicAddress;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    Eq,
    PartialEq,
    Hash,
    Serialize,
    Deserialize,
    borsh::BorshDeserialize,
    borsh::BorshSerialize,
)]
pub struct MockAddress([u8; 32]);

impl BasicAddress for MockAddress {}
