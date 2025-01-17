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

//! Vesting pallet benchmarking.

#![cfg(feature = "runtime-benchmarks")]

use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::{Pallet as System, RawOrigin};
use sp_runtime::traits::Bounded;

use super::*;
use crate::Pallet as Vesting;

const SEED: u32 = 0;

type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

fn add_locks<T: Config>(who: &T::AccountId, n: u8) {
	for id in 0..n {
		let lock_id = [id; 8];
		let locked = 100u32;
		let reasons = WithdrawReasons::TRANSFER | WithdrawReasons::RESERVE;
		T::Currency::set_lock(lock_id, who, locked.into(), reasons);
	}
}

fn add_vesting_schedule<T: Config>(who: &T::AccountId) -> Result<(), &'static str> {
	let locked = 100u32;
	let per_block = 10u32;
	let starting_block = 1u32;

	System::<T>::set_block_number(0u32.into());

	Vesting::<T>::init_vesting_start_at(RawOrigin::Root.into(), 0u32.into())?;

	// Add schedule to avoid `NotVesting` error.
	Vesting::<T>::add_vesting_schedule(
		&who,
		locked.into(),
		per_block.into(),
		starting_block.into(),
	)?;
	Ok(())
}

benchmarks! {
	vest_locked {
		let l in 0 .. MaxLocksOf::<T>::get();

		let caller = whitelisted_caller();
		T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
		add_locks::<T>(&caller, l as u8);
		add_vesting_schedule::<T>(&caller)?;
		// At block zero, everything is vested.
		System::<T>::set_block_number(T::BlockNumber::zero());
		assert_eq!(
			Vesting::<T>::vesting_balance(&caller),
			Some(100u32.into()),
			"Vesting schedule not added",
		);
	}: vest(RawOrigin::Signed(caller.clone()))
	verify {
		// Nothing happened since everything is still vested.
		assert_eq!(
			Vesting::<T>::vesting_balance(&caller),
			Some(100u32.into()),
			"Vesting schedule was removed",
		);
	}

	vest_unlocked {
		let l in 0 .. MaxLocksOf::<T>::get();

		let caller = whitelisted_caller();
		T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
		add_locks::<T>(&caller, l as u8);
		add_vesting_schedule::<T>(&caller)?;
		// At block 20, everything is unvested.
		System::<T>::set_block_number(20u32.into());
		assert_eq!(
			Vesting::<T>::vesting_balance(&caller),
			Some(BalanceOf::<T>::zero()),
			"Vesting schedule still active",
		);
	}: vest(RawOrigin::Signed(caller.clone()))
	verify {
		// Vesting schedule is removed!
		assert_eq!(
			Vesting::<T>::vesting_balance(&caller),
			None,
			"Vesting schedule was not removed",
		);
	}

	vest_other_locked {
		let l in 0 .. MaxLocksOf::<T>::get();

		let other: T::AccountId = account("other", 0, SEED);
		let other_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(other.clone());
		T::Currency::make_free_balance_be(&other, BalanceOf::<T>::max_value());
		add_locks::<T>(&other, l as u8);
		add_vesting_schedule::<T>(&other)?;
		// At block zero, everything is vested.
		System::<T>::set_block_number(T::BlockNumber::zero());
		assert_eq!(
			Vesting::<T>::vesting_balance(&other),
			Some(100u32.into()),
			"Vesting schedule not added",
		);

		let caller: T::AccountId = whitelisted_caller();
	}: vest_other(RawOrigin::Signed(caller.clone()), other_lookup)
	verify {
		// Nothing happened since everything is still vested.
		assert_eq!(
			Vesting::<T>::vesting_balance(&other),
			Some(100u32.into()),
			"Vesting schedule was removed",
		);
	}

	vest_other_unlocked {
		let l in 0 .. MaxLocksOf::<T>::get();

		let other: T::AccountId = account("other", 0, SEED);
		let other_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(other.clone());
		T::Currency::make_free_balance_be(&other, BalanceOf::<T>::max_value());
		add_locks::<T>(&other, l as u8);
		add_vesting_schedule::<T>(&other)?;
		// At block 20, everything is unvested.
		System::<T>::set_block_number(20u32.into());
		assert_eq!(
			Vesting::<T>::vesting_balance(&other),
			Some(BalanceOf::<T>::zero()),
			"Vesting schedule still active",
		);

		let caller: T::AccountId = whitelisted_caller();
	}: vest_other(RawOrigin::Signed(caller.clone()), other_lookup)
	verify {
		// Vesting schedule is removed!
		assert_eq!(
			Vesting::<T>::vesting_balance(&other),
			None,
			"Vesting schedule was not removed",
		);
	}

	vested_transfer {
		let l in 0 .. MaxLocksOf::<T>::get();

		let caller: T::AccountId = whitelisted_caller();
		T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
		let target: T::AccountId = account("target", 0, SEED);
		let target_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(target.clone());
		// Give target existing locks
		add_locks::<T>(&target, l as u8);

		let transfer_amount = T::MinVestedTransfer::get();

		let vesting_schedule = VestingInfo {
			locked: transfer_amount,
			per_block: 10u32.into(),
			starting_block: 1u32.into(),
		};
	}: _(RawOrigin::Signed(caller), target_lookup, vesting_schedule)
	verify {
		assert_eq!(
			T::MinVestedTransfer::get(),
			T::Currency::free_balance(&target),
			"Transfer didn't happen",
		);
		assert_eq!(
			Vesting::<T>::vesting_balance(&target),
			Some(T::MinVestedTransfer::get()),
			"Lock not created",
		);
	}

	force_vested_transfer {
		let l in 0 .. MaxLocksOf::<T>::get();

		let source: T::AccountId = account("source", 0, SEED);
		let source_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(source.clone());
		T::Currency::make_free_balance_be(&source, BalanceOf::<T>::max_value());
		let target: T::AccountId = account("target", 0, SEED);
		let target_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(target.clone());
		// Give target existing locks
		add_locks::<T>(&target, l as u8);

		let transfer_amount = T::MinVestedTransfer::get();

		let vesting_schedule = VestingInfo {
			locked: transfer_amount,
			per_block: 10u32.into(),
			starting_block: 1u32.into(),
		};
	}: _(RawOrigin::Root, source_lookup, target_lookup, vesting_schedule)
	verify {
		assert_eq!(
			T::MinVestedTransfer::get(),
			T::Currency::free_balance(&target),
			"Transfer didn't happen",
		);
		assert_eq!(
			Vesting::<T>::vesting_balance(&target),
			Some(T::MinVestedTransfer::get()),
			"Lock not created",
		);
	}
}

impl_benchmark_test_suite!(
	Vesting,
	crate::tests::ExtBuilder::default().existential_deposit(256).build(),
	crate::tests::Test,
);
