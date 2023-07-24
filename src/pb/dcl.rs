// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigInt {
    #[prost(string, tag="1")]
    pub value: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Items {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<Item>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Item {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub rarity: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub max_supply: ::core::option::Option<BigInt>,
    #[prost(message, optional, tag="4")]
    pub total_supply: ::core::option::Option<BigInt>,
    #[prost(message, optional, tag="5")]
    pub price: ::core::option::Option<BigInt>,
    #[prost(string, tag="6")]
    pub beneficiary: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub metadata: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub content_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="9")]
    pub blockchain_item_id: ::core::option::Option<BigInt>,
    #[prost(string, tag="10")]
    pub collection_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="11")]
    pub created_at: u64,
    #[prost(string, tag="12")]
    pub item_type: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub category: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub body_shapes: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Collections {
    #[prost(message, repeated, tag="1")]
    pub collections: ::prost::alloc::vec::Vec<Collection>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Collection {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub owner: ::prost::alloc::string::String,
    #[prost(bool, tag="6")]
    pub is_completed: bool,
    #[prost(bool, tag="7")]
    pub is_approved: bool,
    #[prost(uint64, tag="8")]
    pub created_at: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NfTs {
    #[prost(message, repeated, tag="1")]
    pub nfts: ::prost::alloc::vec::Vec<Nft>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Nft {
    #[prost(string, tag="1")]
    pub beneficiary: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub token_id: ::core::option::Option<BigInt>,
    #[prost(string, tag="3")]
    pub item_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub issued_id: ::core::option::Option<BigInt>,
    #[prost(string, tag="5")]
    pub collection_address: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub created_at: u64,
    #[prost(uint64, tag="7")]
    pub updated_at: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfers {
    #[prost(message, repeated, tag="1")]
    pub transfers: ::prost::alloc::vec::Vec<Transfer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfer {
    #[prost(string, tag="1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub to: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub token_id: ::core::option::Option<BigInt>,
    #[prost(string, tag="4")]
    pub collection_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub created_at: u64,
    #[prost(string, tag="6")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="7")]
    pub log_index: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LandTransfers {
    #[prost(message, repeated, tag="1")]
    pub land_transfers: ::prost::alloc::vec::Vec<LandTransfer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LandTransfer {
    #[prost(string, tag="1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub to: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub asset_id: ::core::option::Option<BigInt>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompletedCollections {
    #[prost(string, repeated, tag="1")]
    pub completed_collections: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(uint64, tag="1")]
    pub number: u64,
    #[prost(uint64, tag="2")]
    pub timestamp: u64,
}
// @@protoc_insertion_point(module)
