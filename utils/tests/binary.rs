use utils::binary::{binary_string, get_bit, set_bit};

#[test]
fn binary_string_success() {
    assert_eq!("0000000000000000", binary_string(0b0000_0000_0000_0000));
    assert_eq!("0000000000000001", binary_string(0b0000_0000_0000_0001));
    assert_eq!("0000000000001000", binary_string(0b0000_0000_0000_1000));
    assert_eq!("0000000010000000", binary_string(0b0000_0000_1000_0000));
    assert_eq!("0000100000000000", binary_string(0b0000_1000_0000_0000));
    assert_eq!("1000000000000000", binary_string(0b1000_0000_0000_0000));
}

#[test]
fn get_bit_success() {
    assert_eq!(0, get_bit(0b0000_0000_0000_0000, 1));
    assert_eq!(1, get_bit(0b0000_0000_0000_0001, 1));
    assert_eq!(1, get_bit(0b0000_0000_0000_1000, 4));
    assert_eq!(0, get_bit(0b1111_0111_1111_1111, 12));
}

#[test]
fn set_bit_success() {
    assert_eq!(
        0b0000_0000_0000_0001,
        set_bit(0b0000_0000_0000_0000, 1, true)
    );
    assert_eq!(
        0b0000_1000_0000_0000,
        set_bit(0b0000_0000_0000_0000, 12, true)
    );
    assert_eq!(
        0b1111_1111_1111_0111,
        set_bit(0b1111_1111_1111_1111, 4, false)
    );
    assert_eq!(
        0b1111_1111_0111_1111,
        set_bit(0b1111_1111_1111_1111, 8, false)
    );
}
