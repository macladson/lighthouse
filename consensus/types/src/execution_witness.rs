use crate::{test_utils::TestRandom, *};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use test_random_derive::TestRandom;
use tree_hash_derive::TreeHash;

#[derive(
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
    arbitrary::Arbitrary,
)]
#[derivative(PartialEq, Hash(bound = "E: EthSpec"))]
#[arbitrary(bound = "E: EthSpec")]
#[serde(bound = "E: EthSpec")]
#[ssz(struct_behaviour = "transparent")]
#[serde(transparent)]
pub struct BanderwagonGroupElement<E: EthSpec> {
    #[serde(with = "ssz_types::serde_utils::hex_fixed_vec")]
    inner: FixedVector<u8, E::BytesPerBanderwagonElement>,
}

#[derive(
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
    arbitrary::Arbitrary,
)]
#[derivative(PartialEq, Hash(bound = "E: EthSpec"))]
#[arbitrary(bound = "E: EthSpec")]
#[serde(bound = "E: EthSpec")]
#[ssz(struct_behaviour = "transparent")]
#[serde(transparent)]
pub struct BanderwagonFieldElement<E: EthSpec> {
    #[serde(with = "ssz_types::serde_utils::hex_fixed_vec")]
    inner: FixedVector<u8, E::BytesPerBanderwagonElement>,
}

#[derive(
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
    arbitrary::Arbitrary,
)]
#[derivative(PartialEq, Hash(bound = "E: EthSpec"))]
#[arbitrary(bound = "E: EthSpec")]
#[serde(bound = "E: EthSpec")]
#[ssz(struct_behaviour = "transparent")]
#[serde(transparent)]
pub struct Stem<E: EthSpec> {
    #[serde(with = "ssz_types::serde_utils::hex_fixed_vec")]
    inner: FixedVector<u8, E::MaxStemLength>,
}

#[derive(
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
    arbitrary::Arbitrary,
)]
#[derivative(PartialEq, Hash(bound = "E: EthSpec"))]
#[arbitrary(bound = "E: EthSpec")]
#[serde(bound = "E: EthSpec")]
#[ssz(struct_behaviour = "transparent")]
#[serde(transparent)]
pub struct StateDiffValue<E: EthSpec> {
    #[serde(with = "ssz_types::serde_utils::hex_fixed_vec")]
    inner: FixedVector<u8, E::BytesPerSuffixStateDiffValue>,
}

#[derive(
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
    arbitrary::Arbitrary,
)]
#[derivative(PartialEq, Hash(bound = "E: EthSpec"))]
#[arbitrary(bound = "E: EthSpec")]
#[serde(bound = "E: EthSpec")]
pub struct SuffixStateDiff<E: EthSpec> {
    //#[serde(with = "eth2_serde_utils::quoted_u8")]
    pub suffix: u8,
    // `None` means not currently present.
    pub current_value: Optional<StateDiffValue<E>>,
    // `None` means value is not updated.
    pub new_value: Optional<StateDiffValue<E>>,
}

#[derive(
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
    arbitrary::Arbitrary,
)]
#[derivative(PartialEq, Hash(bound = "E: EthSpec"))]
#[arbitrary(bound = "E: EthSpec")]
#[serde(bound = "E: EthSpec")]
pub struct StemStateDiff<E: EthSpec> {
    pub stem: Stem<E>,
    pub suffix_diffs: VariableList<SuffixStateDiff<E>, E::MaxVerkleWidth>,
}

#[derive(
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
    arbitrary::Arbitrary,
)]
#[derivative(PartialEq, Hash(bound = "E: EthSpec"))]
#[arbitrary(bound = "E: EthSpec")]
#[serde(bound = "E: EthSpec")]
#[ssz(struct_behaviour = "transparent")]
#[serde(transparent)]
pub struct StateDiff<E: EthSpec> {
    pub inner: VariableList<StemStateDiff<E>, E::MaxStems>,
}

#[derive(
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
    arbitrary::Arbitrary,
)]
#[derivative(PartialEq, Hash(bound = "E: EthSpec"))]
#[arbitrary(bound = "E: EthSpec")]
#[serde(bound = "E: EthSpec")]
pub struct IpaProof<E: EthSpec> {
    pub cl: FixedVector<BanderwagonGroupElement<E>, E::IpaProofDepth>,
    pub cr: FixedVector<BanderwagonGroupElement<E>, E::IpaProofDepth>,
    pub final_evaluation: BanderwagonFieldElement<E>,
}

#[derive(
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
    arbitrary::Arbitrary,
)]
#[derivative(PartialEq, Hash(bound = "E: EthSpec"))]
#[arbitrary(bound = "E: EthSpec")]
#[serde(bound = "E: EthSpec")]
#[ssz(struct_behaviour = "transparent")]
#[serde(transparent)]
pub struct StemValue<E: EthSpec> {
    #[serde(with = "ssz_types::serde_utils::hex_fixed_vec")]
    inner: FixedVector<u8, E::MaxStemLength>,
}

#[derive(
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
    arbitrary::Arbitrary,
)]
#[derivative(PartialEq, Hash(bound = "E: EthSpec"))]
#[arbitrary(bound = "E: EthSpec")]
#[serde(bound = "E: EthSpec")]
pub struct VerkleProof<E: EthSpec> {
    pub other_stems: VariableList<StemValue<E>, E::MaxStems>,
    #[serde(with = "ssz_types::serde_utils::hex_var_list")]
    pub depth_extension_present: VariableList<u8, E::MaxStems>,
    pub commitments_by_path: VariableList<BanderwagonGroupElement<E>, E::MaxCommittments>,
    pub d: BanderwagonGroupElement<E>,
    pub ipa_proof: IpaProof<E>,
}

#[derive(
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
    arbitrary::Arbitrary,
)]
#[derivative(PartialEq, Hash(bound = "E: EthSpec"))]
#[arbitrary(bound = "E: EthSpec")]
#[serde(bound = "E: EthSpec")]
pub struct ExecutionWitness<E: EthSpec> {
    pub state_diff: StateDiff<E>,
    pub verkle_proof: VerkleProof<E>,
}
