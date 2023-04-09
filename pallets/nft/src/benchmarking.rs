#![cfg(feature = "runtime-benchmarks")]

use super::*;

#[allow(unused)]
use crate::Pallet as NftBench;

use crate::{Error, Event, types::NftInfo};
use frame_benchmarking::v1::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use frame_support::{assert_ok, BoundedVec, traits::Currency};
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

	impl_benchmark_test_suite!(NftBench, crate::mock::new_test_ext(), crate::mock::Test);
}
