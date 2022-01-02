use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;
use frame_system::ensure_signed;

#[test]
fn create_kitty_test() {
	new_test_ext().execute_with(|| {
		// 首先测试Event::Created事件
		assert_ok!(Balances::set_balance(Origin::root(), 1, 1000, 0));
		assert_eq!(Balances::free_balance(&1), 1000);
		assert_ok!(KittyModule::create_kitty( Origin::signed(1)));

		//测试Error::NotEnoughBalance
		assert_noop!(KittyModule::create_kitty( Origin::signed(2)), Error::<Test>::NotEnoughBalance);

		// assert_eq!(Balances::free_balance(Origin::signed(1)), 1000);

	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		// assert_noop!(TemplateModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
	});
}
