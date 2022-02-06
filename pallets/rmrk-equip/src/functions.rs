use super::*;

impl<T: Config> Pallet<T> {
	pub fn get_next_base_id() -> Result<BaseId, Error<T>> {
		NextBaseId::<T>::try_mutate(|id| {
			let current_id = *id;
			*id = id.checked_add(1).ok_or(Error::<T>::NoAvailableBaseId)?;
			Ok(current_id)
		})
	}
}


impl<T: Config> Base<T::AccountId, StringLimitOf<T>> for Pallet<T>
{
	fn base_create(
		issuer: T::AccountId,
		base_type: StringLimitOf<T>,
		symbol: StringLimitOf<T>,
		parts: Vec<FixedOrSlotPart<StringLimitOf<T>>>,
	) -> Result<BaseId, DispatchError> {
		let base_id = Self::get_next_base_id()?;
		let base = BaseInfo { issuer, base_type, symbol, parts };
		Bases::<T>::insert(base_id, base);
		Ok(base_id)
	}
	fn do_equip(
		issuer: T::AccountId, // Maybe don't need?
		nft: NftId,
		base_id: u32, // Maybe BaseId ?
		slot: u32 // Maybe SlotId ?
	)-> Result<(), DispatchError> {
		Equippings::<T>::insert(nft, base_id, slot);
		Ok(())
	}
}
