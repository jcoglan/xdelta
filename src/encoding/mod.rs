mod tests;

pub mod copy {
    pub fn encode(offset: usize, size: usize) -> Vec<u8> {
        let value = (size << 32) | offset;
        let mut buffer = vec![0x80];

        for i in 0..7 {
            let byte = (value >> (8 * i)) & 0xff;

            if byte > 0 {
                buffer[0] |= 1 << i;
                buffer.push(byte as u8);
            }
        }

        buffer
    }
}

pub mod varint {
    const MASK: usize = 0x7f;
    const SHIFT: usize = 7;

    pub fn encode(mut value: usize) -> Vec<u8> {
        let mut buffer = Vec::new();

        while value > MASK {
            buffer.push(0x80 | low_bits(value));
            value >>= SHIFT;
        }

        buffer.push(low_bits(value));
        buffer
    }

    fn low_bits(value: usize) -> u8 {
        (value & MASK) as u8
    }
}
