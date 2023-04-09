use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::BoundedVec;
use scale_info::TypeInfo;

#[derive(Encode, Decode, Debug, TypeInfo, MaxEncodedLen, PartialEq)]
pub struct NftInfo<AccountId> {
    pub minter: AccountId,
    pub metadata: BoundedVec<u8, frame_support::traits::ConstU32<128>>,
    pub transferable: bool,
}