#![cfg(feature = "runtime-benchmarks")]

use super::*;

#[allow(unused)]
use crate::Pallet as NftBench;

use crate::{Error, Event, types::NftInfo};
use frame_benchmarking::v1::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use frame_support::{assert_ok, BoundedVec};

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

benchmarks! {
	create_collection {
        let caller: T::AccountId = whitelisted_caller();
        let collection_id: T::CollectionId = 2;
	}: _(RawOrigin::Signed(caller), collection_id)
	verify {
		assert_last_event(Event::CollectionCreated { collection_id: 2 });
	}

	impl_benchmark_test_suite!(NftBench, crate::mock::new_test_ext(), crate::mock::Test);
}
