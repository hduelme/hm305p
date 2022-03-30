use crate::voltage::*;

#[test]
fn test_get() {
    assert_eq!(2270, get([1, 3, 2, 0, 227, 249, 205, 0]));
}

#[test]
fn test_set() {
    helper_set(
        1800,
        [0x01, 0x06, 0x00, 0x30, 0x00, 0x00, 0x00, 0x00],
        [0x01, 0x06, 0x00, 0x30, 0x00, 0xb4, 0x00, 0x00],
    );

    helper_set(
        1800,
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        [0x00, 0x00, 0x00, 0x00, 0x00, 0xb4, 0x00, 0x00],
    );

    helper_set(
        2400,
        [0x01, 0x06, 0x00, 0x30, 0x00, 0x00, 0x00, 0x00],
        [0x01, 0x06, 0x00, 0x30, 0x00, 0xf0, 0x00, 0x00],
    );

    helper_set(
        2800,
        [0x01, 0x06, 0x00, 0x30, 0x00, 0x00, 0x00, 0x00],
        [0x01, 0x06, 0x00, 0x30, 0x01, 0x18, 0x00, 0x00],
    );
}

fn helper_set(voltage_mv: u16, input: [u8; MESSAGE_LENGTH], output: [u8; MESSAGE_LENGTH]) {
    let mut message = input;
    set(voltage_mv, &mut message);
    assert_eq!(output, message);
}
