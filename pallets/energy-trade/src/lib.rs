#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{pallet_prelude::*, traits::Currency};
    use frame_system::pallet_prelude::*;
    use scale_info::TypeInfo;
    use sp_std::prelude::*;
    use pallet_energy_token;

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    pub enum OrderType {
        Ask,    // Seller's offer
        Bid,    // Buyer's offer
    }

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    pub enum OrderStatus {
        Open,
        Matched,
        InTransfer,
        Completed,
        Cancelled,
        Failed,
    }

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    #[scale_info(skip_type_params(T))]
    pub struct TradeOrder<T: Config> {
        pub order_type: OrderType,
        pub creator: T::AccountId,
        pub counterparty: Option<T::AccountId>,
        pub energy_amount: T::TokenBalance,
        pub price_per_unit: T::TokenBalance,
        pub total_price: T::TokenBalance,
        pub status: OrderStatus,
        pub grid_location: Vec<u8>,
        pub created_at: T::BlockNumber,
        pub matched_at: Option<T::BlockNumber>,
        pub completed_at: Option<T::BlockNumber>,
        pub transfer_verification: Option<T::Hash>,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_energy_token::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type TokenBalance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;
        type Currency: Currency<Self::AccountId>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn trade_orders)]
    pub type TradeOrders<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, TradeOrder<T>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn user_orders)]
    pub type UserOrders<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Vec<T::Hash>,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        AskOrderCreated {
            order_id: T::Hash,
            seller: T::AccountId,
            amount: T::TokenBalance,
            price: T::TokenBalance,
            location: Vec<u8>,
        },
        BidOrderCreated {
            order_id: T::Hash,
            buyer: T::AccountId,
            amount: T::TokenBalance,
            price: T::TokenBalance,
            location: Vec<u8>,
        },
        OrdersMatched {
            ask_id: T::Hash,
            bid_id: T::Hash,
            seller: T::AccountId,
            buyer: T::AccountId,
            amount: T::TokenBalance,
            price: T::TokenBalance,
        },
        TransferVerified {
            order_id: T::Hash,
            verification_hash: T::Hash,
        },
        OrderCompleted {
            order_id: T::Hash,
            seller: T::AccountId,
            buyer: T::AccountId,
            amount: T::TokenBalance,
            price: T::TokenBalance,
        },
        OrderFailed {
            order_id: T::Hash,
            reason: Vec<u8>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        OrderNotFound,
        InvalidOrderStatus,
        InsufficientBalance,
        UnauthorizedAccess,
        InvalidAmount,
        InvalidPrice,
        OrderMismatch,
        TransferVerificationFailed,
        PaymentFailed,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn create_ask_order(
            origin: OriginFor<T>,
            energy_amount: T::TokenBalance,
            price_per_unit: T::TokenBalance,
            grid_location: Vec<u8>,
        ) -> DispatchResult {
            let seller = ensure_signed(origin)?;
            ensure!(!energy_amount.is_zero(), Error::<T>::InvalidAmount);
            ensure!(!price_per_unit.is_zero(), Error::<T>::InvalidPrice);

            let total_price = price_per_unit
                .checked_mul(&energy_amount)
                .ok_or(Error::<T>::InvalidPrice)?;

            let order = TradeOrder {
                order_type: OrderType::Ask,
                creator: seller.clone(),
                counterparty: None,
                energy_amount,
                price_per_unit,
                total_price,
                status: OrderStatus::Open,
                grid_location: grid_location.clone(),
                created_at: <frame_system::Pallet<T>>::block_number(),
                matched_at: None,
                completed_at: None,
                transfer_verification: None,
            };

            let order_id = T::Hashing::hash_of(&order);
            <TradeOrders<T>>::insert(order_id, order);
            
            UserOrders::<T>::append(seller.clone(), order_id);

            Self::deposit_event(Event::AskOrderCreated {
                order_id,
                seller,
                amount: energy_amount,
                price: total_price,
                location: grid_location,
            });

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn create_bid_order(
            origin: OriginFor<T>,
            energy_amount: T::TokenBalance,
            price_per_unit: T::TokenBalance,
            grid_location: Vec<u8>,
        ) -> DispatchResult {
            let buyer = ensure_signed(origin)?;
            ensure!(!energy_amount.is_zero(), Error::<T>::InvalidAmount);
            ensure!(!price_per_unit.is_zero(), Error::<T>::InvalidPrice);

            let total_price = price_per_unit
                .checked_mul(&energy_amount)
                .ok_or(Error::<T>::InvalidPrice)?;

            // Check if buyer has enough balance
            ensure!(
                T::Currency::free_balance(&buyer) >= total_price,
                Error::<T>::InsufficientBalance
            );

            let order = TradeOrder {
                order_type: OrderType::Bid,
                creator: buyer.clone(),
                counterparty: None,
                energy_amount,
                price_per_unit,
                total_price,
                status: OrderStatus::Open,
                grid_location: grid_location.clone(),
                created_at: <frame_system::Pallet<T>>::block_number(),
                matched_at: None,
                completed_at: None,
                transfer_verification: None,
            };

            let order_id = T::Hashing::hash_of(&order);
            <TradeOrders<T>>::insert(order_id, order);
            
            UserOrders::<T>::append(buyer.clone(), order_id);

            Self::deposit_event(Event::BidOrderCreated {
                order_id,
                buyer,
                amount: energy_amount,
                price: total_price,
                location: grid_location,
            });

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn match_orders(
            origin: OriginFor<T>,
            ask_id: T::Hash,
            bid_id: T::Hash,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;

            let mut ask_order = TradeOrders::<T>::get(ask_id).ok_or(Error::<T>::OrderNotFound)?;
            let mut bid_order = TradeOrders::<T>::get(bid_id).ok_or(Error::<T>::OrderNotFound)?;

            ensure!(ask_order.status == OrderStatus::Open, Error::<T>::InvalidOrderStatus);
            ensure!(bid_order.status == OrderStatus::Open, Error::<T>::InvalidOrderStatus);
            ensure!(ask_order.energy_amount == bid_order.energy_amount, Error::<T>::OrderMismatch);
            ensure!(ask_order.price_per_unit <= bid_order.price_per_unit, Error::<T>::OrderMismatch);

            let current_block = <frame_system::Pallet<T>>::block_number();
            
            // Update orders
            ask_order.status = OrderStatus::Matched;
            ask_order.counterparty = Some(bid_order.creator.clone());
            ask_order.matched_at = Some(current_block);

            bid_order.status = OrderStatus::Matched;
            bid_order.counterparty = Some(ask_order.creator.clone());
            bid_order.matched_at = Some(current_block);

            <TradeOrders<T>>::insert(ask_id, ask_order.clone());
            <TradeOrders<T>>::insert(bid_id, bid_order.clone());

            Self::deposit_event(Event::OrdersMatched {
                ask_id,
                bid_id,
                seller: ask_order.creator,
                buyer: bid_order.creator,
                amount: ask_order.energy_amount,
                price: ask_order.total_price,
            });

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn verify_transfer(
            origin: OriginFor<T>,
            order_id: T::Hash,
            verification_data: Vec<u8>,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;

            TradeOrders::<T>::try_mutate(order_id, |order| -> DispatchResult {
                let order = order.as_mut().ok_or(Error::<T>::OrderNotFound)?;
                ensure!(order.status == OrderStatus::Matched, Error::<T>::InvalidOrderStatus);

                // Verify the transfer using IoT data
                let verification_hash = T::Hashing::hash_of(&verification_data);
                order.transfer_verification = Some(verification_hash);
                order.status = OrderStatus::InTransfer;

                Self::deposit_event(Event::TransferVerified {
                    order_id,
                    verification_hash,
                });

                Ok(())
            })
        }

        #[pallet::weight(10_000)]
        pub fn complete_trade(
            origin: OriginFor<T>,
            order_id: T::Hash,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;

            TradeOrders::<T>::try_mutate(order_id, |order| -> DispatchResult {
                let order = order.as_mut().ok_or(Error::<T>::OrderNotFound)?;
                ensure!(order.status == OrderStatus::InTransfer, Error::<T>::InvalidOrderStatus);
                ensure!(order.transfer_verification.is_some(), Error::<T>::TransferVerificationFailed);

                let seller = order.creator.clone();
                let buyer = order.counterparty.clone().ok_or(Error::<T>::OrderMismatch)?;

                // Transfer payment
                T::Currency::transfer(
                    &buyer,
                    &seller,
                    order.total_price,
                    frame_support::traits::ExistenceRequirement::KeepAlive,
                )?;

                // Update order status
                order.status = OrderStatus::Completed;
                order.completed_at = Some(<frame_system::Pallet<T>>::block_number());

                Self::deposit_event(Event::OrderCompleted {
                    order_id,
                    seller,
                    buyer,
                    amount: order.energy_amount,
                    price: order.total_price,
                });

                Ok(())
            })
        }
    }
}
