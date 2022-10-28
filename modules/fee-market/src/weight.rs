// This file is part of Substrate.

// Copyright (C) 2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for darwinia_fee_market
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-11-16, STEPS: [100, ], REPEAT: 50, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/drml
// benchmark
// --chain
// dev
// --wasm-execution
// compiled
// --pallet
// darwinia_fee_market
// --execution
// wasm
// --extrinsic
// *
// --steps
// 100
// --repeat
// 50
// --raw
// --heap-pages=4096
// --output=./frame/fee-market/src/weight.rs
// --template=./.maintain/frame-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for darwinia_fee_market.
pub trait WeightInfo {
	fn enroll_and_lock_collateral() -> Weight;
	fn increase_locked_collateral() -> Weight;
	fn decrease_locked_collateral() -> Weight;
	fn update_relay_fee() -> Weight;
	fn cancel_enrollment() -> Weight;
	fn set_slash_protect() -> Weight;
	fn set_assigned_relayers_number() -> Weight;
}

/// Weights for darwinia_fee_market using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn enroll_and_lock_collateral() -> Weight {
		Weight::from_ref_time(196_247_000 as u64)
			.saturating_add(T::DbWeight::get().reads(25 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}

	fn increase_locked_collateral() -> Weight {
		Weight::from_ref_time(192_480_000 as u64)
			.saturating_add(T::DbWeight::get().reads(25 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}

	fn decrease_locked_collateral() -> Weight {
		Weight::from_ref_time(200_415_000 as u64)
			.saturating_add(T::DbWeight::get().reads(25 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}

	// Storage: PangoroFeeMarket Relayers (r:1 w:0) Storage: PangoroFeeMarket RelayersMap (r:20 w:1)
	// Storage: PangoroFeeMarket Orders (r:1 w:0) Storage: PangoroFeeMarket AssignedRelayersNumber
	// (r:1 w:0) Storage: PangoroFeeMarket AssignedRelayers (r:0 w:1)
	fn update_relay_fee() -> Weight {
		Weight::from_ref_time(178_163_000 as u64)
			.saturating_add(T::DbWeight::get().reads(23 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}

	fn cancel_enrollment() -> Weight {
		Weight::from_ref_time(192_109_000 as u64)
			.saturating_add(T::DbWeight::get().reads(25 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}

	fn set_slash_protect() -> Weight {
		Weight::from_ref_time(17_332_000 as u64).saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn set_assigned_relayers_number() -> Weight {
		Weight::from_ref_time(170_128_000 as u64)
			.saturating_add(T::DbWeight::get().reads(22 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn enroll_and_lock_collateral() -> Weight {
		Weight::from_ref_time(196_247_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(25 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}

	fn increase_locked_collateral() -> Weight {
		Weight::from_ref_time(192_480_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(25 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}

	fn decrease_locked_collateral() -> Weight {
		Weight::from_ref_time(200_415_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(25 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}

	fn update_relay_fee() -> Weight {
		Weight::from_ref_time(178_163_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(23 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}

	fn cancel_enrollment() -> Weight {
		Weight::from_ref_time(192_109_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(25 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}

	fn set_slash_protect() -> Weight {
		Weight::from_ref_time(17_332_000 as u64).saturating_add(RocksDbWeight::get().writes(1 as u64))
	}

	fn set_assigned_relayers_number() -> Weight {
		Weight::from_ref_time(170_128_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(22 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
}
