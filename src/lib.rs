pub mod account;
pub mod booking;
pub mod client;
pub mod error;
pub mod order;
pub mod product;

type CommonUnsigned = u32;
type CommonFloat    = f64;

pub type Account = account::Account;
pub type AuthenticateWith<'a> = account::AuthenticateWith<'a>;
pub type Member = account::Member;

pub type BookingStatus = booking::BookingStatus;
pub type BookingRequest<'a> = booking::BookingRequest<'a, CommonUnsigned, CommonFloat>;
pub type BookingResponse = booking::BookingResponse<CommonUnsigned, CommonFloat>;

pub type TransdirectClient<'a> = client::Client<'a>;

pub type Error = error::Error;

pub type OrderStatus = order::OrderStatus;
// Missing Order

pub type Dimensions = product::Dimensions<CommonUnsigned>;
pub type Product = product::Product<CommonUnsigned>;
pub type Service = product::Service<CommonFloat>;
