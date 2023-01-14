#![allow(non_snake_case, dead_code)]
pub mod Transdirect {
    use std::str::FromStr;
    use std::collections::HashMap;
    use std::default::Default;
    use restson as rest;
    use num_traits::{Float, Unsigned};
    use serde_derive::{Deserialize, Serialize};
    
    type Booking = BookingResponse<f64, u32>; // Most general case
    
    static API_ENDPOINT: &str = "https://www.transdirect.com.au/api/";

    /// Errors which can be returned from the Transdirect API
    /// 
    /// 
    #[non_exhaustive]
    #[derive(Debug, PartialEq, Eq)]
    pub enum Error {
        UnreadableResponse,
        UnknownStatus,
        HTTPError(String),
    }

    /// Enum describing the status of a booking
    /// 
    /// As defined by the [specification](https://transdirectapiv4.docs.apiary.io/reference/bookings-/-simple-quotes/single-booking)
    #[non_exhaustive]
    #[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
    pub enum BookingStatus {
        New,
        PendingPayment,
        Paid,
        RequestSent,
        Reviewed,
        Confirmed,
        Cancelled,
        PendingReview,
        RequestFailed,
        BookedManually,
    }
    
    /// Enum describing the status of an order (as a result of a booking request)
    /// 
    /// Defined in the [transdirect API documentation](https://transdirectapiv4.docs.apiary.io/reference/orders/create-orders)
    #[non_exhaustive]
    #[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
    pub enum OrderStatus {
        Pending,
        Booked,
        ManuallyDispatched,
        Cancelled,
    }
    
    /// Enum describing possible authentication objects
    ///
    /// OAuth authentication is not yet supported
    /// 
    /// User-Password Basic authentication is supported, as is
    /// API key authentication
    #[non_exhaustive]
    pub enum AuthenticateWith<'a> {
        Basic(&'a str, &'a str),
        APIKey(&'a str),
    }

    impl FromStr for BookingStatus {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "new"             => Ok(Self::New),
                "pending_payment" => Ok(Self::PendingPayment),
                "paid"            => Ok(Self::Paid),
                "request_sent"    => Ok(Self::RequestSent),
                "reviewed"        => Ok(Self::Reviewed),
                "confirmed"       => Ok(Self::Confirmed),
                "cancelled"       => Ok(Self::Cancelled),
                "pending_review"  => Ok(Self::PendingReview),
                "request_failed"  => Ok(Self::RequestFailed),
                "booked_manually" => Ok(Self::BookedManually),
                _ => Err(Self::Err::UnknownStatus),
            }
        }
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

    /// Represents an item and its associated quantity
    /// 
    /// 
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Product<T, U> where T: Float, U: Unsigned  {
        pub weight: T,
        pub length: T,
        pub height: T,
        pub width: T,
        pub quantity: U,
        pub description: String,
        pub id: Option<u32>,
    }
    
    /// A service provided by one of the companies listed by Transdirect.
    /// 
    ///
    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Service<T> where T: Float {
        pub total: T,
        pub price_insurance_ex: T,
        pub fee: T,
        pub insured_amount: T,
        pub service: String,
        pub transit_time: String,
        pub pickup_dates: Vec<String>,
        pub pickup_time: HashMap<String, String>,
    }
    
    /// A user account (sender or receiver)
    /// 
    /// 
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Account {
        address: String,
        email: String,
        name: String,
        postcode: u8,
        state: String,
        suburb: String,
        kind: String, // "type" is a keyword
        country: String, // two-letter ISO country code
        company_name: String,
    }

    /// Currently authenticated member
    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Member {
        id: u8,
        company_name: String,
        postcode: u8,
        active: bool,
    }

    /// Represents a single booking request (quote or order)
    /// 
    /// 
    #[derive(Debug, Serialize, Default)]
    pub struct BookingRequest<'a, T, U> where T: Float, U: Unsigned {
        declared_value: f64,
        referrer: String,
        requesting_site: String,
        tailgate_pickup: bool,
        tailgate_delivery: bool,
        items: Vec<Product<T, U>>,
        sender: Option<&'a Account>,
        receiver: Option<&'a Account>,
    }
    
    /// Represents a response due to a booking request from the server
    /// 
    ///
    #[derive(Debug, Deserialize)]
    pub struct BookingResponse<T, U> where T: Float, U: Unsigned {
        id: u32,
        status: BookingStatus,
        #[serde(with = "time::serde::rfc3339")]
        booked_at: time::OffsetDateTime,
        booked_by: String, // Expected to be "sender"
        #[serde(with = "time::serde::rfc3339")]
        created_at: time::OffsetDateTime,
        #[serde(with = "time::serde::rfc3339")]
        updated_at: time::OffsetDateTime,
        declared_value: f64,
        insured_value: f64,
        description: Option<String>,
        items: Vec<Product<T, U>>,
        label: String,
        // notifications: 
        quotes: Vec<Service<T>>,
        sender: Account,
        receiver: Account,
        pickup_window: Vec<String>,
        connote: String,
        charged_weight: u8,
        scanned_weight: u8,
        special_instructions: String,
        tailgate_delivery: bool,
    }
    
    /// Client object for interacting with the API
    /// 
    /// 
    pub struct Client<'a> {
        authenticated: bool,
        restclient: rest::blocking::RestClient,
        sender: Option<&'a Account>,
        receiver: Option<&'a Account>,
    }
    
    impl<'a> Client<'a> {
        pub fn new() -> Self {
            Self {
                authenticated: false,
                restclient: rest::RestClient::new_blocking(API_ENDPOINT)
                    .expect("Should be a valid URL or connected to the internet"),
                sender: None,
                receiver: None,
            }
        }
        
        pub fn new_auth(auth: AuthenticateWith) -> Result<Self, Error> {
            let mut newclient = Self::new();
            
            Self::auth(&mut newclient, auth)?;

            Ok(newclient)
        }
        
        pub fn auth(&mut self, auth: AuthenticateWith) -> Result<(), Error> {
            use AuthenticateWith::*;

            match auth {
                Basic(user, pass) => self.restclient.set_auth(user, pass),
                APIKey(key) => self.restclient.set_header("Api-key", key).expect("Should be able to set Api-key header"),
            }
            
            match self.restclient.get::<_, Member>(()) {
                Ok(_) => Ok(()),
                Err(err) => Err(Error::HTTPError(err.to_string())),
            }
        }
        
        pub fn quotes(&self, request: &BookingRequest<f64, u32>) -> Result<Booking, Error> {
            let response  = self
                .restclient
                .post_capture::<_, _, BookingResponse<f64, u32>>((), request)
                .map_err(|e| Error::HTTPError(e.to_string()))?
                .into_inner();
            
            Ok(response)
        }
    }
    
    impl<'a> BookingRequest<'a, f64, u32> {

        pub fn new() -> Self {
            Default::default()
        }
        
        /// Convenience method to set the sender
        pub fn sender(&'a mut self, account: &'a Account) -> &'a mut Self {
            self.sender = Some(account);

            self
        }
        
        /// Convenience method to set the receiver
        pub fn receiver(&'a mut self, account: &'a Account) -> &'a mut Self {
            self.receiver = Some(account);

            self
        }

        pub fn declared_value(&mut self, value: &f64) -> &mut Self {
            self.declared_value = value.clone();
            
            if self.declared_value > 25.0 { self.tailgate_pickup(&true); }

            self
        }
        
        pub fn referrer(&mut self, person: &str) -> &mut Self {
            self.referrer = person.to_owned();

            self
        }

        pub fn requesting_site(&mut self, site: &str) -> &mut Self {
            self.requesting_site = site.to_owned();

            self
        }

        pub fn tailgate_pickup(&mut self, tail_pickup: &bool) -> &mut Self {
            self.tailgate_pickup = tail_pickup.clone();

            self
        }
        
        pub fn tailgate_delivery(&mut self, tail_delivery: &bool) -> &mut Self {
            self.tailgate_delivery = tail_delivery.clone();

            self
        }

        pub fn items(&mut self, items: &Vec<Product<f64, u32>>) -> &mut Self {
            self.items = items.to_vec();

            self
        }
    }
    
    impl rest::RestPath<()> for BookingRequest<'_, f64, u32> {
        fn get_path(_: ()) -> Result<String, rest::Error> { Ok(format!("bookings/v4")) }
    }

    impl rest::RestPath<()> for Member {
        fn get_path(_: ()) -> Result<String, rest::Error> { Ok(String::from("member")) }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
