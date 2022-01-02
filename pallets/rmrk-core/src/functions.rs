use sp_runtime::{traits::Saturating, ArithmeticError};

use super::*;

impl<T: Config> Priority<StringLimitOf<T>, T::AccountId> for Pallet<T> {
	fn priority_set(
		sender: T::AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
		priorities: Vec<Vec<u8>>,
	) -> DispatchResult {
		let mut bounded_priorities = Vec::<BoundedVec<u8, T::StringLimit>>::new();
		for priority in priorities {
			let bounded_priority = Self::to_bounded_string(priority)?;
			bounded_priorities.push(bounded_priority);
		}
		Priorities::<T>::insert(collection_id, nft_id, bounded_priorities);
		Self::deposit_event(Event::PrioritySet { collection_id, nft_id });
		Ok(())
	}
}

impl<T: Config> Property<KeyLimitOf<T>, ValueLimitOf<T>, T::AccountId> for Pallet<T> {
	fn property_set(
		sender: T::AccountId,
		collection_id: CollectionId,
		maybe_nft_id: Option<NftId>,
		key: KeyLimitOf<T>,
		value: ValueLimitOf<T>,
	) -> DispatchResult {
		let collection =
			Collections::<T>::get(&collection_id).ok_or(Error::<T>::NoAvailableCollectionId)?;
		ensure!(collection.issuer == sender, Error::<T>::NoPermission);
		if let Some(nft_id) = &maybe_nft_id {
			ensure!(NFTs::<T>::contains_key(collection_id, nft_id), Error::<T>::NoAvailableNftId);
			if let Some(nft) = NFTs::<T>::get(collection_id, nft_id) {
				ensure!(nft.rootowner == collection.issuer, Error::<T>::NoPermission);
			}
		}
		Properties::<T>::insert((&collection_id, maybe_nft_id, &key), &value);
		Self::deposit_event(Event::PropertySet { collection_id, maybe_nft_id, key, value });
		Ok(())
	}
}

impl<T: Config> Resource<StringLimitOf<T>, T::AccountId> for Pallet<T> {
	fn resource_add(
		sender: T::AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
		base: Option<BoundedVec<u8, T::StringLimit>>,
		src: Option<BoundedVec<u8, T::StringLimit>>,
		metadata: Option<BoundedVec<u8, T::StringLimit>>,
		slot: Option<BoundedVec<u8, T::StringLimit>>,
		license: Option<BoundedVec<u8, T::StringLimit>>,
		thumb: Option<BoundedVec<u8, T::StringLimit>>,
	) -> Result<ResourceId, DispatchError> {
		let nft = NFTs::<T>::get(collection_id, nft_id).ok_or(Error::<T>::NoAvailableNftId)?;

		let resource_id = Self::get_next_resource_id()?;
		ensure!(
			Resources::<T>::get((collection_id, nft_id, resource_id)).is_none(),
			Error::<T>::ResourceAlreadyExists
		);

		let empty = base.is_none()
			&& src.is_none()
			&& metadata.is_none()
			&& slot.is_none()
			&& license.is_none()
			&& thumb.is_none();
		ensure!(!empty, Error::<T>::EmptyResource);

		let res = ResourceInfo {
			id: resource_id,
			base,
			src,
			metadata,
			slot,
			license,
			thumb,
			pending: nft.rootowner != sender,
		};
		Resources::<T>::insert((collection_id, nft_id, resource_id), res);

		Ok(resource_id)
	}

	fn accept(
		sender: T::AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
		resource_id: ResourceId,
	) -> DispatchResult {
		let nft = NFTs::<T>::get(collection_id, nft_id).ok_or(Error::<T>::NoAvailableNftId)?;
		ensure!(nft.rootowner == sender, Error::<T>::NoPermission);

		Resources::<T>::try_mutate_exists(
			(collection_id, nft_id, resource_id),
			|resource| -> DispatchResult {
				if let Some(res) = resource {
					res.pending = false;
				}
				Ok(())
			},
		)?;

		Self::deposit_event(Event::ResourceAccepted { nft_id, resource_id });
		Ok(())
	}
}

impl<T: Config> Collection<StringLimitOf<T>, T::AccountId> for Pallet<T> {
	fn issuer(_collection_id: CollectionId) -> Option<T::AccountId> {
		None
	}
	fn collection_create(
		issuer: T::AccountId,
		metadata: StringLimitOf<T>,
		max: u32,
		symbol: StringLimitOf<T>,
	) -> Result<CollectionId, DispatchError> {
		let collection =
			CollectionInfo { issuer: issuer.clone(), metadata, max, symbol, nfts_count: 0 };
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

	fn collection_burn(_issuer: T::AccountId, collection_id: CollectionId) -> DispatchResult {
		let collection = Self::collections(collection_id).ok_or(Error::<T>::CollectionUnknown)?;
		ensure!(collection.nfts_count == 0, Error::<T>::CollectionNotEmpty);
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
			collection.max = collection.nfts_count.try_into().unwrap();
			Ok(())
		})?;
		Ok(collection_id)
	}
}

impl<T: Config> Nft<T::AccountId, StringLimitOf<T>> for Pallet<T> {
	type MaxRecursions = T::MaxRecursions;

	fn nft_mint(
		_sender: T::AccountId,
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

		let rootowner = owner.clone();
		let owner_as_maybe_account = AccountIdOrCollectionNftTuple::AccountId(owner.clone());

		let nft =
			NftInfo { owner: owner_as_maybe_account, rootowner, recipient, royalty, metadata };

		NFTs::<T>::insert(collection_id, nft_id, nft);
		NftsByOwner::<T>::append(owner, (collection_id, nft_id));

		// increment nfts counter
		let nfts_count = collection.nfts_count.checked_add(1).ok_or(ArithmeticError::Overflow)?;
		Collections::<T>::try_mutate(collection_id, |collection| -> DispatchResult {
			let collection = collection.as_mut().ok_or(Error::<T>::CollectionUnknown)?;
			collection.nfts_count = nfts_count;
			Ok(())
		})?;

		Ok((collection_id, nft_id))
	}

	fn nft_burn(
		collection_id: CollectionId,
		nft_id: NftId,
		max_recursions: u32,
	) -> sp_std::result::Result<(CollectionId, NftId), DispatchError> {
		ensure!(max_recursions > 0, Error::<T>::TooManyRecursions);
		NFTs::<T>::remove(collection_id, nft_id);
		if let Some(kids) = Children::<T>::take(collection_id, nft_id) {
			for (child_collection_id, child_nft_id) in kids {
				Self::nft_burn(child_collection_id, child_nft_id, max_recursions - 1)?;
			}
		}

		// decrement nfts counter
		Collections::<T>::try_mutate(collection_id, |collection| -> DispatchResult {
			let collection = collection.as_mut().ok_or(Error::<T>::CollectionUnknown)?;
			collection.nfts_count.saturating_dec();
			Ok(())
		})?;

		Ok((collection_id, nft_id))
	}

	fn nft_send(
		sender: T::AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
		new_owner: AccountIdOrCollectionNftTuple<T::AccountId>,
		max_recursions: u32,
	) -> sp_std::result::Result<(CollectionId, NftId), DispatchError> {
		let mut sending_nft =
			NFTs::<T>::get(collection_id, nft_id).ok_or(Error::<T>::NoAvailableNftId)?;
		ensure!(&sending_nft.rootowner == &sender, Error::<T>::NoPermission);

		match new_owner.clone() {
			AccountIdOrCollectionNftTuple::AccountId(account_id) => {
				// Remove previous parental relationship
				if let AccountIdOrCollectionNftTuple::CollectionAndNftTuple(cid, nid) =
					sending_nft.owner
				{
					if let Some(mut kids) = Children::<T>::take(cid, nid) {
						kids.retain(|&kid| kid != (collection_id, nft_id));
						Children::<T>::insert(cid, nid, kids);
					}
				}
				sending_nft.rootowner = account_id.clone();
			}
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(cid, nid) => {
				let recipient_nft = NFTs::<T>::get(cid, nid).ok_or(Error::<T>::NoAvailableNftId)?;
				// Check if sending NFT is already a child of recipient NFT
				ensure!(
					!Pallet::<T>::is_x_descendent_of_y(cid, nid, collection_id, nft_id),
					Error::<T>::CannotSendToDescendent
				);

				// Remove parent if exists: first we only care if the owner is a non-AccountId)
				if let AccountIdOrCollectionNftTuple::CollectionAndNftTuple(cid, nid) =
					sending_nft.owner
				{
					// second we only care if the parent has children (it should)
					if let Some(mut kids) = Children::<T>::take(cid, nid) {
						// third we only "retain" the other children
						kids.retain(|&kid| kid != (collection_id, nft_id));
						Children::<T>::insert(cid, nid, kids);
					}
				}
				if sending_nft.rootowner != recipient_nft.rootowner {
					// sending_nft.rootowner = recipient_nft.rootowner
					sending_nft.rootowner = recipient_nft.rootowner.clone();

					Pallet::<T>::recursive_update_rootowner(
						collection_id,
						nft_id,
						recipient_nft.rootowner,
						max_recursions,
					)?;
				}
				match Children::<T>::take(cid, nid) {
					None => Children::<T>::insert(cid, nid, vec![(collection_id, nft_id)]),
					Some(mut kids) => {
						kids.push((collection_id, nft_id));
						Children::<T>::insert(cid, nid, kids);
					}
				}
			}
		};
		sending_nft.owner = new_owner.clone();

		NFTs::<T>::insert(collection_id, nft_id, sending_nft);

		Ok((collection_id, nft_id))
	}
}

impl<T: Config> Pallet<T> {
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
					return true;
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
