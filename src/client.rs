use num_traits::{Float,Unsigned};
use serde::de::DeserializeOwned;
use serde::Serialize;
use restson::{RestClient, blocking::RestClient as BRestClient};

use crate::Error;
use crate::account::{Account,AuthenticateWith,Member};
use crate::booking::{BookingRequest,BookingResponse};

static API_ENDPOINT: &str = "https://www.transdirect.com.au/api/";

/// Client object for interacting with the API
/// 
/// 
pub struct Client<'a> {
    authenticated: bool,
    restclient: BRestClient,
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