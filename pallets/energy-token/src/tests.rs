use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn mint_tokens_works() {
    new_test_ext().execute_with(|| {
        let account = 1;
        let amount = 100;

        assert_ok!(EnergyToken::mint_tokens(RuntimeOrigin::signed(account), amount));
        
        assert_eq!(EnergyToken::token_balance(account), amount);
        
        System::assert_last_event(Event::TokensMinted {
            account,
            amount,
        }.into());
    });
}

#[test]
fn transfer_works() {
    new_test_ext().execute_with(|| {
        let from = 1;
        let to = 2;
        let amount = 50;

        // First mint some tokens
        assert_ok!(EnergyToken::mint_tokens(RuntimeOrigin::signed(from), 100));
        
        // Then transfer
        assert_ok!(EnergyToken::transfer(RuntimeOrigin::signed(from), to, amount));
        
        assert_eq!(EnergyToken::token_balance(from), 50);
        assert_eq!(EnergyToken::token_balance(to), amount);
        
        System::assert_last_event(Event::TokensTransferred {
            from,
            to,
            amount,
        }.into());
    });
}

#[test]
fn transfer_fails_with_insufficient_balance() {
    new_test_ext().execute_with(|| {
        let from = 1;
        let to = 2;
        let amount = 100;

        assert_noop!(
            EnergyToken::transfer(RuntimeOrigin::signed(from), to, amount),
            Error::<Test>::InsufficientBalance
        );
    });
}
