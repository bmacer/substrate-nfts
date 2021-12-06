use frame_support::{assert_noop, assert_ok, error::BadOrigin};

use crate::types::ClassType;

use super::*;
use mock::*;
use pallet_uniques as UNQ;

type NFTPallet = Pallet<Test>;

#[test]
fn create_class_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			NFTPallet::create_class(
				Origin::signed(ALICE),
				ClassType::PoolShare,
				b"metadata".to_vec()
			),
			Error::<Test>::NotPermitted
		);
		assert_ok!(NFTPallet::create_class(
			Origin::root(),
			ClassType::PoolShare,
			b"metadata".to_vec()
		));
		assert_ok!(NFTPallet::create_class(
			Origin::signed(ALICE),
			ClassType::Marketplace,
			b"metadata".to_vec()
		));
		assert_noop!(
			NFTPallet::create_class(
				Origin::signed(ALICE),
				ClassType::Marketplace,
				vec![0; <Test as UNQ::Config>::StringLimit::get() as usize + 1]
			),
			Error::<Test>::TooLong
		);
		NextClassId::<Test>::mutate(|id| *id = <Test as UNQ::Config>::ClassId::max_value());
		assert_noop!(
			NFTPallet::create_class(
				Origin::signed(ALICE),
				ClassType::Marketplace,
				b"metadata".to_vec()
			),
			Error::<Test>::NoAvailableClassId
		);
	})
}

#[test]
fn mint_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NFTPallet::create_class(
			Origin::signed(ALICE),
			ClassType::Marketplace,
			b"metadata".to_vec()
		));
		assert_ok!(NFTPallet::create_class(
			Origin::root(),
			ClassType::PoolShare,
			b"metadata".to_vec()
		));
		assert_ok!(NFTPallet::mint_for_marketplace(
			Origin::signed(ALICE),
			CLASS_ID_0,
			ALICE,
			20,
			b"metadata".to_vec(),
		));
		assert_ok!(NFTPallet::mint_for_marketplace(
			Origin::signed(BOB),
			CLASS_ID_0,
			CHARLIE,
			20,
			b"metadata".to_vec(),
		));

		assert_ok!(NFTPallet::create_class(
			Origin::root(),
			ClassType::PoolShare,
			b"metadata".to_vec()
		));
		assert_noop!(
			NFTPallet::mint_for_marketplace(
				Origin::signed(ALICE),
				NOT_EXISTING_CLASS_ID,
				CHARLIE,
				20,
				b"metadata".to_vec(),
			),
			Error::<Test>::ClassUnknown
		);
		assert_noop!(
			NFTPallet::mint_for_marketplace(
				Origin::signed(ALICE),
				CLASS_ID_0,
				CHARLIE,
				123,
				b"metadata".to_vec(),
			),
			Error::<Test>::NotInRange
		);
		assert_noop!(
			NFTPallet::mint_for_liquidity_mining(
				Origin::signed(ALICE),
				ALICE,
				CLASS_ID_1,
				123,
				654,
				b"metadata".to_vec(),
			),
			BadOrigin
		);
		assert_ok!(NFTPallet::mint_for_liquidity_mining(
			Origin::root(),
			ALICE,
			CLASS_ID_1,
			123,
			654,
			b"metadata".to_vec(),
		));
		NextInstanceId::<Test>::mutate(CLASS_ID_0, |id| {
			*id = <Test as UNQ::Config>::InstanceId::max_value()
		});
		assert_noop!(
			NFTPallet::mint_for_marketplace(
				Origin::signed(ALICE),
				CLASS_ID_0,
				CHARLIE,
				20,
				b"metadata".to_vec(),
			),
			Error::<Test>::NoAvailableInstanceId
		);

		assert_noop!(
			NFTPallet::destroy_class(Origin::signed(ALICE), NOT_EXISTING_CLASS_ID),
			Error::<Test>::ClassUnknown
		);
	});
}

#[test]
fn transfer_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NFTPallet::create_class(
			Origin::signed(ALICE),
			ClassType::Marketplace,
			b"metadata".to_vec()
		));
		assert_ok!(NFTPallet::create_class(
			Origin::root(),
			ClassType::PoolShare,
			b"metadata".to_vec()
		));
		assert_ok!(NFTPallet::mint_for_liquidity_mining(
			Origin::root(),
			ALICE,
			CLASS_ID_1,
			123,
			654,
			b"metadata".to_vec(),
		));
		assert_eq!(Balances::free_balance(ALICE), 10_000 * BSX);
		assert_ok!(NFTPallet::mint_for_marketplace(
			Origin::signed(ALICE),
			CLASS_ID_0,
			CHARLIE,
			20,
			b"metadata".to_vec(),
		));
		assert_eq!(Balances::free_balance(ALICE), 9_900 * BSX);
		assert_ok!(NFTPallet::transfer(Origin::signed(ALICE), CLASS_ID_1, TOKEN_ID_0, BOB));

		assert_noop!(
			NFTPallet::transfer(Origin::signed(CHARLIE), CLASS_ID_0, TOKEN_ID_0, ALICE),
			Error::<Test>::NotPermitted
		);
		assert_eq!(Balances::free_balance(BOB), 15_000 * BSX);
		assert_ok!(NFTPallet::transfer(Origin::signed(ALICE), CLASS_ID_0, TOKEN_ID_0, BOB));
		assert_eq!(Balances::free_balance(BOB), 14_900 * BSX);
		assert_ok!(NFTPallet::transfer(Origin::signed(BOB), CLASS_ID_0, TOKEN_ID_0, CHARLIE));
		assert_eq!(Balances::free_balance(ALICE), 10_000 * BSX);
		assert_eq!(Balances::free_balance(BOB), 15_000 * BSX);
		assert_eq!(Balances::free_balance(CHARLIE), 149_900 * BSX);
	});
}

#[test]
fn burn_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NFTPallet::create_class(
			Origin::signed(ALICE),
			ClassType::Marketplace,
			b"metadata".to_vec()
		));
		assert_ok!(NFTPallet::create_class(
			Origin::root(),
			ClassType::PoolShare,
			b"metadata".to_vec()
		));
		assert_ok!(NFTPallet::mint_for_liquidity_mining(
			Origin::root(),
			ALICE,
			CLASS_ID_1,
			123,
			654,
			b"metadata".to_vec(),
		));
		assert_ok!(NFTPallet::mint_for_marketplace(
			Origin::signed(ALICE),
			CLASS_ID_0,
			ALICE,
			20,
			b"metadata".to_vec(),
		));

		assert_noop!(
			NFTPallet::burn(Origin::signed(BOB), CLASS_ID_0, TOKEN_ID_0),
			Error::<Test>::NotPermitted
		);
		assert_noop!(
			NFTPallet::burn(Origin::signed(BOB), CLASS_ID_1, TOKEN_ID_0),
			Error::<Test>::NotPermitted
		);

		assert_ok!(NFTPallet::burn(Origin::signed(ALICE), CLASS_ID_0, TOKEN_ID_0,));
		assert_ok!(NFTPallet::burn(Origin::signed(ALICE), CLASS_ID_1, TOKEN_ID_0,));
	});
}

#[test]
fn destroy_class_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NFTPallet::create_class(
			Origin::signed(ALICE),
			ClassType::Marketplace,
			b"metadata".to_vec()
		));
		assert_ok!(NFTPallet::create_class(
			Origin::root(),
			ClassType::PoolShare,
			b"metadata".to_vec()
		));
		assert_ok!(NFTPallet::mint_for_liquidity_mining(
			Origin::root(),
			ALICE,
			CLASS_ID_1,
			123,
			654,
			b"metadata".to_vec(),
		));
		assert_ok!(NFTPallet::mint_for_marketplace(
			Origin::signed(ALICE),
			CLASS_ID_0,
			ALICE,
			20,
			b"metadata".to_vec(),
		));

		assert_noop!(
			NFTPallet::destroy_class(Origin::signed(ALICE), CLASS_ID_0),
			Error::<Test>::TokenClassNotEmpty
		);

		assert_ok!(NFTPallet::burn(Origin::signed(ALICE), CLASS_ID_0, TOKEN_ID_0,));
		assert_ok!(NFTPallet::destroy_class(Origin::signed(ALICE), CLASS_ID_0));
		assert_ok!(NFTPallet::burn(Origin::signed(ALICE), CLASS_ID_1, TOKEN_ID_0,));
		assert_noop!(
			NFTPallet::destroy_class(Origin::signed(ALICE), CLASS_ID_1),
			Error::<Test>::NotPermitted
		);
		assert_ok!(NFTPallet::destroy_class(Origin::root(), CLASS_ID_1));
		assert_noop!(
			NFTPallet::destroy_class(Origin::signed(ALICE), CLASS_ID_0),
			Error::<Test>::ClassUnknown
		);
	});
}
