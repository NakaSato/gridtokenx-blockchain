use crate::{mock::*, Error, Event, UserRole, DeviceType};
use frame_support::{assert_noop, assert_ok};

#[test]
fn register_user_works() {
    new_test_ext().execute_with(|| {
        let account = 1;
        let role = UserRole::Prosumer;

        assert_ok!(UserRegistry::register_user(
            RuntimeOrigin::signed(account),
            role.clone()
        ));

        let profile = UserRegistry::user_profiles(account).unwrap();
        assert_eq!(profile.role, role);
        assert_eq!(profile.devices.len(), 0);
        assert!(profile.active);

        System::assert_last_event(Event::UserRegistered {
            account,
            role,
        }.into());
    });
}

#[test]
fn register_device_works() {
    new_test_ext().execute_with(|| {
        let account = 1;
        let device_type = DeviceType::SolarPanel;
        let max_capacity = 1000;

        // First register user as prosumer
        assert_ok!(UserRegistry::register_user(
            RuntimeOrigin::signed(account),
            UserRole::Prosumer
        ));

        assert_ok!(UserRegistry::register_device(
            RuntimeOrigin::signed(account),
            device_type.clone(),
            max_capacity
        ));

        let device_id = System::events()
            .iter()
            .find_map(|r| {
                if let Event::DeviceRegistered { device_id, .. } = r.event {
                    Some(device_id)
                } else {
                    None
                }
            })
            .unwrap();

        let device = UserRegistry::devices(device_id).unwrap();
        assert_eq!(device.owner, account);
        assert_eq!(device.device_type, device_type);
        assert_eq!(device.max_capacity, max_capacity);
        assert!(device.active);

        let profile = UserRegistry::user_profiles(account).unwrap();
        assert!(profile.devices.contains(&device_id));
    });
}

#[test]
fn register_device_fails_for_consumer() {
    new_test_ext().execute_with(|| {
        let account = 1;
        
        // Register user as consumer
        assert_ok!(UserRegistry::register_user(
            RuntimeOrigin::signed(account),
            UserRole::Consumer
        ));

        assert_noop!(
            UserRegistry::register_device(
                RuntimeOrigin::signed(account),
                DeviceType::SolarPanel,
                1000
            ),
            Error::<Test>::Unauthorized
        );
    });
}

#[test]
fn update_user_role_works() {
    new_test_ext().execute_with(|| {
        let admin = 1;
        let user = 2;
        let new_role = UserRole::Prosumer;

        // Register admin
        assert_ok!(UserRegistry::register_user(
            RuntimeOrigin::signed(admin),
            UserRole::Admin
        ));

        // Register user
        assert_ok!(UserRegistry::register_user(
            RuntimeOrigin::signed(user),
            UserRole::Consumer
        ));

        // Update user's role
        assert_ok!(UserRegistry::update_user_role(
            RuntimeOrigin::signed(admin),
            user,
            new_role.clone()
        ));

        let profile = UserRegistry::user_profiles(user).unwrap();
        assert_eq!(profile.role, new_role);
    });
}
