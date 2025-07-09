#![cfg_attr(not(feature = "std"), no_std)]

pub use frame_system::pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use pallet_energy_trade::{self as energy_trade, OrderType};
    use scale_info::TypeInfo;
    use sp_std::prelude::*;

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    pub struct PricePoint<T: Config> {
        pub price: T::TokenBalance,
        pub timestamp: T::BlockNumber,
        pub volume: T::TokenBalance,
        pub location: Vec<u8>,
    }

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    pub struct MarketData<T: Config> {
        pub current_price: T::TokenBalance,
        pub daily_high: T::TokenBalance,
        pub daily_low: T::TokenBalance,
        pub daily_volume: T::TokenBalance,
        pub price_history: Vec<PricePoint<T>>,
    }

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    pub struct GridMetrics {
        pub congestion_level: u8,    // 0-100
        pub loss_factor: u8,         // 0-100
        pub stability_index: u8,     // 0-100
    }

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    pub struct LocationPriority {
        pub location: Vec<u8>,
        pub priority: u8,            // 0-100
        pub distance_factor: u8,     // 0-100
    }

    #[pallet::config]
    pub trait Config: frame_system::Config + energy_trade::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn market_data)]
    pub type MarketDataStore<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        Vec<u8>,  // location
        MarketData<T>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn grid_metrics)]
    pub type GridMetricsStore<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        Vec<u8>,  // location
        GridMetrics,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn location_priorities)]
    pub type LocationPriorities<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        Vec<u8>,  // source_location
        Vec<LocationPriority>,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        PriceUpdated {
            location: Vec<u8>,
            new_price: T::TokenBalance,
        },
        GridMetricsUpdated {
            location: Vec<u8>,
            congestion: u8,
            loss_factor: u8,
        },
        OptimalMatchFound {
            ask_id: T::Hash,
            bid_id: T::Hash,
            matched_price: T::TokenBalance,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        InvalidPrice,
        InvalidMetrics,
        NoMarketData,
        PriceOutOfRange,
    }

    impl<T: Config> Pallet<T> {
        // Calculate optimal price based on market conditions and grid metrics
        pub fn calculate_optimal_price(
            location: &Vec<u8>,
            base_price: T::TokenBalance,
        ) -> Result<T::TokenBalance, Error<T>> {
            let market_data = Self::market_data(location).ok_or(Error::<T>::NoMarketData)?;
            let grid_metrics = Self::grid_metrics(location).ok_or(Error::<T>::NoMarketData)?;

            // Adjust price based on grid congestion
            let congestion_factor = grid_metrics.congestion_level as u32;
            let loss_factor = grid_metrics.loss_factor as u32;
            
            // Price adjustment formula:
            // optimal_price = base_price * (1 + congestion_factor/100) * (1 + loss_factor/100)
            let mut optimal_price = base_price;
            
            if congestion_factor > 0 {
                optimal_price = optimal_price.saturating_mul((100 + congestion_factor).into());
                optimal_price = optimal_price.saturating_div(100.into());
            }
            
            if loss_factor > 0 {
                optimal_price = optimal_price.saturating_mul((100 + loss_factor).into());
                optimal_price = optimal_price.saturating_div(100.into());
            }

            // Ensure price is within daily range
            ensure!(
                optimal_price >= market_data.daily_low && optimal_price <= market_data.daily_high,
                Error::<T>::PriceOutOfRange
            );

            Ok(optimal_price)
        }

        // Find optimal match considering location and grid conditions
        pub fn find_optimal_match(
            order_id: T::Hash,
            order_type: OrderType,
        ) -> Option<(T::Hash, T::TokenBalance)> {
            if let Some(order) = energy_trade::Pallet::<T>::trade_orders(order_id) {
                let location = order.grid_location.clone();
                let priorities = Self::location_priorities(&location);
                
                // Get all potential matching orders
                let matching_orders: Vec<_> = energy_trade::Pallet::<T>::trade_orders()
                    .iter()
                    .filter(|(_, o)| {
                        o.order_type != order_type && 
                        o.energy_amount == order.energy_amount &&
                        o.status == energy_trade::OrderStatus::Open
                    })
                    .collect();

                // Score each potential match
                let mut scored_matches: Vec<_> = matching_orders
                    .iter()
                    .filter_map(|(id, matching_order)| {
                        let location_score = Self::calculate_location_score(
                            &location,
                            &matching_order.grid_location,
                            &priorities,
                        );
                        
                        let price_score = Self::calculate_price_score(
                            &order,
                            matching_order,
                        );

                        let grid_score = Self::calculate_grid_score(
                            &location,
                            &matching_order.grid_location,
                        );

                        let total_score = location_score
                            .saturating_add(price_score)
                            .saturating_add(grid_score);

                        Some((*id, total_score, matching_order.price_per_unit))
                    })
                    .collect();

                // Sort by score and return the best match
                if !scored_matches.is_empty() {
                    scored_matches.sort_by(|a, b| b.1.cmp(&a.1));
                    let (matched_id, _, price) = scored_matches[0];
                    return Some((matched_id, price));
                }
            }
            None
        }

        // Calculate score based on location proximity and priorities
        fn calculate_location_score(
            source: &Vec<u8>,
            target: &Vec<u8>,
            priorities: &Vec<LocationPriority>,
        ) -> u32 {
            if let Some(priority) = priorities.iter().find(|p| p.location == *target) {
                return (priority.priority as u32)
                    .saturating_mul(2)
                    .saturating_add(priority.distance_factor as u32);
            }
            0
        }

        // Calculate score based on price matching
        fn calculate_price_score<O>(order: &O, matching_order: &O) -> u32 
        where
            O: PartialEq + Clone,
        {
            let price_diff = if matching_order.price_per_unit > order.price_per_unit {
                matching_order.price_per_unit - order.price_per_unit
            } else {
                order.price_per_unit - matching_order.price_per_unit
            };

            // Lower price difference = higher score
            if price_diff.is_zero() {
                100
            } else {
                let max_price = order.price_per_unit.max(matching_order.price_per_unit);
                let score = (price_diff * 100.into()) / max_price;
                100 - score.saturated_into::<u32>()
            }
        }

        // Calculate score based on grid conditions
        fn calculate_grid_score(source: &Vec<u8>, target: &Vec<u8>) -> u32 {
            let source_metrics = Self::grid_metrics(source);
            let target_metrics = Self::grid_metrics(target);

            match (source_metrics, target_metrics) {
                (Some(s), Some(t)) => {
                    let congestion_score = 100 - s.congestion_level.max(t.congestion_level) as u32;
                    let stability_score = (s.stability_index.min(t.stability_index) as u32)
                        .saturating_mul(2);
                    let loss_score = 100 - ((s.loss_factor + t.loss_factor) / 2) as u32;

                    congestion_score
                        .saturating_add(stability_score)
                        .saturating_add(loss_score)
                        .saturating_div(3)
                },
                _ => 0,
            }
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn update_market_data(
            origin: OriginFor<T>,
            location: Vec<u8>,
            price: T::TokenBalance,
            volume: T::TokenBalance,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            ensure!(!price.is_zero(), Error::<T>::InvalidPrice);

            MarketDataStore::<T>::try_mutate(&location, |market_data| -> DispatchResult {
                let data = market_data.get_or_insert(MarketData {
                    current_price: price,
                    daily_high: price,
                    daily_low: price,
                    daily_volume: volume,
                    price_history: Vec::new(),
                });

                // Update market data
                data.current_price = price;
                data.daily_high = data.daily_high.max(price);
                data.daily_low = data.daily_low.min(price);
                data.daily_volume = data.daily_volume.saturating_add(volume);

                // Add to price history
                data.price_history.push(PricePoint {
                    price,
                    timestamp: <frame_system::Pallet<T>>::block_number(),
                    volume,
                    location: location.clone(),
                });

                // Keep only last 24 hours of history
                if data.price_history.len() > 24 {
                    data.price_history.remove(0);
                }

                Self::deposit_event(Event::PriceUpdated {
                    location: location.clone(),
                    new_price: price,
                });

                Ok(())
            })
        }

        #[pallet::weight(10_000)]
        pub fn update_grid_metrics(
            origin: OriginFor<T>,
            location: Vec<u8>,
            congestion: u8,
            loss_factor: u8,
            stability: u8,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            
            ensure!(congestion <= 100, Error::<T>::InvalidMetrics);
            ensure!(loss_factor <= 100, Error::<T>::InvalidMetrics);
            ensure!(stability <= 100, Error::<T>::InvalidMetrics);

            let metrics = GridMetrics {
                congestion_level: congestion,
                loss_factor,
                stability_index: stability,
            };

            GridMetricsStore::<T>::insert(&location, metrics);

            Self::deposit_event(Event::GridMetricsUpdated {
                location,
                congestion,
                loss_factor,
            });

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn update_location_priorities(
            origin: OriginFor<T>,
            source: Vec<u8>,
            priorities: Vec<LocationPriority>,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            
            for priority in priorities.iter() {
                ensure!(priority.priority <= 100, Error::<T>::InvalidMetrics);
                ensure!(priority.distance_factor <= 100, Error::<T>::InvalidMetrics);
            }

            LocationPriorities::<T>::insert(&source, priorities);
            Ok(())
        }
    }
}
