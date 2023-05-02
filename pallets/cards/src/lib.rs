#![cfg_attr(not(feature = "std"), no_std)]
#![feature(more_qualified_paths)]

use codec::{Decode, Encode, MaxEncodedLen};
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, traits::EnsureOriginWithArg};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T, I = ()>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config<I: 'static = ()>:
		frame_system::Config + pallet_uniques::Config<I, CollectionId = u64>
	{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self, I>>
			+ IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T: Config<I>, I: 'static = ()> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config<I>, I: 'static = ()> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		CardMinted,
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T, I = ()> {
		CardAlreadyMinted,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn mint_colossus(origin: OriginFor<T>) -> DispatchResult {
			let collosus_collection_id: u64 = 1;
			let owner = <<T as pallet_uniques::Config<I>>::CreateOrigin>::ensure_origin(
				origin,
				&collosus_collection_id,
			)?;

			// pallet_uniques::Pallet::do_create_collection(collection, owner, admin, deposit,
			// free_holding, event)

			pallet_uniques::Pallet::do_create_collection(
				collosus_collection_id,
				owner.clone(),
				owner.clone(),
				T::CollectionDeposit::get(),
				false,
				<pallet_uniques::Event<T, I>>::Created {
					collection: collosus_collection_id,
					creator: owner.clone(),
					owner: owner.clone(),
				},
			)
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			Err(Error::<T, I>::CardAlreadyMinted.into())
		}
	}
}
