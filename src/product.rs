/// Represents an item and its associated quantity
/// 
/// 
use std::default::Default;
use std::collections::HashMap;
use num_traits::{Float,Unsigned};
use serde_derive::{Serialize,Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Product<T> where T: Unsigned {
    pub quantity: T,
    pub weight: T, // Transdirect calculates weight in increments of 1kg
    pub dimensions: Dimensions<T>,
    pub description: String,
    id: Option<u32>,
}

impl<R> Product<R>
where R: Unsigned + Default {
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
    
impl<T> Product<T>
where T: Unsigned + Default {
    // pub fn is_valid(&self) -> bool {
    //     self.dimensions.is_valid()
    // }
    
    pub fn from_dimensions() {}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Dimensions<T> where T: Unsigned {
    pub length: T,
    pub width: T,
    pub height: T,
}
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
