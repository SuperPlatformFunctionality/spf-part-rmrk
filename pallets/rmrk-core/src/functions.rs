use codec::Encode;
use sp_core::blake2_256;
use super::*;

impl<T: Config> Pallet<T> {

	pub fn nft_to_account_id(collection_id: CollectionId, nft_id: NftId) -> AccountId {
		let preimage = (b"RmrkNft", collleciton_id, nft_id).encode();
		let hash = blake2_256(&preimage);
		AccountId::from(&hash)
	}

	fn lookup_root(collection_id: CollectionId, nft_id: NftId) -> (T::AccountId, (CollectionId,NftId)) {
		let parent = pallet_unique::Pallet::<T>::owner_of(collection_id, nft_id);
		match AccountPreimage::<T>::get(parent) {
			None => (parent, (collection_id, nft_id)),
			Some((collection_id, nft_id)) => lookup_root_owner(collection_id, nft_id),
		}
	}

	pub fn is_x_descendent_of_y(
		child_collection_id: CollectionId,
		child_nft_id: NftId,
		parent_collection_id: CollectionId,
		parent_nft_id: NftId,
	) -> bool {
		let mut found_child = false;
		if let Some(children) = Children::<T>::get(parent_collection_id, parent_nft_id) {
			for child in children {
				if child == (child_collection_id, child_nft_id) {
					return true
				} else {
					if Pallet::<T>::is_x_descendent_of_y(
						child_collection_id,
						child_nft_id,
						child.0,
						child.1,
					) {
						found_child = true;
					}
				}
			}
		}
		found_child
	}

	pub fn recursive_update_rootowner(
		collection_id: CollectionId,
		nft_id: NftId,
		new_rootowner: T::AccountId,
		max_recursions: u32,
	) -> DispatchResult {
		ensure!(max_recursions > 0, Error::<T>::TooManyRecursions);
		NFTs::<T>::try_mutate_exists(collection_id, nft_id, |nft| -> DispatchResult {
			if let Some(n) = nft {
				n.rootowner = new_rootowner.clone();
			}
			Ok(())
		})?;
		if let Some(children) = Children::<T>::get(collection_id, nft_id) {
			for child in children {
				Pallet::<T>::recursive_update_rootowner(
					child.0,
					child.1,
					new_rootowner.clone(),
					max_recursions - 1,
				)?;
			}
		}
		Ok(())
	}

	pub fn recursive_burn(
		collection_id: CollectionId,
		nft_id: NftId,
		max_recursions: u32,
	) -> DispatchResult {
		ensure!(max_recursions > 0, Error::<T>::TooManyRecursions);
		NFTs::<T>::remove(collection_id, nft_id);
		if let Some(kids) = Children::<T>::take(collection_id, nft_id) {
			for child in kids {
				Pallet::<T>::recursive_burn(child.0, child.1, max_recursions - 1)?;
			}
		}
		Ok(())
	}
}
