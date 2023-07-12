pub fn sext(value: u32, len: usize) -> u32 {
    let bit_len = std::mem::size_of::<u32>() << 3;
    assert!(len > 0 && len <= bit_len);
    if len == bit_len {
        return value;
    }
    let sign = value >> (len - 1) as u32 & 0x1;
    let mask = ((1 as u32) << (len as u32)) - 1 as u32;
    if sign == 0 {
        value & mask
    } else {
        let high = (((1 as u32) << (bit_len as u32 - len as u32)) - 1 as u32) << (len as u32);
        value & mask | high
    }
}
