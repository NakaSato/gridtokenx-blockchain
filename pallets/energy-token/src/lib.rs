#![cfg_attr(not(feature = "std"), no_std)]

pub use frame_support::pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{pallet_prelude::*, traits::Currency};
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{AtLeast32BitUnsigned, Zero};

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type TokenBalance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn token_balance)]
    pub type TokenBalance<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        T::TokenBalance,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        TokensMinted {
            account: T::AccountId,
            amount: T::TokenBalance,
        },
        TokensTransferred {
            from: T::AccountId,
            to: T::AccountId,
            amount: T::TokenBalance,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        InsufficientBalance,
        OverflowError,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn mint_tokens(
            origin: OriginFor<T>,
            amount: T::TokenBalance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            
            TokenBalance::<T>::try_mutate(&who, |balance| -> DispatchResult {
                *balance = balance.checked_add(&amount)
                    .ok_or(Error::<T>::OverflowError)?;
                Ok(())
            })?;

            Self::deposit_event(Event::TokensMinted {
                account: who,
                amount,
            });
            
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn transfer(
            origin: OriginFor<T>,
            to: T::AccountId,
            amount: T::TokenBalance,
        ) -> DispatchResult {
            let from = ensure_signed(origin)?;
            
            TokenBalance::<T>::try_mutate(&from, |from_balance| -> DispatchResult {
                if *from_balance < amount {
                    return Err(Error::<T>::InsufficientBalance.into());
                }
                *from_balance = from_balance.checked_sub(&amount)
                    .ok_or(Error::<T>::OverflowError)?;
                
                TokenBalance::<T>::try_mutate(&to, |to_balance| -> DispatchResult {
                    *to_balance = to_balance.checked_add(&amount)
                        .ok_or(Error::<T>::OverflowError)?;
                    Ok(())
                })?;
                
                Ok(())
            })?;

            Self::deposit_event(Event::TokensTransferred {
                from,
                to,
                amount,
            });
            
            Ok(())
        }
    }
}
