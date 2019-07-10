use super::{copy, varint};

macro_rules! encode {
    ($mod:ident, ($( $value:expr ),+), [$( $byte:expr ),+]) => {
        assert_eq!($mod::encode($( $value ),+), vec![$( $byte ),+]);
    };
    ($mod:ident, $value:expr, [$( $byte:expr ),+]) => {
        assert_eq!($mod::encode($value), vec![$( $byte ),+]);
    };
}

#[test]
fn test_copy() {
    encode!(copy, (0, 0), [0b1_000_0000]);
    encode!(copy, (0x12, 0xab), [0b1_001_0001, 0x12, 0xab]);

    encode!(copy, (0x120056, 0xab), [0b1_001_0101, 0x56, 0x12, 0xab]);
    encode!(copy, (0x12000078, 0xab), [0b1_001_1001, 0x78, 0x12, 0xab]);
    encode!(copy, (0x12005600, 0xab), [0b1_001_1010, 0x56, 0x12, 0xab]);

    encode!(copy, (0x12, 0xab00ef), [0b1_101_0001, 0x12, 0xef, 0xab]);
    encode!(copy, (0x12, 0xabcd00), [0b1_110_0001, 0x12, 0xcd, 0xab]);

    encode!(
        copy,
        (0x12345678, 0xabcdef),
        [0b1_111_1111, 0x78, 0x56, 0x34, 0x12, 0xef, 0xcd, 0xab]
    );

    encode!(
        copy,
        (0x123456, 0xab),
        [0b1_001_0111, 0x56, 0x34, 0x12, 0xab]
    );
}

#[test]
fn test_varint() {
    encode!(varint, 0, [0]);

    encode!(varint, 127, [0x7f]);
    encode!(varint, 128, [0x80, 0x01]);

    encode!(varint, 129, [0x81, 0x01]);
    encode!(varint, 255, [0xff, 0x01]);
    encode!(varint, 256, [0x80, 0x02]);

    encode!(varint, 16_383, [0xff, 0x7f]);
    encode!(varint, 16_384, [0x80, 0x80, 0x01]);

    encode!(varint, 2_097_151, [0xff, 0xff, 0x7f]);
    encode!(varint, 2_097_152, [0x80, 0x80, 0x80, 0x01]);
}
