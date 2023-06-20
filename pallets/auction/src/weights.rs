// This file is part of Metaverse.Network & Bit.Country.

// Copyright (C) 2020-2022 Metaverse.Network & Bit.Country .
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for auction
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-24, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/metaverse-node
// benchmark
// pallet
// --pallet=auction
// --extrinsic=*
// --steps=20
// --repeat=10
// --execution=wasm
// --wasm-execution=compiled
// --template=./template/weight-template.hbs
// --output
// ./pallets/auction/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for auction.
pub trait WeightInfo {	fn create_new_auction() -> Weight;	fn create_new_buy_now() -> Weight;	fn bid() -> Weight;	fn buy_now() -> Weight;	fn cancel_listing() -> Weight;	fn authorise_metaverse_collection() -> Weight;	fn remove_authorise_metaverse_collection() -> Weight;	fn make_offer() -> Weight;	fn withdraw_offer() -> Weight;	fn accept_offer() -> Weight;	fn on_finalize() -> Weight;}

/// Weights for auction using the for collator node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {	fn create_new_auction() -> Weight {
		Weight::from_ref_time(50_000_000)			.saturating_add(T::DbWeight::get().reads(10))			.saturating_add(T::DbWeight::get().writes(8))	}	fn create_new_buy_now() -> Weight {
		Weight::from_ref_time(49_000_000)			.saturating_add(T::DbWeight::get().reads(10))			.saturating_add(T::DbWeight::get().writes(8))	}	fn bid() -> Weight {
		Weight::from_ref_time(38_000_000)			.saturating_add(T::DbWeight::get().reads(4))			.saturating_add(T::DbWeight::get().writes(6))	}	fn buy_now() -> Weight {
		Weight::from_ref_time(100_000_000)			.saturating_add(T::DbWeight::get().reads(13))			.saturating_add(T::DbWeight::get().writes(13))	}	fn cancel_listing() -> Weight {
		Weight::from_ref_time(38_000_000)			.saturating_add(T::DbWeight::get().reads(5))			.saturating_add(T::DbWeight::get().writes(7))	}	fn authorise_metaverse_collection() -> Weight {
		Weight::from_ref_time(16_000_000)			.saturating_add(T::DbWeight::get().reads(3))			.saturating_add(T::DbWeight::get().writes(2))	}	fn remove_authorise_metaverse_collection() -> Weight {
		Weight::from_ref_time(16_000_000)			.saturating_add(T::DbWeight::get().reads(3))			.saturating_add(T::DbWeight::get().writes(2))	}	fn make_offer() -> Weight {
		Weight::from_ref_time(30_000_000)			.saturating_add(T::DbWeight::get().reads(5))			.saturating_add(T::DbWeight::get().writes(3))	}	fn withdraw_offer() -> Weight {
		Weight::from_ref_time(23_000_000)			.saturating_add(T::DbWeight::get().reads(3))			.saturating_add(T::DbWeight::get().writes(3))	}	fn accept_offer() -> Weight {
		Weight::from_ref_time(55_000_000)			.saturating_add(T::DbWeight::get().reads(7))			.saturating_add(T::DbWeight::get().writes(7))	}	fn on_finalize() -> Weight {
		Weight::from_ref_time(2_000_000)	}}

// For backwards compatibility and tests
impl WeightInfo for () {	fn create_new_auction() -> Weight {
		Weight::from_ref_time(50_000_000)			.saturating_add(RocksDbWeight::get().reads(10))			.saturating_add(RocksDbWeight::get().writes(8))	}	fn create_new_buy_now() -> Weight {
		Weight::from_ref_time(49_000_000)			.saturating_add(RocksDbWeight::get().reads(10))			.saturating_add(RocksDbWeight::get().writes(8))	}	fn bid() -> Weight {
		Weight::from_ref_time(38_000_000)			.saturating_add(RocksDbWeight::get().reads(4))			.saturating_add(RocksDbWeight::get().writes(6))	}	fn buy_now() -> Weight {
		Weight::from_ref_time(100_000_000)			.saturating_add(RocksDbWeight::get().reads(13))			.saturating_add(RocksDbWeight::get().writes(13))	}	fn cancel_listing() -> Weight {
		Weight::from_ref_time(38_000_000)			.saturating_add(RocksDbWeight::get().reads(5))			.saturating_add(RocksDbWeight::get().writes(7))	}	fn authorise_metaverse_collection() -> Weight {
		Weight::from_ref_time(16_000_000)			.saturating_add(RocksDbWeight::get().reads(3))			.saturating_add(RocksDbWeight::get().writes(2))	}	fn remove_authorise_metaverse_collection() -> Weight {
		Weight::from_ref_time(16_000_000)			.saturating_add(RocksDbWeight::get().reads(3))			.saturating_add(RocksDbWeight::get().writes(2))	}	fn make_offer() -> Weight {
		Weight::from_ref_time(30_000_000)			.saturating_add(RocksDbWeight::get().reads(5))			.saturating_add(RocksDbWeight::get().writes(3))	}	fn withdraw_offer() -> Weight {
		Weight::from_ref_time(23_000_000)			.saturating_add(RocksDbWeight::get().reads(3))			.saturating_add(RocksDbWeight::get().writes(3))	}	fn accept_offer() -> Weight {
		Weight::from_ref_time(55_000_000)			.saturating_add(RocksDbWeight::get().reads(7))			.saturating_add(RocksDbWeight::get().writes(7))	}	fn on_finalize() -> Weight {
		Weight::from_ref_time(2_000_000)	}}
