
//! Autogenerated weights for `market_state`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-12, STEPS: `5`, REPEAT: 5, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `CT-HP`, CPU: `11th Gen Intel(R) Core(TM) i7-1195G7 @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/parachain-template-node
// benchmark
// pallet
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// market_state
// --extrinsic
// *
// --steps
// 5
// --repeat
// 5
// --output
// pallets/market-state/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `market_state`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> market_state::WeightInfo for WeightInfo<T> {
	// Storage: MarketState Stage (r:1 w:0)
	// Storage: MarketState AccountBids (r:1 w:1)
	// Storage: MarketState LastBidId (r:1 w:1)
	// Storage: MarketState Bids (r:0 w:1)
	/// The range of component `c` is `[1, 50]`.
	/// The range of component `l` is `[1, 5]`.
	fn submit_bids(c: u32, l: u32, ) -> Weight {
		// Minimum execution time: 39_806 nanoseconds.
		Weight::from_ref_time(10_443_397)
			// Standard Error: 127_139
			.saturating_add(Weight::from_ref_time(2_600_403).saturating_mul(c.into()))
			// Standard Error: 1_563_975
			.saturating_add(Weight::from_ref_time(6_597_784).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
	}
	// Storage: MarketState Stage (r:1 w:0)
	// Storage: MarketState AccountAsks (r:1 w:1)
	// Storage: MarketState LastAskId (r:1 w:1)
	// Storage: MarketState Asks (r:0 w:1)
	/// The range of component `c` is `[1, 50]`.
	/// The range of component `l` is `[1, 5]`.
	fn submit_asks(c: u32, l: u32, ) -> Weight {
		// Minimum execution time: 31_172 nanoseconds.
		Weight::from_ref_time(22_353_594)
			// Standard Error: 149_341
			.saturating_add(Weight::from_ref_time(2_402_210).saturating_mul(c.into()))
			// Standard Error: 1_837_080
			.saturating_add(Weight::from_ref_time(3_089_318).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
	}
}
