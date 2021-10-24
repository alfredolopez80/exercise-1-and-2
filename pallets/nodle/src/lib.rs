#![cfg_attr(not(feature = "std"), no_std)]

/// Test Pallet for Nodle
/// This pallet is used to proof knowledge about Substrate
/// and in this case is a Pallet for Update Gradually On Chain

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

	#[pallet::config]
	pub trait Config: frame_system::Config {
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

}
