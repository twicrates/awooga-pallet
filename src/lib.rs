#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use codec::{Decode, Encode};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch,
    traits::{Currency, Get, LockIdentifier, LockableCurrency, Randomness, Time, WithdrawReasons},
};
use frame_system::ensure_signed;
use sp_core::RuntimeDebug;
use sp_std::vec::Vec;

use awooga_pallet_commodities::nft::UniqueAssets;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

const MODULE_ID: LockIdentifier = *b"awooogas";

/// Attributes that uniquely identify a kitty
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, Default, RuntimeDebug)]
pub struct BoobaInfo<Hash, Moment> {
    dob: Moment,
    dna: Hash,
}

/// Attributes that do not uniquely identify a kitty
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, Default, RuntimeDebug)]
pub struct BoobaMetadata {
    name: Vec<u8>,
}

type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
type BoobaInfoOf<T> =
    BoobaInfo<<T as frame_system::Config>::Hash, <<T as Config>::Time as Time>::Moment>;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Boobas: UniqueAssets<
		Self::AccountId,
		AssetId = Self::Hash,
		AssetInfo = BoobaInfoOf<Self>
	>;
    type Time: frame_support::traits::Time;
    type Randomness: frame_support::traits::Randomness<Self::Hash>;
    type Currency: frame_support::traits::LockableCurrency<Self::AccountId>;
    type BasePrice: Get<BalanceOf<Self>>;
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
    trait Store for Module<T: Config> as Boobas {
        MetadataForBooba get(fn metadata_for_booba): map hasher(identity) T::Hash => BoobaMetadata;
    }
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
    pub enum Event<T>
    where
        BoobaId = <T as frame_system::Config>::Hash,
        AccountId = <T as frame_system::Config>::AccountId,
    {
        Conjured(BoobaId, AccountId),
    }
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Config> {
		BoobaConjureFail,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;

        /// Reserve funds from the sender's account before conjuring them a kitty.
        ///
        /// The dispatch origin for this call must be Signed.
        #[weight = 10_000]
        pub fn conjure(origin, name: Vec<u8>) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            T::Currency::set_lock(MODULE_ID, &who, T::BasePrice::get(), WithdrawReasons::FEE | WithdrawReasons::RESERVE);
            match T::Boobas::mint(&who, BoobaInfo{dob: T::Time::now(), dna: T::Randomness::random(&MODULE_ID)}) {
                Ok(id) => {
                    MetadataForBooba::<T>::insert(id, BoobaMetadata{name: name});
                    Self::deposit_event(RawEvent::Conjured(id, who));
                },
                Err(err) => Err(err)?
            }

            // TODO: allow senders to supply extra funds to lock, which will serve as a power boost

            Ok(())
        }

        // TODO: BOOST
        // power up a kitty by locking more funds
        // increases power without altering DNA
        // store as metadata in this pallet

        // TODO: RECOUP
        // remove boost and associated lock

        // TODO: FLIRT
        // post intent to breed, must have power boost

        // TODO: BREED
        // respond to intent to breed, must have power boost
        // DNA and power derived from parents
        // each parent randomly contributes power from boost
        // offspring owner randomly assigned between parent owners

        // TODO: SELL
        // post intent to sell including price

        // TODO: BUY
        // respond to intent to sell
        // transfer funds to seller and transfer kitty ownership

        // TODO: RELEASE
        // burn kitty and unlock funds
	}
}
