#![cfg_attr(not(feature = "std"), no_std)]

pub mod base;
pub mod collection;
pub mod nft;
pub mod priority;
pub mod property;
pub mod resource;
pub mod part;
pub mod theme;

pub use base::{BaseInfo, Base};
pub use theme::{Theme, ThemeProperty};
pub use part::{NewPartTypes, FixedPart, SlotPart, EquippableList};
// pub use part::{PartInfo};
pub use collection::{Collection, CollectionInfo};
pub use nft::{AccountIdOrCollectionNftTuple, Nft, NftInfo};
pub use priority::Priority;
pub use property::Property;
pub use resource::{
	ResourceInfo,
	Resource,
};
pub mod primitives {
	pub type CollectionId = u32;
	pub type ResourceId = u32;
	pub type NftId = u32;
	pub type BaseId = u32;
	pub type SlotId = u32;
	pub type PartId = u32;
	pub type ZIndex = u32;
}
