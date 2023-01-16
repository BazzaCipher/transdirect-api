use std::str::FromStr;
use serde_derive::{Serialize,Deserialize};

use crate::Error;
/// Enum describing the status of an order: for member to create a booking
/// at a later date
/// 
/// Defined in the [transdirect API documentation](https://transdirectapiv4.docs.apiary.io/reference/orders/create-orders)
#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Default)]
pub enum OrderStatus {
    #[default]
    Pending,
    Booked,
    ManuallyDispatched,
    Cancelled,
}

impl FromStr for OrderStatus {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pending"             => Ok(Self::Pending),
            "booked"              => Ok(Self::Booked),
            "manually_dispatched" => Ok(Self::ManuallyDispatched),
            "cancelled"           => Ok(Self::Cancelled),
            _ => Err(Self::Err::UnknownStatus)
        }
    }
}