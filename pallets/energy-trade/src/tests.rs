use crate::{mock::*, Error, Event, OrderStatus};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_sell_order_works() {
    new_test_ext().execute_with(|| {
        let seller = 1;
        let amount = 100;
        let price = 10;

        assert_ok!(EnergyTrade::create_sell_order(
            RuntimeOrigin::signed(seller),
            amount,
            price
        ));

        let order_id = System::events()
            .iter()
            .find_map(|r| {
                if let Event::TradeOrderCreated { order_id, .. } = r.event {
                    Some(order_id)
                } else {
                    None
                }
            })
            .unwrap();

        let order = EnergyTrade::trade_orders(order_id).unwrap();
        assert_eq!(order.seller, seller);
        assert_eq!(order.energy_amount, amount);
        assert_eq!(order.price_per_unit, price);
        assert_eq!(order.status, OrderStatus::Open);
    });
}

#[test]
fn match_order_works() {
    new_test_ext().execute_with(|| {
        let seller = 1;
        let buyer = 2;
        let amount = 100;
        let price = 10;

        assert_ok!(EnergyTrade::create_sell_order(
            RuntimeOrigin::signed(seller),
            amount,
            price
        ));

        let order_id = System::events()
            .iter()
            .find_map(|r| {
                if let Event::TradeOrderCreated { order_id, .. } = r.event {
                    Some(order_id)
                } else {
                    None
                }
            })
            .unwrap();

        assert_ok!(EnergyTrade::match_order(
            RuntimeOrigin::signed(buyer),
            order_id
        ));

        let order = EnergyTrade::trade_orders(order_id).unwrap();
        assert_eq!(order.status, OrderStatus::Matched);
        assert_eq!(order.buyer, Some(buyer));
    });
}

#[test]
fn complete_order_works() {
    new_test_ext().execute_with(|| {
        let seller = 1;
        let buyer = 2;
        let amount = 100;
        let price = 10;

        assert_ok!(EnergyTrade::create_sell_order(
            RuntimeOrigin::signed(seller),
            amount,
            price
        ));

        let order_id = System::events()
            .iter()
            .find_map(|r| {
                if let Event::TradeOrderCreated { order_id, .. } = r.event {
                    Some(order_id)
                } else {
                    None
                }
            })
            .unwrap();

        assert_ok!(EnergyTrade::match_order(
            RuntimeOrigin::signed(buyer),
            order_id
        ));

        assert_ok!(EnergyTrade::complete_order(
            RuntimeOrigin::signed(seller),
            order_id
        ));

        let order = EnergyTrade::trade_orders(order_id).unwrap();
        assert_eq!(order.status, OrderStatus::Completed);
    });
}
