#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
mod types;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, Blake2_128Concat};
	use frame_system::pallet_prelude::*;

use crate::types::NftInfo;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_uniques::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::storage]
	#[pallet::getter(fn nfts)]
	pub type Nfts<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::CollectionId,
		Blake2_128Concat,
		T::ItemId,
		crate::types::NftInfo<T::AccountId>
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		NftMinted { collection_id: T::CollectionId, nft_id: T::ItemId, minter: T::AccountId },
		NftTransfered { collection_id: T::CollectionId, nft_id: T::ItemId, from: T::AccountId, to: T::AccountId }
	}

	#[pallet::error]
	pub enum Error<T> {
		NftExists,
		NftNotFound,
		SenderNotOwner,
	}

	/// Private methods
	impl<T: Config> Pallet<T> {
		pub fn nft_exists(collection_id: T::CollectionId, item_id: T::ItemId) -> bool {
			Nfts::<T>::get(collection_id, item_id).is_some()
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn mint(origin: OriginFor<T>,
			collection_id: T::CollectionId,
			item_id: T::ItemId,
			metadata: BoundedVec<u8, ConstU32<128>>,
			owner: T::AccountId,
			transferable: bool,
		) -> DispatchResult {
			ensure!(!Self::nft_exists(collection_id, item_id), Error::<T>::NftExists);

			let minter = ensure_signed(origin)?;
			let nft = NftInfo {
				minter: minter.clone(),
				metadata,
				transferable
			};

			Nfts::<T>::insert(collection_id, item_id, nft);
			// NOTE: collection is hardcoded for now
			pallet_uniques::Pallet::<T>::do_mint(collection_id, item_id, owner.clone(), |_| {
				Ok(())
			})?;

			Self::deposit_event(Event::NftMinted { collection_id, nft_id: item_id, minter });

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn transfer(origin: OriginFor<T>,
			collection_id: T::CollectionId,
			item_id: T::ItemId,
			to: T::AccountId
		) -> DispatchResult {
			ensure!(Self::nft_exists(collection_id, item_id), Error::<T>::NftExists);
			let sender = ensure_signed(origin)?;

			pallet_uniques::Pallet::<T>::do_transfer(collection_id, item_id, to.clone(), |_, _| {
				Ok(())
			})?;

			Self::deposit_event(Event::NftTransfered { collection_id, nft_id: item_id, from: sender, to });

			Ok(())
		}
	}
}
