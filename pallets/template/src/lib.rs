#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;

	#[pallet::config]
	pub trait Config: frame_system:: Config {
		
	}
	#[pallet::event]
	#[pallet::error]
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::generate_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
}



