#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use pallet_energy_trade::{self as energy_trade, TradeOrder};
    use scale_info::TypeInfo;
    use sp_std::prelude::*;

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    pub struct TransferData<T: Config> {
        pub order_id: T::Hash,
        pub start_time: T::Moment,
        pub end_time: Option<T::Moment>,
        pub energy_delivered: T::TokenBalance,
        pub grid_metrics: Vec<u8>,
        pub status: TransferStatus,
    }

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    pub enum TransferStatus {
        Pending,
        InProgress,
        Completed,
        Failed,
    }

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    pub struct IoTMeasurement {
        pub device_id: Vec<u8>,
        pub timestamp: u64,
        pub energy_amount: u64,
        pub grid_frequency: u32,
        pub voltage: u32,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config + energy_trade::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Moment: Parameter + Default + Copy;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn transfers)]
    pub type Transfers<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, TransferData<T>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn iot_measurements)]
    pub type IoTMeasurements<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::Hash,  // order_id
        Vec<IoTMeasurement>,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        TransferStarted {
            order_id: T::Hash,
            start_time: T::Moment,
        },
        MeasurementRecorded {
            order_id: T::Hash,
            device_id: Vec<u8>,
            energy_amount: u64,
        },
        TransferCompleted {
            order_id: T::Hash,
            total_energy: T::TokenBalance,
        },
        TransferFailed {
            order_id: T::Hash,
            reason: Vec<u8>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        TransferNotFound,
        InvalidTransferStatus,
        InvalidMeasurement,
        TransferAlreadyStarted,
        DeviceNotAuthorized,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn start_transfer(
            origin: OriginFor<T>,
            order_id: T::Hash,
            start_time: T::Moment,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;

            ensure!(!Transfers::<T>::contains_key(&order_id), Error::<T>::TransferAlreadyStarted);

            let transfer = TransferData {
                order_id,
                start_time,
                end_time: None,
                energy_delivered: T::TokenBalance::default(),
                grid_metrics: Vec::new(),
                status: TransferStatus::InProgress,
            };

            Transfers::<T>::insert(order_id, transfer);

            Self::deposit_event(Event::TransferStarted {
                order_id,
                start_time,
            });

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn record_measurement(
            origin: OriginFor<T>,
            order_id: T::Hash,
            measurement: IoTMeasurement,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;

            ensure!(Transfers::<T>::contains_key(&order_id), Error::<T>::TransferNotFound);

            let mut measurements = IoTMeasurements::<T>::get(&order_id);
            measurements.push(measurement.clone());
            IoTMeasurements::<T>::insert(order_id, measurements);

            Self::deposit_event(Event::MeasurementRecorded {
                order_id,
                device_id: measurement.device_id,
                energy_amount: measurement.energy_amount,
            });

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn complete_transfer(
            origin: OriginFor<T>,
            order_id: T::Hash,
            end_time: T::Moment,
            final_measurement: IoTMeasurement,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;

            Transfers::<T>::try_mutate(order_id, |transfer_opt| -> DispatchResult {
                let transfer = transfer_opt.as_mut().ok_or(Error::<T>::TransferNotFound)?;
                ensure!(transfer.status == TransferStatus::InProgress, Error::<T>::InvalidTransferStatus);

                // Record final measurement
                let mut measurements = IoTMeasurements::<T>::get(&order_id);
                measurements.push(final_measurement.clone());
                IoTMeasurements::<T>::insert(order_id, measurements);

                // Update transfer data
                transfer.end_time = Some(end_time);
                transfer.energy_delivered = T::TokenBalance::from(final_measurement.energy_amount as u32);
                transfer.status = TransferStatus::Completed;

                // Verify transfer on the trade pallet
                energy_trade::Pallet::<T>::verify_transfer(
                    origin,
                    order_id,
                    final_measurement.encode(),
                )?;

                Self::deposit_event(Event::TransferCompleted {
                    order_id,
                    total_energy: transfer.energy_delivered,
                });

                Ok(())
            })
        }

        #[pallet::weight(10_000)]
        pub fn report_transfer_failure(
            origin: OriginFor<T>,
            order_id: T::Hash,
            reason: Vec<u8>,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;

            Transfers::<T>::try_mutate(order_id, |transfer_opt| -> DispatchResult {
                let transfer = transfer_opt.as_mut().ok_or(Error::<T>::TransferNotFound)?;
                transfer.status = TransferStatus::Failed;

                Self::deposit_event(Event::TransferFailed {
                    order_id,
                    reason,
                });

                Ok(())
            })
        }
    }
}
