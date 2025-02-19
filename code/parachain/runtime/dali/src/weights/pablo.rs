
//! Autogenerated weights for `pablo`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-14, STEPS: `50`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ddf18ea9c649`, CPU: `Intel(R) Xeon(R) CPU @ 3.10GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dali-dev"), DB CACHE: 1024

// Executed Command:
// /nix/store/7as5b27dws6pfhhpjrs68qfvfx2ldcli-composable/bin/composable
// benchmark
// pallet
// --chain=dali-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=10
// --output=code/parachain/runtime/dali/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pablo`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pablo::WeightInfo for WeightInfo<T> {
	// Storage: CurrencyFactory AssetIdRanges (r:1 w:1)
	// Storage: Pablo PoolCount (r:1 w:1)
	// Storage: Pablo Pools (r:0 w:1)
	fn create() -> Weight {
		// Minimum execution time: 43_469 nanoseconds.
		Weight::from_ref_time(46_233_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Pablo Pools (r:1 w:0)
	// Storage: Tokens Accounts (r:5 w:5)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Pablo PriceCumulativeState (r:1 w:1)
	fn add_liquidity() -> Weight {
		// Minimum execution time: 238_908 nanoseconds.
		Weight::from_ref_time(245_625_000)
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	// Storage: Pablo Pools (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: Tokens Accounts (r:5 w:5)
	// Storage: System Account (r:1 w:0)
	// Storage: Pablo PriceCumulativeState (r:1 w:1)
	fn remove_liquidity() -> Weight {
		// Minimum execution time: 145_189 nanoseconds.
		Weight::from_ref_time(148_912_000)
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	// Storage: Pablo Pools (r:1 w:0)
	// Storage: Tokens Accounts (r:4 w:4)
	// Storage: System Account (r:2 w:1)
	// Storage: Pablo PriceCumulativeState (r:1 w:1)
	fn buy() -> Weight {
		// Minimum execution time: 127_921 nanoseconds.
		Weight::from_ref_time(131_651_000)
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: Pablo Pools (r:1 w:0)
	// Storage: Tokens Accounts (r:4 w:4)
	// Storage: System Account (r:2 w:1)
	// Storage: Pablo PriceCumulativeState (r:1 w:1)
	fn swap() -> Weight {
		// Minimum execution time: 130_498 nanoseconds.
		Weight::from_ref_time(136_949_000)
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: CurrencyFactory AssetIdRanges (r:1 w:1)
	// Storage: Pablo PoolCount (r:1 w:1)
	// Storage: Pablo Pools (r:0 w:1)
	fn do_create_pool() -> Weight {
		// Minimum execution time: 38_715 nanoseconds.
		Weight::from_ref_time(39_914_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}
