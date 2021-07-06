use crate::message::*;

#[test]
fn test_verify_read() {
    assert!(verify_read([0x01, 0x03, 0x02, 0x00, 0xe3, 0xf9, 0xcd, 0x00]).is_ok());
    assert!(verify_read([0x01, 0x03, 0x02, 0x00, 0xe3, 0xf9, 0xcc, 0x00]).is_err());
}

#[test]
fn test_verify_write() {
    assert!(verify_write([0x01, 0x06, 0x00, 0x30, 0x00, 0x5a, 0x09, 0xfe]).is_ok());
    assert!(verify_write([0x01, 0x06, 0x00, 0x30, 0x00, 0x5b, 0x09, 0xfe]).is_err());
}
