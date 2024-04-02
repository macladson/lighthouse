use crate::{test_utils::TestRandom, *};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use ssz::Decode;
use ssz_derive::{Decode, Encode};
use test_random_derive::TestRandom;
use tree_hash::TreeHash;
use tree_hash_derive::TreeHash;

#[superstruct(
    variants(Merge, Capella, Deneb),
    variant_attributes(
        derive(
            Default,
            Debug,
            Clone,
            Serialize,
            Deserialize,
            Encode,
            Decode,
            TreeHash,
            TestRandom,
            Derivative,
            arbitrary::Arbitrary
        ),
        derivative(PartialEq, Hash(bound = "E: EthSpec")),
        serde(bound = "E: EthSpec", deny_unknown_fields),
        arbitrary(bound = "E: EthSpec")
    ),
    ref_attributes(
        derive(PartialEq, TreeHash, Debug),
        tree_hash(enum_behaviour = "transparent")
    ),
    cast_error(ty = "Error", expr = "BeaconStateError::IncorrectStateVariant"),
    partial_getter_error(ty = "Error", expr = "BeaconStateError::IncorrectStateVariant")
)]
#[derive(
    Debug, Clone, Serialize, Deserialize, Encode, TreeHash, Derivative, arbitrary::Arbitrary,
)]
#[derivative(PartialEq, Hash(bound = "E: EthSpec"))]
#[serde(bound = "E: EthSpec", untagged)]
#[arbitrary(bound = "E: EthSpec")]
#[tree_hash(enum_behaviour = "transparent")]
#[ssz(enum_behaviour = "transparent")]
pub struct ExecutionPayloadHeader<E: EthSpec> {
    #[superstruct(getter(copy))]
    pub parent_hash: ExecutionBlockHash,
    #[superstruct(getter(copy))]
    pub fee_recipient: Address,
    #[superstruct(getter(copy))]
    pub state_root: Hash256,
    #[superstruct(getter(copy))]
    pub receipts_root: Hash256,
    #[serde(with = "ssz_types::serde_utils::hex_fixed_vec")]
    pub logs_bloom: FixedVector<u8, E::BytesPerLogsBloom>,
    #[superstruct(getter(copy))]
    pub prev_randao: Hash256,
    #[serde(with = "serde_utils::quoted_u64")]
    #[superstruct(getter(copy))]
    pub block_number: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    #[superstruct(getter(copy))]
    pub gas_limit: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    #[superstruct(getter(copy))]
    pub gas_used: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    #[superstruct(getter(copy))]
    pub timestamp: u64,
    #[serde(with = "ssz_types::serde_utils::hex_var_list")]
    pub extra_data: VariableList<u8, E::MaxExtraDataBytes>,
    #[serde(with = "serde_utils::quoted_u256")]
    #[superstruct(getter(copy))]
    pub base_fee_per_gas: Uint256,
    #[superstruct(getter(copy))]
    pub block_hash: ExecutionBlockHash,
    #[superstruct(getter(copy))]
    pub transactions_root: Hash256,
    #[superstruct(only(Capella, Deneb))]
    #[superstruct(getter(copy))]
    pub withdrawals_root: Hash256,
    #[superstruct(only(Deneb))]
    #[serde(with = "serde_utils::quoted_u64")]
    #[superstruct(getter(copy))]
    pub blob_gas_used: u64,
    #[superstruct(only(Deneb))]
    #[serde(with = "serde_utils::quoted_u64")]
    #[superstruct(getter(copy))]
    pub excess_blob_gas: u64,
}

impl<E: EthSpec> ExecutionPayloadHeader<E> {
    pub fn transactions(&self) -> Option<&Transactions<E>> {
        None
    }

    pub fn from_ssz_bytes(bytes: &[u8], fork_name: ForkName) -> Result<Self, ssz::DecodeError> {
        match fork_name {
            ForkName::Base | ForkName::Altair => Err(ssz::DecodeError::BytesInvalid(format!(
                "unsupported fork for ExecutionPayloadHeader: {fork_name}",
            ))),
            ForkName::Merge => ExecutionPayloadHeaderMerge::from_ssz_bytes(bytes).map(Self::Merge),
            ForkName::Capella => {
                ExecutionPayloadHeaderCapella::from_ssz_bytes(bytes).map(Self::Capella)
            }
            ForkName::Deneb => ExecutionPayloadHeaderDeneb::from_ssz_bytes(bytes).map(Self::Deneb),
        }
    }
}

impl<'a, E: EthSpec> ExecutionPayloadHeaderRef<'a, E> {
    pub fn is_default_with_zero_roots(self) -> bool {
        map_execution_payload_header_ref!(&'a _, self, |inner, cons| {
            cons(inner);
            *inner == Default::default()
        })
    }
}

impl<E: EthSpec> ExecutionPayloadHeaderMerge<E> {
    pub fn upgrade_to_capella(&self) -> ExecutionPayloadHeaderCapella<E> {
        ExecutionPayloadHeaderCapella {
            parent_hash: self.parent_hash,
            fee_recipient: self.fee_recipient,
            state_root: self.state_root,
            receipts_root: self.receipts_root,
            logs_bloom: self.logs_bloom.clone(),
            prev_randao: self.prev_randao,
            block_number: self.block_number,
            gas_limit: self.gas_limit,
            gas_used: self.gas_used,
            timestamp: self.timestamp,
            extra_data: self.extra_data.clone(),
            base_fee_per_gas: self.base_fee_per_gas,
            block_hash: self.block_hash,
            transactions_root: self.transactions_root,
            withdrawals_root: Hash256::zero(),
        }
    }
}

impl<E: EthSpec> ExecutionPayloadHeaderCapella<E> {
    pub fn upgrade_to_deneb(&self) -> ExecutionPayloadHeaderDeneb<E> {
        ExecutionPayloadHeaderDeneb {
            parent_hash: self.parent_hash,
            fee_recipient: self.fee_recipient,
            state_root: self.state_root,
            receipts_root: self.receipts_root,
            logs_bloom: self.logs_bloom.clone(),
            prev_randao: self.prev_randao,
            block_number: self.block_number,
            gas_limit: self.gas_limit,
            gas_used: self.gas_used,
            timestamp: self.timestamp,
            extra_data: self.extra_data.clone(),
            base_fee_per_gas: self.base_fee_per_gas,
            block_hash: self.block_hash,
            transactions_root: self.transactions_root,
            withdrawals_root: self.withdrawals_root,
            blob_gas_used: 0,
            excess_blob_gas: 0,
        }
    }
}

impl<'a, E: EthSpec> From<&'a ExecutionPayloadMerge<E>> for ExecutionPayloadHeaderMerge<E> {
    fn from(payload: &'a ExecutionPayloadMerge<E>) -> Self {
        Self {
            parent_hash: payload.parent_hash,
            fee_recipient: payload.fee_recipient,
            state_root: payload.state_root,
            receipts_root: payload.receipts_root,
            logs_bloom: payload.logs_bloom.clone(),
            prev_randao: payload.prev_randao,
            block_number: payload.block_number,
            gas_limit: payload.gas_limit,
            gas_used: payload.gas_used,
            timestamp: payload.timestamp,
            extra_data: payload.extra_data.clone(),
            base_fee_per_gas: payload.base_fee_per_gas,
            block_hash: payload.block_hash,
            transactions_root: payload.transactions.tree_hash_root(),
        }
    }
}
impl<'a, E: EthSpec> From<&'a ExecutionPayloadCapella<E>> for ExecutionPayloadHeaderCapella<E> {
    fn from(payload: &'a ExecutionPayloadCapella<E>) -> Self {
        Self {
            parent_hash: payload.parent_hash,
            fee_recipient: payload.fee_recipient,
            state_root: payload.state_root,
            receipts_root: payload.receipts_root,
            logs_bloom: payload.logs_bloom.clone(),
            prev_randao: payload.prev_randao,
            block_number: payload.block_number,
            gas_limit: payload.gas_limit,
            gas_used: payload.gas_used,
            timestamp: payload.timestamp,
            extra_data: payload.extra_data.clone(),
            base_fee_per_gas: payload.base_fee_per_gas,
            block_hash: payload.block_hash,
            transactions_root: payload.transactions.tree_hash_root(),
            withdrawals_root: payload.withdrawals.tree_hash_root(),
        }
    }
}

impl<'a, E: EthSpec> From<&'a ExecutionPayloadDeneb<E>> for ExecutionPayloadHeaderDeneb<E> {
    fn from(payload: &'a ExecutionPayloadDeneb<E>) -> Self {
        Self {
            parent_hash: payload.parent_hash,
            fee_recipient: payload.fee_recipient,
            state_root: payload.state_root,
            receipts_root: payload.receipts_root,
            logs_bloom: payload.logs_bloom.clone(),
            prev_randao: payload.prev_randao,
            block_number: payload.block_number,
            gas_limit: payload.gas_limit,
            gas_used: payload.gas_used,
            timestamp: payload.timestamp,
            extra_data: payload.extra_data.clone(),
            base_fee_per_gas: payload.base_fee_per_gas,
            block_hash: payload.block_hash,
            transactions_root: payload.transactions.tree_hash_root(),
            withdrawals_root: payload.withdrawals.tree_hash_root(),
            blob_gas_used: payload.blob_gas_used,
            excess_blob_gas: payload.excess_blob_gas,
        }
    }
}

// These impls are required to work around an inelegance in `to_execution_payload_header`.
// They only clone headers so they should be relatively cheap.
impl<'a, E: EthSpec> From<&'a Self> for ExecutionPayloadHeaderMerge<E> {
    fn from(payload: &'a Self) -> Self {
        payload.clone()
    }
}

impl<'a, E: EthSpec> From<&'a Self> for ExecutionPayloadHeaderCapella<E> {
    fn from(payload: &'a Self) -> Self {
        payload.clone()
    }
}

impl<'a, E: EthSpec> From<&'a Self> for ExecutionPayloadHeaderDeneb<E> {
    fn from(payload: &'a Self) -> Self {
        payload.clone()
    }
}

impl<'a, E: EthSpec> From<ExecutionPayloadRef<'a, E>> for ExecutionPayloadHeader<E> {
    fn from(payload: ExecutionPayloadRef<'a, E>) -> Self {
        map_execution_payload_ref_into_execution_payload_header!(
            &'a _,
            payload,
            |inner, cons| cons(inner.into())
        )
    }
}

impl<E: EthSpec> TryFrom<ExecutionPayloadHeader<E>> for ExecutionPayloadHeaderMerge<E> {
    type Error = BeaconStateError;
    fn try_from(header: ExecutionPayloadHeader<E>) -> Result<Self, Self::Error> {
        match header {
            ExecutionPayloadHeader::Merge(execution_payload_header) => Ok(execution_payload_header),
            _ => Err(BeaconStateError::IncorrectStateVariant),
        }
    }
}
impl<E: EthSpec> TryFrom<ExecutionPayloadHeader<E>> for ExecutionPayloadHeaderCapella<E> {
    type Error = BeaconStateError;
    fn try_from(header: ExecutionPayloadHeader<E>) -> Result<Self, Self::Error> {
        match header {
            ExecutionPayloadHeader::Capella(execution_payload_header) => {
                Ok(execution_payload_header)
            }
            _ => Err(BeaconStateError::IncorrectStateVariant),
        }
    }
}
impl<E: EthSpec> TryFrom<ExecutionPayloadHeader<E>> for ExecutionPayloadHeaderDeneb<E> {
    type Error = BeaconStateError;
    fn try_from(header: ExecutionPayloadHeader<E>) -> Result<Self, Self::Error> {
        match header {
            ExecutionPayloadHeader::Deneb(execution_payload_header) => Ok(execution_payload_header),
            _ => Err(BeaconStateError::IncorrectStateVariant),
        }
    }
}

impl<E: EthSpec> ForkVersionDeserialize for ExecutionPayloadHeader<E> {
    fn deserialize_by_fork<'de, D: serde::Deserializer<'de>>(
        value: serde_json::value::Value,
        fork_name: ForkName,
    ) -> Result<Self, D::Error> {
        let convert_err = |e| {
            serde::de::Error::custom(format!(
                "ExecutionPayloadHeader failed to deserialize: {:?}",
                e
            ))
        };

        Ok(match fork_name {
            ForkName::Merge => Self::Merge(serde_json::from_value(value).map_err(convert_err)?),
            ForkName::Capella => Self::Capella(serde_json::from_value(value).map_err(convert_err)?),
            ForkName::Deneb => Self::Deneb(serde_json::from_value(value).map_err(convert_err)?),
            ForkName::Base | ForkName::Altair => {
                return Err(serde::de::Error::custom(format!(
                    "ExecutionPayloadHeader failed to deserialize: unsupported fork '{}'",
                    fork_name
                )));
            }
        })
    }
}
