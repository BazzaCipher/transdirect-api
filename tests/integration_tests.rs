use transdirect::product::*;

#[test]
fn should_accept_unsigned_dimensions() {
    let m = Dimensions::<u32> { width: 1, height: 1, length: 1 };
}