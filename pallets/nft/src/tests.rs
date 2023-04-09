use crate::{mock::*, Error, Event, types::NftInfo};
use frame_support::{assert_ok, BoundedVec, assert_noop};
use sp_core::ConstU32;

#[test]
fn basic_mint() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
        let metadata: BoundedVec<u8, ConstU32<128>> = vec![0, 1, 2].try_into().unwrap();
        let nft = NftInfo {
            minter: ALICE,
            metadata: metadata.clone(),
            transferable: true,
        };
        assert_ok!(NftModule::create_collection(RuntimeOrigin::signed(ALICE), 2));
		assert_ok!(NftModule::mint(RuntimeOrigin::signed(ALICE), COLLECTION_ID, 3, metadata, ALICE, true));
		assert_eq!(NftModule::nfts(COLLECTION_ID, 3), Some(nft));
		System::assert_last_event(Event::NftMinted {
            collection_id: COLLECTION_ID, nft_id: 3, minter: ALICE }.into());
	});
}

#[test]
fn basic_transfer() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
        let metadata: BoundedVec<u8, ConstU32<128>> = vec![0, 1, 2].try_into().unwrap();

        assert_ok!(NftModule::create_collection(RuntimeOrigin::signed(ALICE), 2));
		assert_ok!(NftModule::mint(RuntimeOrigin::signed(ALICE), COLLECTION_ID, 3, metadata.clone(), ALICE, true));
        assert_ok!(NftModule::mint(RuntimeOrigin::signed(BOB), COLLECTION_ID, 4, metadata, BOB, true));

        assert_ok!(NftModule::transfer(RuntimeOrigin::signed(ALICE), COLLECTION_ID, 3, BOB));
        // Transfer again, should return error
        assert_noop!(
            NftModule::transfer(RuntimeOrigin::signed(ALICE), COLLECTION_ID, 3, BOB),
            Error::<Test>::SenderNotOwner
        );

        assert_ok!(NftModule::transfer(RuntimeOrigin::signed(BOB), COLLECTION_ID, 3, ALICE));
	});
}
