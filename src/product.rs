/// Represents an item and its associated quantity
/// 
/// 
use std::default::Default;
use std::collections::HashMap;
use num_traits::{Float,Unsigned};
use serde_derive::{Serialize,Deserialize};
use serde::ser;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Product<T, U> where T: Unsigned, U: Float {
    pub quantity: T,
    pub weight: U, // Transdirect calculates weight in increments of 1kg
    #[serde(flatten)]
    pub dimensions: Dimensions<U>,
    pub description: String,
    pub id: Option<u32>, // Not necessary for Client to create
}

impl<T, U> Product<T, U>
where T: Unsigned + ser::Serialize + Default, U: Float + ser::Serialize + Default {
    /// Creates a new empty Product instance
    /// 
    /// This is a convenience function to create a valid Product fast
    /// 
    /// # Examples
    ///
    /// For example, there is an opaque function that modifies the product
    ///
    /// ```
    /// // use transdirect::prelude::Product;
    /// // 
    /// // fn deliver_extra(prod: &mut Product) -> Result<(), String> {
    /// //     
    /// // }
    /// // let m = Product::new();
    ///                
    ///
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct Dimensions<T> where T: Float {
    pub length: T,
    pub width: T,
    pub height: T,
}

impl<T> Dimensions<T> where T: Float + Default {
    pub fn new() -> Self {
        Default::default()
    }
    
    pub fn from_lwh(length: T, width: T, height: T) -> Self {
        Dimensions {
            length,
            width,
            height,
        }
    }
}

// impl<T> ser::Serialize for Dimensions<T> where T: Float + ser::Serialize {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where S: ser::Serializer {
//         let mut seq = serializer.serialize_seq(Some(self.len()))?;
//         for e in self {
//             seq.serialize_element(e)?
//         }
//         seq.end()
//     }
// }

/// A service provided by one of the companies listed by Transdirect.
/// It is put in the products file because it is a product provided by
/// external companies.
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
