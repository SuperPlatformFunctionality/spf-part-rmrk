use frame_support::{assert_noop, assert_ok, error::BadOrigin};
// use sp_runtime::AccountId32;
use sp_runtime::Permill;
// use crate::types::ClassType;

use super::*;
use mock::{Event as MockEvent, *};
use pallet_uniques as UNQ;
use sp_std::{convert::TryInto, vec::Vec};

type RMRKCore = Pallet<Test>;

/// Turns a string into a BoundedVec
fn stb(s: &str) -> BoundedVec<u8, ValueLimit> {
	s.as_bytes().to_vec().try_into().unwrap()
}

/// Turns a string into a BoundedVec
fn stbk(s: &str) -> BoundedVec<u8, KeyLimit> {
	s.as_bytes().to_vec().try_into().unwrap()
}

/// Turns a string into a Vec
fn stv(s: &str) -> Vec<u8> {
	s.as_bytes().to_vec()
}

macro_rules! bvec {
	($( $x:tt )*) => {
		vec![$( $x )*].try_into().unwrap()
	}
}

/// Shortcut for a test collection creation (Alice is issue, max NFTs is 5)
fn basic_collection() -> DispatchResult {
	RMRKCore::create_collection(Origin::signed(ALICE), bvec![0u8; 20], Some(5), bvec![0u8; 15])
}

/// Shortcut for a basic mint (Alice owner, Collection ID 0, Royalty 1.525)
fn basic_mint() -> DispatchResult {
	RMRKCore::mint_nft(
		Origin::signed(ALICE),
		ALICE,
		COLLECTION_ID_0,
		Some(ALICE),
		Some(Permill::from_float(1.525)),
		bvec![0u8; 20]
	)
}

// Tests ordered as follows:
// Collection: create, lock, destroy, changeissuer
// NFT: mint, send, burn
// Resource: create, add, accept
// Property: set
// Priority: set

/// Collection: Basic collection tests (RMRK2.0 spec: CREATE)
#[test]
fn create_collection_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Creating collection should trigger CollectionCreated event
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::CollectionCreated {
			issuer: ALICE,
			collection_id: 0,
		}));
		// Reassign CollectionIndex to max value
		CollectionIndex::<Test>::mutate(|id| *id = CollectionId::max_value());
		// Creating collection above max_value of CollectionId (4294967295) should fail
		assert_noop!(
			RMRKCore::create_collection(
				Origin::signed(ALICE),
				bvec![0u8; 20],
				None,
				bvec![0u8; 15],
			),
			Error::<Test>::NoAvailableCollectionId
		);	
	});
}

/// Collection: Locking collection tests (RMRK2.0 spec: LOCK)
#[test]
fn lock_collection_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection (has 5 max)
		assert_ok!(basic_collection());
		// Mint 4 NFTs
		for _ in 0..4 {
			assert_ok!(basic_mint());
		}
		// Lock collection
		assert_ok!(RMRKCore::lock_collection(Origin::signed(ALICE), 0));
		// Locking collection should trigger CollectionLocked event
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::CollectionLocked {
			issuer: ALICE,
			collection_id: 0,
		}));
		// Attempt to mint in a locked collection should fail
		assert_noop!(
			basic_mint(),
			Error::<Test>::CollectionFullOrLocked
		);
		// Burn an NFT
		assert_ok!(RMRKCore::burn_nft(Origin::signed(ALICE), COLLECTION_ID_0, NFT_ID_0));
		// Should now have only three NFTS in collection
		assert_eq!(RMRKCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 3);
		// Still we should be unable to mint another NFT
		assert_noop!(
			basic_mint(),
			Error::<Test>::CollectionFullOrLocked
		);
	});
}

/// Collection: Destroy collection tests (RMRK2.0 spec: doesn't exist)
#[test]
fn destroy_collection_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection (has 5 max)
		assert_ok!(basic_collection());
		// Mint an NFT
		assert_ok!(basic_mint());
		// Non-empty collection should not be able to be destroyed
		assert_noop!(
			RMRKCore::destroy_collection(Origin::signed(ALICE), COLLECTION_ID_0),
			Error::<Test>::CollectionNotEmpty
		);
		// Burn the single NFT in collection
		assert_ok!(RMRKCore::burn_nft(Origin::signed(ALICE), COLLECTION_ID_0, NFT_ID_0));
		// Empty collection can be destroyed
		assert_ok!(RMRKCore::destroy_collection(Origin::signed(ALICE), COLLECTION_ID_0));
		// Destroy event is triggered by successful destroy_collection
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::CollectionDestroyed {
			issuer: ALICE,
			collection_id: COLLECTION_ID_0,
		}));
	});
}


//CHANGEISSUER (collection)
#[test]
fn change_issuer_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(basic_collection());
		assert_ok!(RMRKCore::change_issuer(Origin::signed(ALICE), 0, BOB));
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::IssuerChanged {
			old_issuer: ALICE,
			new_issuer: BOB,
			collection_id: 0,
		}));
		assert_eq!(RMRKCore::collections(0).unwrap().issuer, BOB);
	});
}

//MINT (nft)
#[test]
fn mint_nft_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(basic_collection());
		assert_ok!(basic_mint());
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::NftMinted {
			owner: ALICE,
			collection_id: 0,
			nft_id: 0,
		}));
		assert_eq!(RMRKCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 1);
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,
			COLLECTION_ID_0,
			Some(ALICE),
			Some(Permill::from_float(20.525)),
			bvec![0u8; 20]
		));
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(BOB),
			BOB,
			COLLECTION_ID_0,
			Some(CHARLIE),
			Some(Permill::from_float(20.525)),
			bvec![0u8; 20]
		));
		assert_eq!(RMRKCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 3);
		assert_noop!(
			RMRKCore::mint_nft(
				Origin::signed(ALICE),
				ALICE,
				NOT_EXISTING_CLASS_ID,
				Some(CHARLIE),
				Some(Permill::from_float(20.525)),
				bvec![0u8; 20]
			),
			Error::<Test>::CollectionUnknown
		);
	});
}

//MINT (nft)
#[test]
fn mint_collection_max_logic_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(RMRKCore::create_collection(
			Origin::signed(ALICE),
			bvec![0u8; 20],
			Some(1),
			bvec![0u8; 15]
		));
		assert_ok!(basic_mint());
		assert_ok!(RMRKCore::burn_nft(Origin::signed(ALICE), COLLECTION_ID_0, 0));
		assert_noop!(
			basic_mint(),
			Error::<Test>::CollectionFullOrLocked
		);
	});
}

//MINT (nft)
#[test]
fn mint_beyond_collection_max_fails() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(basic_collection());
		for _ in 0..5 {
			assert_ok!(basic_mint());
		}
		assert_noop!(
			basic_mint(),
			Error::<Test>::CollectionFullOrLocked
		);
	});
}

//SEND (nft)
#[test]
fn send_nft_to_minted_nft_works() {
	ExtBuilder::default().build().execute_with(|| {
		// let nft_metadata = bvec![0u8; 20];
		assert_ok!(basic_collection());
		// Alice mints NFT (0, 0) [will be the parent]
		assert_ok!(basic_mint());
		// Alice mints NFT (0, 1) [will be the child]
		assert_ok!(basic_mint());
		// Alice mints NFT (0, 2) [will be the child]
		assert_ok!(basic_mint());

		// Alice sends NFT (0, 0) [parent] to Bob
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			0,
			AccountIdOrCollectionNftTuple::AccountId(BOB),
		));
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::NFTSent {
			sender: ALICE,
			recipient: AccountIdOrCollectionNftTuple::AccountId(BOB),
			collection_id: 0,
			nft_id: 0,
		}));
		// Alice sends NFT (0, 1) [child] to NFT (0, 0) [parent]
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			1,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
		));
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::NFTSent {
			sender: ALICE,
			recipient: AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
			collection_id: 0,
			nft_id: 1,
		}));
		// Alice sends NFT (0, 2) [child] to NFT (0, 0) [parent]
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			2,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 1),
		));
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::NFTSent {
			sender: ALICE,
			recipient: AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 1),
			collection_id: 0,
			nft_id: 2,
		}));
		// Check that NFT (0,1) [child] is owned by NFT (0,0) [parent]
		assert_eq!(
			UNQ::Pallet::<Test>::owner(0, 1),
			Some(RMRKCore::nft_to_account_id(0, 0)),
		);

		// Check NFT (0,0) has NFT (0,1) in Children StorageMap
		assert_eq!(RMRKCore::children((0, 0)), vec![(0,1)]);

		// Error if trying to assign send a nft to self nft
		assert_noop!(
			RMRKCore::send(
				Origin::signed(BOB),
				0,
				0,
				AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0)
			),
			Error::<Test>::CannotSendToDescendentOrSelf
		);

		// Error if trying to assign send a nft creating circular reference
		assert_noop!(
			RMRKCore::send(
				Origin::signed(BOB),
				0,
				0,
				AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 2)
			),
			Error::<Test>::CannotSendToDescendentOrSelf
		);

		// Check that Bob now root-owns NFT (0, 1) [child] since he wasn't originally rootowner
		if let Ok((root_owner, _)) = RMRKCore::lookup_root_owner(0, 1) {
			assert_eq!(root_owner, BOB);
		}

		// Error if sender doesn't root-own sending NFT
		assert_noop!(
			RMRKCore::send(
				Origin::signed(CHARLIE),
				0,
				0,
				AccountIdOrCollectionNftTuple::AccountId(BOB)
			),
			Error::<Test>::NoPermission
		);

		// Error if sending NFT doesn't exist
		assert_noop!(
			RMRKCore::send(
				Origin::signed(ALICE),
				666,
				666,
				AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0)
			),
			Error::<Test>::NoAvailableNftId
		);

		// BOB can send back child NFT to ALICE
		assert_ok!(RMRKCore::send(
			Origin::signed(BOB),
			0,
			1,
			AccountIdOrCollectionNftTuple::AccountId(ALICE)
		));

		// Error if recipient is NFT and that NFT doesn't exist
		assert_noop!(
			RMRKCore::send(
				Origin::signed(ALICE),
				0,
				1,
				AccountIdOrCollectionNftTuple::CollectionAndNftTuple(666, 666)
			),
			Error::<Test>::NoAvailableNftId
		);
	});
}

//SEND (nft)
#[test]
fn send_two_nfts_to_same_nft_creates_two_children() {
	ExtBuilder::default().build().execute_with(|| {
		// let nft_metadata = bvec![0u8; 20];
		assert_ok!(basic_collection());
		// Alice mints NFT (0, 0)
		assert_ok!(basic_mint());
		// Alice mints NFT (0, 1)
		assert_ok!(basic_mint());
		// Alice mints NFT (0, 2)
		assert_ok!(basic_mint());

		// Alice sends NFT (0, 1) to NFT (0, 0)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			1,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
		));

		// Check NFT (0,0) has NFT (0,1) in Children StorageMap
		assert_eq!(RMRKCore::children((0, 0)), vec![(0,1)]);

		// Alice sends NFT (0, 2) to NFT (0, 0)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			2,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
		));

		// Check NFT (0,0) has NFT (0,1) & (0,2) in Children StorageMap
		assert_eq!(RMRKCore::children((0, 0)), vec![(0,1), (0,2)]);
	});
}

//SEND (nft)
#[test]
fn send_nft_removes_existing_parent() {
	ExtBuilder::default().build().execute_with(|| {
		// let nft_metadata = bvec![0u8; 20];
		assert_ok!(basic_collection());
		// Alice mints NFT (0, 0)
		assert_ok!(basic_mint());
		// Alice mints NFT (0, 1)
		assert_ok!(basic_mint());
		// Alice mints NFT (0, 2)
		assert_ok!(basic_mint());
		// Alice mints NFT (0, 3)
		assert_ok!(basic_mint());

		// Alice sends NFT (0, 1) to NFT (0, 0)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			1,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
		));
		// Alice sends NFT (0, 2) to NFT (0, 0)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			2,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
		));

		// NFT (0, 0) is parent of NFT (0, 1)
		assert_eq!(RMRKCore::children((0, 0)), vec![(0, 1), (0, 2)]);

		// Alice sends NFT (0, 1) to NFT (0, 2)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			1,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 2),
		));

		// NFT (0, 0) is not parent of NFT (0, 1)
		assert_eq!(RMRKCore::children((0, 0)), vec![(0, 2)]);
	});
}

//SEND (nft)
#[test]
fn send_to_grandchild_fails() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(basic_collection());
		// Alice mints (0, 0)
		assert_ok!(basic_mint());
		// Alice mints (0, 1)
		assert_ok!(basic_mint());
		// Alice mints (0, 2)
		assert_ok!(basic_mint());
		// Alice sends NFT (0, 1) to NFT (0, 0)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			1,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
		));
		// Alice sends NFT (0, 2) to NFT (0, 1)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			2,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 1),
		));

		// Alice sends (0, 0) to (0, 2)
		assert_noop!(
			RMRKCore::send(
				Origin::signed(ALICE),
				0,
				0,
				AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 2),
			),
			Error::<Test>::CannotSendToDescendentOrSelf
		);
	});
}

//BURN (nft)
#[test]
fn burn_nft_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(basic_collection());
		assert_eq!(RMRKCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 0);
		assert_ok!(basic_mint());

		assert_noop!(RMRKCore::burn_nft(Origin::signed(BOB), COLLECTION_ID_0, NFT_ID_0), Error::<Test>::NoPermission);
		assert_eq!(RMRKCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 1);
    assert_ok!(RMRKCore::burn_nft(Origin::signed(ALICE), COLLECTION_ID_0, NFT_ID_0));
		assert_eq!(RMRKCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 0);
    assert_noop!(RMRKCore::burn_nft(Origin::signed(ALICE), COLLECTION_ID_0, NFT_ID_0), Error::<Test>::NoAvailableNftId);
		
		assert_eq!(RMRKCore::nfts(COLLECTION_ID_0, NFT_ID_0).is_none(), true);
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::NFTBurned {
			owner: ALICE,
			nft_id: 0,
		}));
	});
}

//BURN (nft)
#[test]
fn burn_nft_with_great_grandchildren_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(basic_collection());
		// Alice mints (0, 0)
		assert_ok!(basic_mint());
		// Alice mints (0, 1)
		assert_ok!(basic_mint());
		// Alice mints (0, 2)
		assert_ok!(basic_mint());
		// Alice mints (0, 3)
		assert_ok!(basic_mint());
		// Alice sends NFT (0, 1) to NFT (0, 0)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			1,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
		));
		// Alice sends NFT (0, 2) to NFT (0, 1)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			2,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 1),
		));
		// Alice sends NFT (0, 3) to NFT (0, 2)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			3,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 2),
		));
		// Child is alive
		assert_eq!(RMRKCore::nfts(COLLECTION_ID_0, 3).is_some(), true);
		// Burn great-grandfather
		assert_ok!(RMRKCore::burn_nft(Origin::signed(ALICE), COLLECTION_ID_0, NFT_ID_0));
		// Child is dead
		assert!(RMRKCore::nfts(COLLECTION_ID_0, 3).is_none())
	});
}

//RESADD (resource)
#[test]
fn create_resource_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Creating a resource for non-existent NFT fails
		assert_noop!(
			RMRKCore::add_resource(
				Origin::signed(ALICE),
				0,
				0,
				Some(bvec![0u8; 20]),
				Some(bvec![0u8; 20]),
				Some(bvec![0u8; 20]),
				Some(bvec![0u8; 20]),
				Some(bvec![0u8; 20]),
				Some(bvec![0u8; 20]),
			),
			Error::<Test>::NoAvailableNftId
		);
		// Create collection and NFT
		assert_ok!(basic_collection());
		assert_ok!(basic_mint());
		// Add resource works
		assert_ok!(RMRKCore::add_resource(
			Origin::signed(ALICE),
			0,
			0,
			Some(bvec![0u8; 20]),
			Some(bvec![0u8; 20]),
			Some(bvec![0u8; 20]),
			Some(bvec![0u8; 20]),
			Some(bvec![0u8; 20]),
			Some(bvec![0u8; 20]),
		));
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::ResourceAdded {
			nft_id: 0,
			resource_id: 0,
		}));
		// Since ALICE rootowns NFT (0, 0), pending should be false
		assert_eq!(RMRKCore::resources((0, 0, 0)).unwrap().pending, false);
	});
}

//RESADD (resource)
#[test]
fn create_empty_resource_fails() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(basic_collection());
		assert_ok!(basic_mint());
		assert_noop!(
			RMRKCore::add_resource(
				Origin::signed(ALICE),
				COLLECTION_ID_0,
				NFT_ID_0,
				None,
				None,
				None,
				None,
				None,
				None
			),
			Error::<Test>::EmptyResource
		);
	});
}

//RESADD (resource)
#[test]
fn add_resource_pending_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(basic_collection());
		assert_ok!(basic_mint());
		assert_ok!(RMRKCore::add_resource(
			Origin::signed(BOB),
			0,
			0,
			Some(bvec![0u8; 20]),
			Some(bvec![0u8; 20]),
			Some(bvec![0u8; 20]),
			Some(bvec![0u8; 20]),
			Some(bvec![0u8; 20]),
			Some(bvec![0u8; 20]),
		));
		assert_eq!(RMRKCore::resources((0, 0, 0)).unwrap().pending, true);
	});
}

//ACCEPT (resource)
#[test]
fn accept_resource_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(basic_collection());
		assert_ok!(basic_mint());
		assert_ok!(RMRKCore::add_resource(
			Origin::signed(BOB),
			0,
			0,
			Some(bvec![0u8; 20]),
			Some(bvec![0u8; 20]),
			Some(bvec![0u8; 20]),
			Some(bvec![0u8; 20]),
			Some(bvec![0u8; 20]),
			Some(bvec![0u8; 20]),
		));
		assert_eq!(RMRKCore::resources((0, 0, 0)).unwrap().pending, true);
		// Bob can't accept Alice's NFT's resource
		assert_noop!(RMRKCore::accept(Origin::signed(BOB), 0, 0, 0), Error::<Test>::NoPermission);
		// Alice can accept her own NFT's resource
		assert_ok!(RMRKCore::accept(Origin::signed(ALICE), 0, 0, 0));
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::ResourceAccepted {
			nft_id: 0,
			resource_id: 0,
		}));
		// Resource should now be pending = false
		assert_eq!(RMRKCore::resources((0, 0, 0)).unwrap().pending, false);
	});
}

//SETPROPERTY (property)
#[test]
fn set_property_works() {
	ExtBuilder::default().build().execute_with(|| {
		let key = stbk("test-key");
		let value = stb("test-value");
		assert_ok!(basic_collection());
		assert_ok!(basic_mint());
		assert_ok!(RMRKCore::set_property(
			Origin::signed(ALICE),
			0,
			Some(0),
			key.clone(),
			value.clone()
		));
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::PropertySet {
			collection_id: 0,
			maybe_nft_id: Some(0),
			key: key.clone(),
			value: value.clone(),
		}));
		// Error when set_property with BOB
		assert_noop!(RMRKCore::set_property(
			Origin::signed(BOB),
			0,
			Some(0),
			key.clone(),
			value.clone()
			),
			Error::<Test>::NoPermission
		);
		assert_eq!(RMRKCore::properties((0, Some(0), key)).unwrap(), value);
	});
}

//SETPRIORITY (priority)
#[test]
fn set_priority_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(basic_collection());
		assert_ok!(basic_mint());
		assert_ok!(RMRKCore::set_priority(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NFT_ID_0,
			vec![stv("hello"), stv("world")]
		));
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::PrioritySet {
			collection_id: 0,
			nft_id: 0,
		}));
		assert_eq!(
			RMRKCore::priorities(COLLECTION_ID_0, NFT_ID_0).unwrap(),
			vec![stv("hello"), stv("world")]
		);
	});
}



// #[test]
// TODO fn cannot send to its own descendent?  this should be easy enough to check
// TODO fn cannot send to its own grandparent?  this seems difficult to check without implementing a
// new Parent storage struct
