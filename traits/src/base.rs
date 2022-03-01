use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{RuntimeDebug, DispatchError};
use crate::primitives::{BaseId, SlotId};
use sp_std::{vec::Vec};
use super::theme::{Theme};
use super::part::{NewPartTypes, EquippableList};


#[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct BaseInfo<AccountId, BoundedString> {
	/// Original creator of the Base
	pub issuer: AccountId,
	/// Specifies how an NFT should be rendered, ie "svg"
	pub base_type: BoundedString,
	/// User provided symbol during Base creation
	pub symbol: BoundedString,
	/// Parts, full list of both Fixed and Slot parts
	pub parts: Vec<NewPartTypes<BoundedString>>,
}

// Abstraction over a Base system.
pub trait Base<AccountId, CollectionId, NftId, BoundedString> {
	fn base_create(
		issuer: AccountId,
		base_type: BoundedString,
		symbol: BoundedString,
		parts: Vec<NewPartTypes<BoundedString>>
) -> Result<BaseId, DispatchError>;
	fn do_equip(
		issuer: AccountId, // Maybe don't need?
		item: (CollectionId, NftId),
		equipper: (CollectionId, NftId),
		base_id: BaseId, // Maybe BaseId ?
		slot: SlotId // Maybe SlotId ?
)-> Result<(CollectionId, NftId, BaseId, SlotId, bool), DispatchError>;
	fn do_equippable(
		issuer: AccountId, 
		base_id: BaseId,
		slot: SlotId,
		equippables: EquippableList,
	)-> Result<(BaseId, SlotId), DispatchError>;
	fn add_theme(
		issuer: AccountId,
		base_id: BaseId,
		theme: Theme<BoundedString>,
	) -> Result<(), DispatchError>;
}
