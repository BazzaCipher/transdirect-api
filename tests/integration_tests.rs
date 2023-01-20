use transdirect::product::*;
use transdirect::booking::*;

#[test]
fn should_accept_unsigned_dimensions() {
    let _m = Dimensions { width: 1.0f64, height: 1.0, length: 1.0 };
}

#[test]
fn should_create_booking() {
    let m = Product { dimensions: Dimensions { length: 15.0, height: 15.0, width: 15.0}, quantity: 1u32, weight: 3.0, ..Product::new() };
    let b = BookingRequest { declared_value: 53.3,
        items: vec![m],
        ..BookingRequest::new()
    };
}