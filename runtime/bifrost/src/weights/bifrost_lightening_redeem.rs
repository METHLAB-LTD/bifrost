// This file is part of Bifrost.

// Copyright (C) 2019-2021 Liebi Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for `bifrost_lightening_redeem`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-10-09, STEPS: `50`, REPEAT: 1, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("asgard-local"), DB CACHE: 128

// Executed Command:
// target/release/bifrost
// benchmark
// --chain=asgard-local
// --steps=50
// --repeat=1
// --pallet=bifrost_lightening_redeem
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./HEADER-GPL3
// --output=./runtime/asgard/src/weights/bifrost_lightening_redeem.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for bifrost_lightening_redeem.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> bifrost_lightening_redeem::WeightInfo for WeightInfo<T> {
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn add_ksm_to_pool() -> Weight {
		(89_467_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: LighteningRedeem PoolAmount (r:1 w:1)
	// Storage: LighteningRedeem ExchangePriceDiscount (r:1 w:0)
	// Storage: Tokens Accounts (r:6 w:6)
	// Storage: System Account (r:1 w:1)
	fn exchange_for_ksm() -> Weight {
		(189_565_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	// Storage: LighteningRedeem ExchangePriceDiscount (r:1 w:1)
	fn edit_exchange_price() -> Weight {
		(25_427_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: LighteningRedeem TokenReleasePerDay (r:1 w:1)
	fn edit_release_per_day() -> Weight {
		(25_718_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: LighteningRedeem StartEndReleaseBlock (r:1 w:1)
	fn edit_release_start_and_end_block() -> Weight {
		(25_608_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: LighteningRedeem StartEndReleaseBlock (r:1 w:0)
	fn on_initialize() -> Weight {
		(5_009_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
}
