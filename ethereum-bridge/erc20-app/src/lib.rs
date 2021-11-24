//! # ERC20
//!
//! An application that implements bridged ERC20 token assets.
//!
//! ## Overview
//!
//! ETH balances are stored in the tightly-coupled [`asset`] runtime module. When an account holder
//! burns some of their balance, a `Transfer` event is emitted. An external relayer will listen for
//! this event and relay it to the other chain.
//!
//! ## Interface
//!
//! ### Dispatchable Calls
//!
//! - `burn`: Burn an ERC20 token balance.
#![cfg_attr(not(feature = "std"), no_std)]

mod payload;
mod weights;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;
//
// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

use ethereum_bridge_primitives::{ChannelId, OutboundRouter};
use frame_support::{
	dispatch::{DispatchError, DispatchResult},
	pallet_prelude::*,
	traits::EnsureOrigin,
	transactional,
};
use frame_system::{ensure_signed, pallet_prelude::*};
use node_primitives::{CurrencyId, TokenSymbol};
use orml_traits::MultiCurrency;
pub use pallet::*;
use payload::OutboundPayload;
use sp_core::{H160, U256};
use sp_runtime::traits::StaticLookup;
use sp_std::prelude::*;
pub use weights::WeightInfo;

#[allow(type_alias_bounds)]
type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

#[allow(type_alias_bounds)]
type BalanceOf<T: Config> = <<T as Config>::Assets as MultiCurrency<AccountIdOf<T>>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use sp_std::convert::TryInto;

	use super::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		// type Assets: MultiAsset<<Self as frame_system::Config>::AccountId>;
		type Assets: MultiCurrency<AccountIdOf<Self>, CurrencyId = CurrencyId>;

		type OutboundRouter: OutboundRouter<AccountIdOf<Self>>;

		type CallOrigin: EnsureOrigin<Self::Origin, Success = H160>;

		type WeightInfo: WeightInfo;
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Burned(H160, AccountIdOf<T>, H160, U256),
		Minted(H160, H160, AccountIdOf<T>, U256),
	}

	#[pallet::storage]
	#[pallet::getter(fn address)]
	pub(super) type Address<T: Config> = StorageValue<_, H160, ValueQuery>;

	#[pallet::error]
	pub enum Error<T> {
		ConvertFailure,
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig {
		pub address: H160,
	}

	#[cfg(feature = "std")]
	impl Default for GenesisConfig {
		fn default() -> Self {
			Self { address: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {
			<Address<T>>::put(self.address);
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::burn())]
		#[transactional]
		pub fn burn(
			origin: OriginFor<T>,
			channel_id: ChannelId,
			token: H160,
			recipient: H160,
			amount: U256,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let amount_u128 = TryInto::<BalanceOf<T>>::try_into(amount.as_u128())
				.map_err(|_| Error::<T>::ConvertFailure)?;

			T::Assets::withdraw(CurrencyId::Token(TokenSymbol::VETH), &who, amount_u128)?;

			let message = OutboundPayload {
				token,
				sender: who.clone(),
				recipient: recipient.clone(),
				amount,
			};

			T::OutboundRouter::submit(channel_id, &who, <Address<T>>::get(), &message.encode())?;
			Self::deposit_event(Event::Burned(token, who.clone(), recipient, amount));

			Ok(())
		}

		#[pallet::weight(T::WeightInfo::mint())]
		#[transactional]
		pub fn mint(
			origin: OriginFor<T>,
			token: H160,
			sender: H160,
			recipient: <T::Lookup as StaticLookup>::Source,
			amount: U256,
		) -> DispatchResult {
			let who = T::CallOrigin::ensure_origin(origin)?;
			if who != <Address<T>>::get() {
				return Err(DispatchError::BadOrigin.into());
			}
			let recipient = T::Lookup::lookup(recipient)?;
			let amount_u128 = TryInto::<BalanceOf<T>>::try_into(amount.as_u128())
				.map_err(|_| Error::<T>::ConvertFailure)?;
			T::Assets::deposit(CurrencyId::Token(TokenSymbol::VETH), &recipient, amount_u128)?;
			Self::deposit_event(Event::Minted(token, sender, recipient, amount));

			Ok(())
		}
	}
}
