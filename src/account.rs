use serde_derive::{Serialize, Deserialize};
use restson::{Error as RestsonError,RestPath};

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

/// Currently authenticated member
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Member {
    id: u8,
    company_name: String,
    postcode: u8,
    active: bool,
}

impl RestPath<()> for Member {
    fn get_path(_: ()) -> Result<String, RestsonError> { Ok(String::from("member")) }
}

/// A user account (sender or receiver)
/// 
/// 
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    pub address: String,
    pub email: String,
    pub name: String,
    pub postcode: String,
    pub state: String,
    pub suburb: String,
    #[serde(alias = "type")]
    pub kind: String, // "type" is a keyword
    pub country: String, // two-letter ISO country code
    pub company_name: String,
}