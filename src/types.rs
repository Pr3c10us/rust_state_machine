use crate::RuntimeCall;

pub type AccountId= String;
pub type Balance = u128;
pub type BlockNumber = u32;
pub type Nonce = u32;
pub type Extrinsic = super::support::Extrinsic<AccountId, RuntimeCall>;
pub type Header = super::support::Header<BlockNumber>;
pub type Block = super::support::Block<Header, Extrinsic>;
pub type Content = &'static str;