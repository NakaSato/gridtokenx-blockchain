#![cfg_attr(not(feature = "std"), no_std)]

pub use frame_system::pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{pallet_prelude::*, traits::Currency};
    use frame_system::pallet_prelude::*;
    use pallet_energy_trade::{self as energy_trade};
    use scale_info::TypeInfo;
    use sp_std::prelude::*;

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    pub enum PaymentMethod {
        Native,             // Platform's native token
        Fiat,              // Traditional currency
        Stablecoin,        // USD-pegged cryptocurrency
        ExternalToken,     // Other cryptocurrency
    }

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    pub enum PaymentStatus {
        Pending,
        Processing,
        Completed,
        Failed,
        Refunded,
    }

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    pub struct ExchangeRate {
        pub from_token: Vec<u8>,
        pub to_token: Vec<u8>,
        pub rate: u128,
        pub timestamp: u64,
    }

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    #[scale_info(skip_type_params(T))]
    pub struct Payment<T: Config> {
        pub order_id: T::Hash,
        pub payer: T::AccountId,
        pub payee: T::AccountId,
        pub amount: T::TokenBalance,
        pub payment_method: PaymentMethod,
        pub status: PaymentStatus,
        pub external_reference: Option<Vec<u8>>,
        pub timestamp: T::BlockNumber,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config + energy_trade::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn payments)]
    pub type Payments<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::Hash,  // payment_id
        Payment<T>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn exchange_rates)]
    pub type ExchangeRates<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        (Vec<u8>, Vec<u8>),  // (from_token, to_token)
        ExchangeRate,
        OptionQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        PaymentCreated {
            payment_id: T::Hash,
            order_id: T::Hash,
            amount: T::TokenBalance,
            method: PaymentMethod,
        },
        PaymentCompleted {
            payment_id: T::Hash,
            order_id: T::Hash,
        },
        PaymentFailed {
            payment_id: T::Hash,
            reason: Vec<u8>,
        },
        ExchangeRateUpdated {
            from_token: Vec<u8>,
            to_token: Vec<u8>,
            rate: u128,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        PaymentNotFound,
        InvalidPaymentStatus,
        InvalidAmount,
        PaymentMethodNotSupported,
        ExchangeRateNotFound,
        InsufficientBalance,
        ExternalPaymentFailed,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn create_payment(
            origin: OriginFor<T>,
            order_id: T::Hash,
            payment_method: PaymentMethod,
            external_reference: Option<Vec<u8>>,
        ) -> DispatchResult {
            let payer = ensure_signed(origin)?;

            let order = energy_trade::Pallet::<T>::trade_orders(order_id)
                .ok_or(Error::<T>::PaymentNotFound)?;

            let payment = Payment {
                order_id,
                payer: payer.clone(),
                payee: order.creator.clone(),
                amount: order.total_price,
                payment_method: payment_method.clone(),
                status: PaymentStatus::Pending,
                external_reference,
                timestamp: <frame_system::Pallet<T>>::block_number(),
            };

            let payment_id = T::Hashing::hash_of(&payment);
            <Payments<T>>::insert(payment_id, payment);

            Self::deposit_event(Event::PaymentCreated {
                payment_id,
                order_id,
                amount: order.total_price,
                method: payment_method,
            });

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn process_native_payment(
            origin: OriginFor<T>,
            payment_id: T::Hash,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;

            Payments::<T>::try_mutate(payment_id, |payment_opt| -> DispatchResult {
                let payment = payment_opt.as_mut().ok_or(Error::<T>::PaymentNotFound)?;
                ensure!(payment.status == PaymentStatus::Pending, Error::<T>::InvalidPaymentStatus);
                ensure!(payment.payment_method == PaymentMethod::Native, Error::<T>::PaymentMethodNotSupported);

                // Transfer native tokens
                T::Currency::transfer(
                    &payment.payer,
                    &payment.payee,
                    payment.amount,
                    frame_support::traits::ExistenceRequirement::KeepAlive,
                )?;

                payment.status = PaymentStatus::Completed;

                Self::deposit_event(Event::PaymentCompleted {
                    payment_id,
                    order_id: payment.order_id,
                });

                Ok(())
            })
        }

        #[pallet::weight(10_000)]
        pub fn process_external_payment(
            origin: OriginFor<T>,
            payment_id: T::Hash,
            proof: Vec<u8>,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;

            Payments::<T>::try_mutate(payment_id, |payment_opt| -> DispatchResult {
                let payment = payment_opt.as_mut().ok_or(Error::<T>::PaymentNotFound)?;
                ensure!(payment.status == PaymentStatus::Pending, Error::<T>::InvalidPaymentStatus);
                
                match payment.payment_method {
                    PaymentMethod::Fiat | PaymentMethod::Stablecoin | PaymentMethod::ExternalToken => {
                        // Verify external payment proof
                        if Self::verify_external_payment(&proof) {
                            payment.status = PaymentStatus::Completed;
                            
                            Self::deposit_event(Event::PaymentCompleted {
                                payment_id,
                                order_id: payment.order_id,
                            });
                        } else {
                            payment.status = PaymentStatus::Failed;
                            
                            Self::deposit_event(Event::PaymentFailed {
                                payment_id,
                                reason: b"External payment verification failed".to_vec(),
                            });
                            
                            return Err(Error::<T>::ExternalPaymentFailed.into());
                        }
                    },
                    _ => return Err(Error::<T>::PaymentMethodNotSupported.into()),
                }

                Ok(())
            })
        }

        #[pallet::weight(10_000)]
        pub fn update_exchange_rate(
            origin: OriginFor<T>,
            from_token: Vec<u8>,
            to_token: Vec<u8>,
            rate: u128,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;

            let exchange_rate = ExchangeRate {
                from_token: from_token.clone(),
                to_token: to_token.clone(),
                rate,
                timestamp: sp_io::offchain::timestamp().unix_millis(),
            };

            ExchangeRates::<T>::insert((from_token.clone(), to_token.clone()), exchange_rate);

            Self::deposit_event(Event::ExchangeRateUpdated {
                from_token,
                to_token,
                rate,
            });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        // Verify external payment proof (implement actual verification logic)
        fn verify_external_payment(proof: &[u8]) -> bool {
            // TODO: Implement actual verification logic
            // This could involve:
            // 1. Verifying cryptographic signatures
            // 2. Checking external API responses
            // 3. Validating transaction hashes
            !proof.is_empty()
        }

        // Convert amount between different currencies
        pub fn convert_amount(
            amount: T::TokenBalance,
            from_token: &[u8],
            to_token: &[u8],
        ) -> Result<T::TokenBalance, Error<T>> {
            if let Some(rate) = ExchangeRates::<T>::get((from_token.to_vec(), to_token.to_vec())) {
                let amount_u128: u128 = amount.saturated_into();
                let converted = amount_u128.saturating_mul(rate.rate);
                Ok(converted.saturated_into())
            } else {
                Err(Error::<T>::ExchangeRateNotFound)
            }
        }
    }
}
