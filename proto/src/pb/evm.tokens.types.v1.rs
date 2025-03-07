// @generated
// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="1")]
    pub transfers: ::prost::alloc::vec::Vec<Transfer>,
    #[prost(message, repeated, tag="2")]
    pub balance_changes: ::prost::alloc::vec::Vec<BalanceChange>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceChange {
    /// -- block --
    #[prost(uint64, tag="1")]
    pub block_num: u64,
    #[prost(string, tag="2")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="4")]
    pub date: ::prost::alloc::string::String,
    /// -- transaction --
    #[prost(string, tag="5")]
    pub transaction_id: ::prost::alloc::string::String,
    /// -- call --
    #[prost(uint32, tag="6")]
    pub call_index: u32,
    /// may indicate the “to” or “from” in a lower-level call context, but is not the address that emitted the event.
    #[prost(string, tag="7")]
    pub call_address: ::prost::alloc::string::String,
    /// -- log --
    ///
    /// Index is the index of the log relative to the transaction. This index is always populated regardless of the state revertion of the the call that emitted this log.
    #[prost(uint32, tag="10")]
    pub log_index: u32,
    /// BlockIndex represents the index of the log relative to the Block.
    #[prost(uint32, tag="11")]
    pub log_block_index: u32,
    /// the block's global ordinal when the transfer was recorded.
    #[prost(uint64, tag="12")]
    pub log_ordinal: u64,
    /// -- balance change --
    ///
    /// storage_change.address
    #[prost(string, tag="20")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag="21")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="22")]
    pub old_balance: ::prost::alloc::string::String,
    #[prost(string, tag="23")]
    pub new_balance: ::prost::alloc::string::String,
    /// -- indexing --
    ///
    /// storage.ordinal or balance_change.ordinal
    #[prost(uint64, tag="30")]
    pub ordinal: u64,
    /// latest global sequence of the balance change (block_num << 32 + index)
    #[prost(uint64, tag="31")]
    pub global_sequence: u64,
    /// -- metadata --
    #[prost(enumeration="BalanceChangeType", tag="99")]
    pub r#type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfer {
    /// -- block --
    #[prost(uint64, tag="1")]
    pub block_num: u64,
    #[prost(string, tag="2")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="4")]
    pub date: ::prost::alloc::string::String,
    /// -- transaction --
    #[prost(string, tag="5")]
    pub transaction_id: ::prost::alloc::string::String,
    /// -- call --
    #[prost(uint32, tag="6")]
    pub call_index: u32,
    /// may indicate the “to” or “from” in a lower-level call context, but is not the address that emitted the event.
    #[prost(string, tag="7")]
    pub call_address: ::prost::alloc::string::String,
    /// -- log --
    ///
    /// Index is the index of the log relative to the transaction. This index is always populated regardless of the state revertion of the the call that emitted this log.
    #[prost(uint32, tag="10")]
    pub log_index: u32,
    /// BlockIndex represents the index of the log relative to the Block.
    #[prost(uint32, tag="11")]
    pub log_block_index: u32,
    /// the block's global ordinal when the transfer was recorded.
    #[prost(uint64, tag="12")]
    pub log_ordinal: u64,
    /// -- transfer --
    ///
    /// log.address
    #[prost(string, tag="20")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag="21")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="22")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="23")]
    pub value: ::prost::alloc::string::String,
    /// -- metadata --
    #[prost(enumeration="TransferType", tag="99")]
    pub r#type: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BalanceChangeType {
    Unspecified = 0,
    /// ERC-20 within a Transfer call
    Erc20Algo1 = 1,
    /// ERC-20 different Transfer call
    Erc20Algo2 = 2,
    /// Native (ETH)
    Native = 3,
}
impl BalanceChangeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BalanceChangeType::Unspecified => "BALANCE_CHANGE_TYPE_UNSPECIFIED",
            BalanceChangeType::Erc20Algo1 => "BALANCE_CHANGE_TYPE_ERC20_ALGO_1",
            BalanceChangeType::Erc20Algo2 => "BALANCE_CHANGE_TYPE_ERC20_ALGO_2",
            BalanceChangeType::Native => "BALANCE_CHANGE_TYPE_NATIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BALANCE_CHANGE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "BALANCE_CHANGE_TYPE_ERC20_ALGO_1" => Some(Self::Erc20Algo1),
            "BALANCE_CHANGE_TYPE_ERC20_ALGO_2" => Some(Self::Erc20Algo2),
            "BALANCE_CHANGE_TYPE_NATIVE" => Some(Self::Native),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransferType {
    Unspecified = 0,
    /// ERC-20 mint
    Mint = 1,
    /// ERC-20 burn
    Burn = 2,
    /// ERC-20 fishing
    Fishing = 3,
}
impl TransferType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TransferType::Unspecified => "TRANSFER_TYPE_UNSPECIFIED",
            TransferType::Mint => "TRANSFER_TYPE_MINT",
            TransferType::Burn => "TRANSFER_TYPE_BURN",
            TransferType::Fishing => "TRANSFER_TYPE_FISHING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TRANSFER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "TRANSFER_TYPE_MINT" => Some(Self::Mint),
            "TRANSFER_TYPE_BURN" => Some(Self::Burn),
            "TRANSFER_TYPE_FISHING" => Some(Self::Fishing),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
