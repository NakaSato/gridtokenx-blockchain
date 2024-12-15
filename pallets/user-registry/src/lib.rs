#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{pallet_prelude::*, traits::Currency};
    use frame_system::pallet_prelude::*;
    use scale_info::TypeInfo;
    use sp_std::prelude::*;

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    pub enum UserRole {
        Consumer,
        Prosumer,
        GridOperator,
        Admin,
    }

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    #[scale_info(skip_type_params(T))]
    pub struct UserProfile<T: Config> {
        pub role: UserRole,
        pub devices: Vec<T::Hash>,
        pub active: bool,
        pub reputation_score: u32,
        pub registration_date: T::BlockNumber,
    }

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    #[scale_info(skip_type_params(T))]
    pub struct Device<T: Config> {
        pub owner: T::AccountId,
        pub device_type: DeviceType,
        pub max_capacity: u32,
        pub active: bool,
        pub registration_date: T::BlockNumber,
    }

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    pub enum DeviceType {
        SolarPanel,
        Battery,
        SmartMeter,
        Other,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn user_profiles)]
    pub type UserProfiles<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        UserProfile<T>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn devices)]
    pub type Devices<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::Hash,
        Device<T>,
        OptionQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        UserRegistered {
            account: T::AccountId,
            role: UserRole,
        },
        DeviceRegistered {
            device_id: T::Hash,
            owner: T::AccountId,
            device_type: DeviceType,
        },
        UserUpdated {
            account: T::AccountId,
        },
        DeviceUpdated {
            device_id: T::Hash,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        UserAlreadyRegistered,
        UserNotFound,
        DeviceAlreadyRegistered,
        DeviceNotFound,
        Unauthorized,
        InvalidRole,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn register_user(
            origin: OriginFor<T>,
            role: UserRole,
        ) -> DispatchResult {
            let account = ensure_signed(origin)?;
            
            ensure!(!UserProfiles::<T>::contains_key(&account), Error::<T>::UserAlreadyRegistered);

            let profile = UserProfile {
                role,
                devices: Vec::new(),
                active: true,
                reputation_score: 100,
                registration_date: <frame_system::Pallet<T>>::block_number(),
            };

            UserProfiles::<T>::insert(&account, profile);

            Self::deposit_event(Event::UserRegistered {
                account,
                role,
            });

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn register_device(
            origin: OriginFor<T>,
            device_type: DeviceType,
            max_capacity: u32,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;
            
            let user = UserProfiles::<T>::get(&owner).ok_or(Error::<T>::UserNotFound)?;
            ensure!(
                matches!(user.role, UserRole::Prosumer | UserRole::GridOperator),
                Error::<T>::Unauthorized
            );

            let device = Device {
                owner: owner.clone(),
                device_type: device_type.clone(),
                max_capacity,
                active: true,
                registration_date: <frame_system::Pallet<T>>::block_number(),
            };

            let device_id = T::Hashing::hash_of(&device);
            ensure!(!Devices::<T>::contains_key(device_id), Error::<T>::DeviceAlreadyRegistered);

            Devices::<T>::insert(device_id, device);
            UserProfiles::<T>::try_mutate(&owner, |profile| -> DispatchResult {
                let profile = profile.as_mut().ok_or(Error::<T>::UserNotFound)?;
                profile.devices.push(device_id);
                Ok(())
            })?;

            Self::deposit_event(Event::DeviceRegistered {
                device_id,
                owner,
                device_type,
            });

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn update_user_role(
            origin: OriginFor<T>,
            account: T::AccountId,
            new_role: UserRole,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            
            let caller_profile = UserProfiles::<T>::get(&who).ok_or(Error::<T>::UserNotFound)?;
            ensure!(
                matches!(caller_profile.role, UserRole::Admin),
                Error::<T>::Unauthorized
            );

            UserProfiles::<T>::try_mutate(&account, |profile| -> DispatchResult {
                let profile = profile.as_mut().ok_or(Error::<T>::UserNotFound)?;
                profile.role = new_role;
                Ok(())
            })?;

            Self::deposit_event(Event::UserUpdated {
                account,
            });

            Ok(())
        }
    }
}
