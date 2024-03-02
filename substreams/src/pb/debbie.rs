// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc20Deployment {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub total_supply: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub decimals: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub blocknumber: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc20Deployments {
    #[prost(message, repeated, tag="1")]
    pub contracts: ::prost::alloc::vec::Vec<Erc20Deployment>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc721Deployment {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub blocknumber: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub token_uri: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc721Deployments {
    #[prost(message, repeated, tag="1")]
    pub contracts: ::prost::alloc::vec::Vec<Erc721Deployment>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc20HoldersTransfers {
    #[prost(message, repeated, tag="1")]
    pub erc20transfers: ::prost::alloc::vec::Vec<Erc20Transfer>,
    #[prost(message, repeated, tag="2")]
    pub token_holders: ::prost::alloc::vec::Vec<TokenHolder>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc20Transfer {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub count: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub volume: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub blocknumber: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc20Transfers {
    #[prost(message, repeated, tag="1")]
    pub transfers: ::prost::alloc::vec::Vec<Erc20Transfer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc721Transfer {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub token_id: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub volume: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub blocknumber: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc721Transfers {
    #[prost(message, repeated, tag="1")]
    pub transfers: ::prost::alloc::vec::Vec<Erc721Transfer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MasterProto {
    #[prost(message, repeated, tag="1")]
    pub erc20contracts: ::prost::alloc::vec::Vec<Erc20Deployment>,
    #[prost(message, repeated, tag="2")]
    pub erc721contracts: ::prost::alloc::vec::Vec<Erc721Deployment>,
    #[prost(message, repeated, tag="3")]
    pub erc20transfers: ::prost::alloc::vec::Vec<Erc20Transfer>,
    #[prost(message, repeated, tag="4")]
    pub erc721transfers: ::prost::alloc::vec::Vec<Erc721Transfer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenHolders {
    #[prost(message, repeated, tag="1")]
    pub token_holders: ::prost::alloc::vec::Vec<TokenHolder>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenHolder {
    #[prost(string, tag="1")]
    pub holder_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub token_address: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub balance: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub transfer_volume: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub transfer_count: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub transfer_amount: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub blocknumber: ::prost::alloc::string::String,
    #[prost(int64, tag="8")]
    pub month_id: i64,
    #[prost(int64, tag="9")]
    pub timestamp_seconds: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc721TransfersHoldersTokens {
    #[prost(message, repeated, tag="1")]
    pub transfers: ::prost::alloc::vec::Vec<Erc721Transfer>,
    #[prost(message, repeated, tag="2")]
    pub erc721_token_holders: ::prost::alloc::vec::Vec<NftHolder>,
    #[prost(message, repeated, tag="3")]
    pub erc721_tokens: ::prost::alloc::vec::Vec<Erc721Token>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc721Tokens {
    #[prost(message, repeated, tag="1")]
    pub erc721_tokens: ::prost::alloc::vec::Vec<Erc721Token>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc721Token {
    #[prost(string, tag="1")]
    pub token_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub token_address: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub transfer_volume: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NftHolders {
    #[prost(message, repeated, tag="1")]
    pub erc721_token_holders: ::prost::alloc::vec::Vec<NftHolder>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NftHolder {
    #[prost(string, tag="1")]
    pub holder_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub token_address: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub token_balance: ::prost::alloc::string::String,
    #[prost(bool, tag="4")]
    pub transfer_from: bool,
}
// @@protoc_insertion_point(module)
