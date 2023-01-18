use num_traits::{Float,Unsigned};
use serde::de::DeserializeOwned;
use serde::Serialize;
use restson::{RestClient, blocking::RestClient as BRestClient};

use crate::Error;
use crate::account::{Account,AuthenticateWith,Member};
use crate::booking::{BookingRequest,BookingResponse};

static API_ENDPOINT: &str = if cfg!(test) { 
    "https://private-anon-a28d0f1a72-transdirectapiv4.apiary-mock.com/api/" }
    else {
    "https://www.transdirect.com.au/api/"
};

/// Client object for interacting with the API
/// 
/// You are forced to use this as a proxy for authentication: it is essentially
/// a factory, but named for convenience (I think). A `Client` object can be
/// constructed the constructors [`new`], [`from_auth`], [`from_basic_auth`],
/// or [`from_apikey`].
/// 
/// Creates a synchronous (currently) client. Optimistically, we will implement
/// an async version through tokio, but I have absolutely no idea what that
/// entails.
/// 
/// # Examples
/// This example details the basic task of retrieving a quote from the
/// Transdirect API.
/// 
/// #[ignore]
/// ```
/// use transdirect::{TransdirectClient, BookingRequest};
/// ```
pub struct Client<'a> {
    authenticated: bool,
    restclient: BRestClient, // restson seems to have no advantages over reqwest
    sender: Option<&'a Account>,
    receiver: Option<&'a Account>,
}

impl<'a> Client<'a> {
    pub fn new() -> Self {
        Self {
            authenticated: false,
            restclient: RestClient::new_blocking(API_ENDPOINT)
                .expect("Should be a valid URL or connected to the internet"),
            sender: None,
            receiver: None
        }
    }
    
    pub fn from_auth(auth: AuthenticateWith) -> Result<Self, Error> {
        let mut newclient = Self::new();
        
        Self::auth(&mut newclient, auth)?;

        Ok(newclient)
    }
    
    pub fn from_basic(user: &str, password: &str) -> Result<Self, Error> {
        Self::from_auth(AuthenticateWith::Basic(user, password))
    }
    
    pub fn from_api_key(apikey: &str) -> Result<Self, Error> {
        Self::from_auth(AuthenticateWith::APIKey(apikey))
    }
    
    pub fn auth(&mut self, auth: AuthenticateWith) -> Result<(), Error> {
        use AuthenticateWith::*;

        match auth {
            Basic(user, pass) => self.restclient.set_auth(user, pass),
            APIKey(key) => self.restclient.set_header("Api-key", key).expect("Should be able to set Api-key header"),
        }
        
        match self.restclient.get::<_, Member>(()) {
            Ok(_) => {
                self.authenticated = true;
                Ok(())
            },
            Err(err) => Err(Error::HTTPError(err.to_string())),
        }
    }
    
    pub fn quotes<'b, T, U>(&self, request: &'b BookingRequest<T, U>) -> Result<BookingResponse<T, U>, Error>
    where T: Unsigned + Serialize + DeserializeOwned, U: Float + DeserializeOwned + Serialize {
        let response  = self
            .restclient
            .post_capture::<_, _, BookingResponse<T, U>>((), request)
            .map_err(|e| Error::HTTPError(e.to_string()))?
            .into_inner();
        
        Ok(response)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn should_generate_valid_client() {
    }
}