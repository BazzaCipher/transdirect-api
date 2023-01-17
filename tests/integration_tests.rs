use transdirect::product::*;
use transdirect::booking::*;

#[test]
fn should_accept_unsigned_dimensions() {
    let _m = Dimensions::<u32> { width: 1, height: 1, length: 1 };
}

#[test]
fn should_create_booking() {
    let m = Product::from_dimensions_quantity(Dimensions::from_lwh(15u32, 15, 15), 3);
    let b = BookingRequest { declared_value: 53.3,
        items: vec![m],
        ..BookingRequest::new()
    };
}