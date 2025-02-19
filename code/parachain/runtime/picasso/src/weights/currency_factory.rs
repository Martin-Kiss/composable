
//! Autogenerated weights for `currency_factory`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-14, STEPS: `50`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `657e6acf5e95`, CPU: `Intel(R) Xeon(R) CPU @ 3.10GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("picasso-dev"), DB CACHE: 1024

// Executed Command:
// /nix/store/7as5b27dws6pfhhpjrs68qfvfx2ldcli-composable/bin/composable
// benchmark
// pallet
// --chain=picasso-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=10
// --output=code/parachain/runtime/picasso/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `currency_factory`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> currency_factory::WeightInfo for WeightInfo<T> {
	// Storage: CurrencyFactory AssetIdRanges (r:1 w:1)
	fn add_range() -> Weight {
		// Minimum execution time: 28_647 nanoseconds.
		Weight::from_ref_time(29_516_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: CurrencyFactory AssetMetadata (r:0 w:1)
	fn set_metadata() -> Weight {
		// Minimum execution time: 9_371 nanoseconds.
		Weight::from_ref_time(9_595_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
