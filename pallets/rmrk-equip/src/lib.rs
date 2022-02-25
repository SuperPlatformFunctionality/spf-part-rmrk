#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]

use frame_support::{BoundedVec, ensure};
use frame_support::dispatch::{
	DispatchError, 
	DispatchResult
};
use sp_std::vec::Vec;

pub use pallet::*;

use rmrk_traits::{
	primitives::*, 
	BaseInfo, 
	Base, 
	NewPartTypes, 
	// FixedPart,
	// SlotPart, 
	AccountIdOrCollectionNftTuple, 
	// PartInfo,
	EquippableList,
	Theme,
	// ThemeProperty
	};

mod functions;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

pub type StringLimitOf<T> = BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>;

pub type BoundedResource<T> = BoundedVec<u8, <T as pallet_rmrk_core::Config>::ResourceSymbolLimit>;


#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_rmrk_core::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		#[pallet::constant]
		type MaxPartsPerBase: Get<u32>;
	}

	#[pallet::storage]
	#[pallet::getter(fn bases)]
	/// Stores bases info
	pub type Bases<T: Config> =
		StorageMap<_, Twox64Concat, BaseId, BaseInfo<T::AccountId, StringLimitOf<T>>>;

	#[pallet::storage]
	#[pallet::getter(fn parts)]
	/// Stores bases info
	pub type Parts<T: Config> =
		StorageDoubleMap<_, Twox64Concat, BaseId, Twox64Concat, PartId, NewPartTypes<StringLimitOf<T>>>;

	#[pallet::storage]
	#[pallet::getter(fn next_base_id)]
	pub type NextBaseId<T: Config> = StorageValue<_, BaseId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn next_part_id)]
	pub type NextPartId<T: Config> = StorageMap<_, Twox64Concat, BaseId, PartId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn equippings)]
	/// Stores Equippings info
	pub type Equippings<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, (CollectionId, NftId)>,
			NMapKey<Blake2_128Concat, BaseId>,
			NMapKey<Blake2_128Concat, SlotId>,
		),
		BoundedResource<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn themes)]
	/// Stores Equippings info
	pub type Themes<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, BaseId>, // Base ID
			NMapKey<Blake2_128Concat, StringLimitOf<T>>, // Theme name
			NMapKey<Blake2_128Concat, StringLimitOf<T>>, // Property name (key)
		),
		StringLimitOf<T>, // Property value
		OptionQuery,
	>;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		BaseCreated { issuer: T::AccountId, base_id: BaseId },
		SlotEquipped {
			item_collection: CollectionId,
			item_nft: NftId,
			base_id: BaseId,
			slot_id: SlotId,
		},
		SlotUnequipped {
			item_collection: CollectionId,
			item_nft: NftId,
			base_id: BaseId,
			slot_id: SlotId,
		},
		EquippablesUpdated {
			base_id: BaseId,
			slot_id: SlotId,
		}
	}

	#[pallet::error]
	pub enum Error<T> {
		PermissionError,
		ItemDoesntExist,
		EquipperDoesntExist,
		NoAvailableBaseId,
		NoAvailablePartId,
		MustBeDirectParent,
		PartDoesntExist,
		BaseDoesntExist,
		CantEquipFixedPart,
		NoResourceForThisBaseFoundOnNft,
		CollectionNotEquippable,
		ItemHasNoResourceToEquipThere,
		NoEquippableOnFixedPart,
		NeedsDefaultThemeFirst,
		AlreadyEquipped,
		UnknownError,
		ExceedsMaxPartsPerBase,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> 
	where
		T: pallet_uniques::Config<ClassId = CollectionId, InstanceId = NftId>,
	{
		/// Equip a child NFT into a parent's slot, or unequip
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn equip(
			origin: OriginFor<T>,
			equipping_item_collection_id: CollectionId,
			equipping_item_nft_id: NftId,
			equipper_collection_id: CollectionId,
			equipper_nft_id: NftId,
			base: BaseId,
			slot: SlotId) -> DispatchResult {

			let sender = ensure_signed(origin)?;

			let (collection_id, nft_id, base_id, slot_id, equipped) = Self::do_equip(
				sender.clone(),
				equipping_item_collection_id,
				equipping_item_nft_id,
				equipper_collection_id,
				equipper_nft_id,
				base,
				slot
			)?;

			if equipped {
				// Send Equip event
				Self::deposit_event(Event::SlotEquipped { 
					item_collection: collection_id,
					item_nft: nft_id,
					base_id: base_id,
					slot_id: slot_id,								
				});
			} else {
				// Send Unequip event
				Self::deposit_event(Event::SlotUnequipped { 
					item_collection: collection_id,
					item_nft: nft_id,
					base_id: base_id,
					slot_id: slot_id,								
				});
			}
			Ok(())
		}

		/// TODO: changes the list of equippable collections on a base's part
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn equippable(
			origin: OriginFor<T>, base_id: BaseId, slot_id: SlotId, equippables: EquippableList
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let (base_id, slot_id) = Self::do_equippable(
				sender,
				base_id,
				slot_id,
				equippables,
			)?;

			Self::deposit_event(Event::EquippablesUpdated {base_id, slot_id});
			Ok(())
		}

		/// TODO: add a new theme to a base
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn theme_add(origin: OriginFor<T>, base_id: BaseId, theme: Theme<BoundedVec<u8, T::StringLimit>>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let _theme_id = Self::add_theme(sender, base_id, theme)?;

			// Self::deposit_event(Event::SomethingStored(something, sender));
			Ok(())
		}

		/// create a base. catalogue of parts. It is not an NFT
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn create_base(
			origin: OriginFor<T>,
			base_type: BoundedVec<u8, T::StringLimit>,
			symbol: BoundedVec<u8, T::StringLimit>,
			parts: Vec<NewPartTypes<StringLimitOf<T>>>
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			
			let part_length: u32 = parts.len().try_into().unwrap();
			ensure!(part_length <= T::MaxPartsPerBase::get(), Error::<T>::ExceedsMaxPartsPerBase);

			let base_id = Self::base_create(sender.clone(), base_type, symbol, parts)?;

			Self::deposit_event(Event::BaseCreated { issuer: sender, base_id });
			Ok(())
		}
	}
}
