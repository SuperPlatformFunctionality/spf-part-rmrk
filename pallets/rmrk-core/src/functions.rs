use super::*;
use codec::{Codec, Decode, Encode};
use sp_runtime::traits::TrailingZeroInput;

// Randomness to generate NFT virtual accounts
pub const SALT_RMRK_NFT: &[u8; 8] = b"RmrkNft/";

impl<T: Config> Collection<StringLimitOf<T>, T::AccountId> for Pallet<T> {
	fn issuer(collection_id: CollectionId) -> Option<T::AccountId> {
		None
	}
	fn collection_create(
		issuer: T::AccountId,
		metadata: StringLimitOf<T>,
		max: u32,
		symbol: StringLimitOf<T>,
	) -> Result<CollectionId, DispatchError> {
		let collection = CollectionInfo { issuer: issuer.clone(), metadata, max, symbol };
		let collection_id =
			<CollectionIndex<T>>::try_mutate(|n| -> Result<CollectionId, DispatchError> {
				let id = *n;
				ensure!(id != CollectionId::max_value(), Error::<T>::NoAvailableCollectionId);
				*n += 1;
				Ok(id)
			})?;
		Collections::<T>::insert(collection_id, collection);
		Ok(collection_id)
	}

	fn collection_burn(issuer: T::AccountId, collection_id: CollectionId) -> DispatchResult {
		ensure!(
			NFTs::<T>::iter_prefix_values(collection_id).count() == 0,
			Error::<T>::CollectionNotEmpty
		);
		Collections::<T>::remove(collection_id);
		Ok(())
	}

	fn collection_change_issuer(
		collection_id: CollectionId,
		new_issuer: T::AccountId,
	) -> Result<(T::AccountId, CollectionId), DispatchError> {
		ensure!(Collections::<T>::contains_key(collection_id), Error::<T>::NoAvailableCollectionId);

		Collections::<T>::try_mutate_exists(collection_id, |collection| -> DispatchResult {
			if let Some(col) = collection {
				col.issuer = new_issuer.clone();
			}
			Ok(())
		})?;

		Ok((new_issuer, collection_id))
	}

	fn collection_lock(collection_id: CollectionId) -> Result<CollectionId, DispatchError> {
		Collections::<T>::try_mutate_exists(collection_id, |collection| -> DispatchResult {
			let collection = collection.as_mut().ok_or(Error::<T>::CollectionUnknown)?;
			let currently_minted = NFTs::<T>::iter_prefix_values(collection_id).count();
			collection.max = currently_minted.try_into().unwrap();
			Ok(())
		})?;
		Ok(collection_id)
	}
}

impl<T: Config> Nft<T::AccountId, StringLimitOf<T>> for Pallet<T>
where
	T: pallet_uniques::Config<ClassId = CollectionId, InstanceId = NftId>,
{
	type MaxRecursions = T::MaxRecursions;

	fn nft_mint(
		sender: T::AccountId,
		owner: T::AccountId,
		collection_id: CollectionId,
		recipient: Option<T::AccountId>,
		royalty: Option<Permill>,
		metadata: StringLimitOf<T>,
	) -> sp_std::result::Result<(CollectionId, NftId), DispatchError> {
		let nft_id = Self::get_next_nft_id(collection_id)?;
		let collection = Self::collections(collection_id).ok_or(Error::<T>::CollectionUnknown)?;
		let max: u32 = collection.max.try_into().unwrap();

		// Prevent minting when next NFT id is greater than the collection max.
		ensure!(
			nft_id < max.try_into().unwrap() || max == max - max,
			Error::<T>::CollectionFullOrLocked
		);

		let recipient = recipient.unwrap_or(owner.clone());
		let royalty = royalty.unwrap_or(Permill::default());

		let owner_as_maybe_account = AccountIdOrCollectionNftTuple::AccountId(owner.clone());

		let nft =
			NftInfo { owner: owner_as_maybe_account, recipient, royalty, metadata };

		NFTs::<T>::insert(collection_id, nft_id, nft);
		NftsByOwner::<T>::append(owner, (collection_id, nft_id));

		Ok((collection_id, nft_id))
	}

	fn nft_burn(
		collection_id: CollectionId,
		nft_id: NftId,
		max_recursions: u32,
	) -> sp_std::result::Result<(CollectionId, NftId), DispatchError> {
		ensure!(max_recursions > 0, Error::<T>::TooManyRecursions);
		NFTs::<T>::remove(collection_id, nft_id);
		if let kids = Children::<T>::take((collection_id, nft_id)) {
			for (child_collection_id, child_nft_id) in kids {
				// Remove child from Children StorageMap
				Pallet::<T>::remove_child(
					(collection_id, nft_id),
					(child_collection_id, child_nft_id)
				);
				Self::nft_burn(
					child_collection_id,
					child_nft_id,
					max_recursions - 1,
				)?;
			}
		}
		Ok((collection_id, nft_id))
	}

	fn nft_send(
		sender: T::AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
		new_owner: AccountIdOrCollectionNftTuple<T::AccountId>,
		max_recursions: u32,
	) -> sp_std::result::Result<T::AccountId, DispatchError> {

		let (root_owner, root_nft) = Pallet::<T>::lookup_root_owner(collection_id, nft_id)?;
		// Check ownership
		ensure!(sender == root_owner, Error::<T>::NoPermission);
		// Get NFT info
		let mut sending_nft =
			NFTs::<T>::get(collection_id, nft_id).ok_or(Error::<T>::NoAvailableNftId)?;

		// Prepare transfer
		let new_owner_account = match new_owner.clone() {
			AccountIdOrCollectionNftTuple::AccountId(id) => id,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(cid, nid) => {
				// Check if NFT target exists
				ensure!(NFTs::<T>::contains_key(cid, nid), Error::<T>::NoAvailableNftId);
				// Check if sending to self
				ensure!((collection_id, nft_id) != (cid, nid), Error::<T>::CannotSendToDescendentOrSelf);
				// Check if collection_id & nft_id are descendent of cid & nid
				ensure!(
					!Pallet::<T>::is_x_descendent_of_y(cid, nid, collection_id, nft_id),
					Error::<T>::CannotSendToDescendentOrSelf
				);
				// Convert to virtual account
				Pallet::<T>::nft_to_account_id::<T::AccountId>(cid, nid)
			},
		};

		NFTs::<T>::insert(collection_id, nft_id, sending_nft);

		Ok(new_owner_account)
	}
}

impl<T: Config> Pallet<T>
where
	T: pallet_uniques::Config<ClassId = CollectionId, InstanceId = NftId>,
{

	/// Encodes a RMRK NFT with randomness + `collection_id` + `nft_id` into a virtual account
	/// then returning the `AccountId`. Note that we must be careful of the size of `AccountId`
	/// as it must be wide enough to keep the size of the prefix as well as the `collection_id`
	/// and `nft_id`.
	///
	/// Parameters:
	/// - `collection_id`: Collection ID that the NFT is contained in
	/// - `nft_id`: NFT ID to be encoded into a virtual account
	///
	/// Output:
	/// `AccountId`: Encoded virtual account that represents the NFT
	///
	/// # Example
	/// ```
	/// let collection_id = 0;
	/// let nft_id = 0;
	///
	/// assert_eq!(nft_to_account_id(collection_id, nft_id), "5Co5sje8foechzYWmKU7PgQsBX349YhqaMb8kZHu19HyYNEQ");
	/// ```
	pub fn nft_to_account_id<AccountId: Codec>(collection_id: CollectionId, nft_id: NftId) -> AccountId {
		(SALT_RMRK_NFT, collection_id, nft_id)
			.using_encoded(|b| AccountId::decode(&mut TrailingZeroInput::new(b)))
			.expect("Decoding with trailing zero never fails; qed.")
	}

	/// Decodes a RMRK NFT a suspected virtual account
	/// then returns an `Option<(CollectionId, NftId)>
	/// where `None` is returned when there is an actual account
	/// and `Some(tuple)` returns tuple of `CollectionId` & `NftId`
	///
	/// Parameters:
	/// - `account_id`: Encoded NFT virtual account or account owner
	///
	/// Output:
	/// `Option<(CollectionId, NftId)>`
	/// # Example
	/// ```
	/// let virtual_account = "5Co5sje8foechzYWmKU7PgQsBX349YhqaMb8kZHu19HyYNEQ";
	/// let collection_id = 0;
	/// let nft_id = 0;
	///
	/// assert_eq!(decode_nft_account_id(virtual_account), Some((collection_id, nft_id)));
	/// ```
	pub fn decode_nft_account_id<AccountId: Codec>(account_id: T::AccountId) -> Option<(CollectionId, NftId)> {
		let (prefix, tuple, suffix) = account_id
			.using_encoded(|mut b| {
				let slice = &mut b;
				let r = <([u8; 8], (CollectionId, NftId))>::decode(slice);
				r.map(|(prefix, tuple)| (prefix, tuple, slice.to_vec()))
			})
			.ok()?;
		// Check prefix and suffix to avoid collision attack
		if &prefix == SALT_RMRK_NFT && suffix.iter().all(|&x| x == 0) {
			Some(tuple)
		} else {
			None
		}
	}

	/// Looks up the root owner of an NFT and returns a `Result` with an AccountId and
	/// a tuple of the root `(CollectionId, NftId)`
	/// or an `Error::<T>::NoAvailableNftId` in the case that the NFT is already burned
	///
	/// Parameters:
	/// - `collection_id`: Collection ID of the NFT to lookup the root owner
	/// - `nft_id`: NFT ID that is to be looked up for the root owner
	///
	/// Output:
	/// - `Result<(T::AcccountId, (CollectionId, NftId)), Error<T>>`
	///
	/// # Example
	/// ```
	/// let parent = Origin::signed(ALICE);
	/// // Alice mints NFTs (0,0) and (0,1) then send (0,1) to (0,0)
	/// let virtual_account = "5Co5sje8foechzYWmKU7PgQsBX349YhqaMb8kZHu19HyYNEQ";
	/// let collection_id = 0;
	/// let nft_id = 1;
	/// let cid = 0;
	/// let nid = 0;
	///
	/// assert_eq!(lookup_root_owner(collection_id, nft_id), Ok((parent, (collection_id, nft_id))));
	/// ```
	pub fn lookup_root_owner(collection_id: CollectionId, nft_id: NftId) -> Result<(T::AccountId, (CollectionId, NftId)), Error<T>> {
		let parent =
			pallet_uniques::Pallet::<T>::owner(collection_id, nft_id);
		// Check if parent returns None which indicates the NFT is not available
		if parent.is_none() {
			return Err(Error::<T>::NoAvailableNftId)
		}
		let owner = parent.as_ref().unwrap();
		match Self::decode_nft_account_id::<T::AccountId>(owner.clone()) {
			None => Ok((owner.clone(), (collection_id, nft_id))),
			Some((cid, nid)) => Pallet::<T>::lookup_root_owner(cid, nid),
		}
	}

	/// Add a child to a parent NFT
	///
	/// Parameters:
	/// - `parent`: Tuple of (CollectionId, NftId) of the parent NFT
	/// - `child`: Tuple of (CollectionId, NftId) of the child NFT to be added
	///
	/// Output:
	/// - Adding a `child` to the Children StorageMap of the `parent`
	pub fn add_child(parent: (CollectionId, NftId), child: (CollectionId, NftId)) {
		Children::<T>::mutate(parent, |v| {
			v.push(child)
		});
	}

	/// Remove a child from a parent NFT
	///
	/// Parameters:
	/// - `parent`: Tuple of (CollectionId, NftId) of the parent NFT
	/// - `child`: Tuple of (CollectionId, NftId) of the child NFT to be removed
	///
	/// Output:
	/// - Removing a `child` from the Children StorageMap of the `parent`
	pub fn remove_child(parent: (CollectionId, NftId), child: (CollectionId, NftId)) {
		Children::<T>::mutate(parent, |v| {
			*v = v.iter().filter(|&nft| *nft != child).cloned().collect();
		});
	}

	/// Has a child NFT present in the Children StorageMap of the parent NFT
	///
	/// Parameters:
	/// - `collection_id`: Collection ID of the NFT to lookup the root owner
	/// - `nft_id`: NFT ID that is to be looked up for the root owner
	///
	/// Output:
	/// - `bool`
	pub fn has_child(parent: (CollectionId, NftId)) -> bool {
		!Children::<T>::get(parent).is_empty()
	}

	/// Check whether a NFT is descends from a suspected parent NFT
	/// and return a `bool` if NFT is or not
	///
	/// Parameters:
	/// - `child_collection_id`: Collection ID of the NFT to lookup the root owner
	/// - `child_nft_id`: NFT ID that is to be looked up for the root owner
	/// - `parent_collection_id`: Collection ID of the NFT to lookup the root owner
	/// - `parent_nft_id`: NFT ID that is to be looked up for the root owner
	/// Output:
	/// - `bool`
	pub fn is_x_descendent_of_y(
		child_collection_id: CollectionId,
		child_nft_id: NftId,
		parent_collection_id: CollectionId,
		parent_nft_id: NftId,
	) -> bool {
		let mut found_child = false;

		let parent =
			pallet_uniques::Pallet::<T>::owner(child_collection_id, child_nft_id);
		// Check if parent returns None which indicates the NFT is not available
		if parent.is_none() {
			return found_child
		}
		let owner = parent.as_ref().unwrap();
		return match Self::decode_nft_account_id::<T::AccountId>(owner.clone()) {
			None => found_child,
			Some((cid, nid)) => {
				if (cid, nid) == (parent_collection_id, parent_nft_id) {
					found_child = true
				} else {
					found_child = Pallet::<T>::is_x_descendent_of_y(
						cid,
						nid,
						parent_collection_id,
						parent_nft_id
					)
				}
				found_child
			},
		}
	}

	pub fn recursive_burn(
		collection_id: CollectionId,
		nft_id: NftId,
		max_recursions: u32,
	) -> DispatchResult {
		ensure!(max_recursions > 0, Error::<T>::TooManyRecursions);
		NFTs::<T>::remove(collection_id, nft_id);
		if let kids = Children::<T>::take((collection_id, nft_id)) {
			for (child_collection_id, child_nft_id) in kids {
				Pallet::<T>::recursive_burn(child_collection_id, child_nft_id, max_recursions - 1)?;
			}
		}
		Ok(())
	}

	pub fn to_bounded_string(name: Vec<u8>) -> Result<BoundedVec<u8, T::StringLimit>, Error<T>> {
		name.try_into().map_err(|_| Error::<T>::TooLong)
	}

	pub fn to_optional_bounded_string(
		name: Option<Vec<u8>>,
	) -> Result<Option<BoundedVec<u8, T::StringLimit>>, Error<T>> {
		Ok(match name {
			Some(n) => Some(Self::to_bounded_string(n)?),
			None => None,
		})		
	}

	pub fn get_next_nft_id(collection_id: CollectionId) -> Result<NftId, Error<T>> {
		NextNftId::<T>::try_mutate(collection_id, |id| {
			let current_id = *id;
			*id = id.checked_add(1).ok_or(Error::<T>::NoAvailableNftId)?;
			Ok(current_id)
		})
	}

	pub fn get_next_resource_id() -> Result<ResourceId, Error<T>> {
		NextResourceId::<T>::try_mutate(|id| {
			let current_id = *id;
			*id = id.checked_add(1).ok_or(Error::<T>::NoAvailableCollectionId)?;
			Ok(current_id)
		})
	}

}
