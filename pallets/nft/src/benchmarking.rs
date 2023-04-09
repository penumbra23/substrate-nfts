#![cfg(feature = "runtime-benchmarks")]

use super::*;

#[allow(unused)]
use crate::Pallet as NftBench;

use crate::Event;
use frame_benchmarking::v1::{benchmarks, whitelisted_caller, vec, account};
use frame_system::RawOrigin;
use frame_support::traits::Currency;
use frame_support::sp_runtime::traits::Bounded;

pub type AccountBalance<T> = <<T as pallet_uniques::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

benchmarks! {
	create_collection {
        let caller: T::AccountId = whitelisted_caller();
        let collection_id: T::CollectionId = T::Helper::to_collection(2);
        <T as pallet_uniques::Config>::Currency::make_free_balance_be(&caller, AccountBalance::<T>::max_value());
	}: _(RawOrigin::Signed(caller), collection_id)
	verify {
		assert_last_event::<T>(Event::CollectionCreated { collection_id: collection_id }.into());
	}

	mint {
        let minter: T::AccountId = whitelisted_caller();
        let collection_id: T::CollectionId = T::Helper::to_collection(2);
		let nft_id: T::ItemId = T::Helper::to_nft(10);
		let metadata = vec![0, 1, 2].try_into().unwrap();
		
        <T as pallet_uniques::Config>::Currency::make_free_balance_be(&minter.clone(), AccountBalance::<T>::max_value());

		let _ = NftBench::<T>::create_collection(RawOrigin::Signed(minter.clone()).into(), collection_id);
	}: _(RawOrigin::Signed(minter.clone()), collection_id, nft_id, metadata, minter.clone(), true)
	verify {
		assert_last_event::<T>(Event::NftMinted { collection_id: collection_id, nft_id: nft_id, minter: minter }.into());
	}

	transfer {
        let minter: T::AccountId = whitelisted_caller();
        let collection_id: T::CollectionId = T::Helper::to_collection(2);
		let nft_id: T::ItemId = T::Helper::to_nft(10);
		let metadata = vec![0, 1, 2].try_into().unwrap();
		let receiver: T::AccountId = account("r", 1, 0);

        <T as pallet_uniques::Config>::Currency::make_free_balance_be(&minter.clone(), AccountBalance::<T>::max_value());

		let _ = NftBench::<T>::create_collection(RawOrigin::Signed(minter.clone()).into(), collection_id);
		let _ = NftBench::<T>::mint(RawOrigin::Signed(minter.clone()).into(), collection_id, nft_id, metadata, minter.clone(), true);
	}: _(RawOrigin::Signed(minter.clone()), collection_id, nft_id, receiver.clone())
	verify {
		assert_last_event::<T>(Event::NftTransfered { collection_id, nft_id: nft_id, from: minter, to: receiver }.into());
	}

	impl_benchmark_test_suite!(NftBench, crate::mock::new_test_ext(), crate::mock::Test);
}
