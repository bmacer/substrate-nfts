use frame_support::{assert_noop, assert_ok, error::BadOrigin};

// use crate::types::ClassType;

use super::*;
use mock::*;
use pallet_uniques as UNQ;

type NFTPallet = Pallet<Test>;

/// Turns a string into a BoundedVec
fn stb(s: &str) -> BoundedVec<u8, ValueLimit> {
	s.as_bytes().to_vec().try_into().unwrap()
}

/// Turns a string into a Vec
fn stv(s: &str) -> Vec<u8> {
	s.as_bytes().to_vec()
}

#[test]
fn create_collection_works() {
	ExtBuilder::default().build().execute_with(|| {
		let metadata = stv("testing");
		assert_ok!(NFTPallet::mint_collection(Origin::signed(ALICE), metadata));
	});
}

#[test]
fn mint_nft_works() {
	ExtBuilder::default().build().execute_with(|| {
		let collection_metadata = stv("testing");
		let nft_metadata = stv("testing");
		assert_ok!(NFTPallet::mint_collection(Origin::signed(ALICE), collection_metadata));
		assert_ok!(NFTPallet::mint_nft(
			Origin::signed(ALICE),
			0,
			Some(ALICE),
			None,
			Some(nft_metadata)
		));
	});
}
