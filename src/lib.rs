type BlockHash = Vec<u8>;


//u8 is equal to hexadecimal
pub fn u32_bits_to_4bytes (u: &u32) -> [u8; 4] { //8 bits per byte 32 bits/8 is 4 bytes
    [
        (u >> 8 * 0x0) as u8, //bitwise shift 8 for a byte
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,
    ]
}

pub fn u64_bits_to_8bytes (u: &u64) -> [u8; 8] { // 64 bits/8 is 8 bytes
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,
        (u >> 8 * 0x4) as u8,
        (u >> 8 * 0x5) as u8,
        (u >> 8 * 0x6) as u8,
        (u >> 8 * 0x7) as u8,
    ]
}

pub fn u128_bits_to_16bytes (u: &u128) -> [u8; 16] { // 128 bits/8 is 16 bytes
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,
        (u >> 8 * 0x4) as u8,
        (u >> 8 * 0x5) as u8,
        (u >> 8 * 0x6) as u8,
        (u >> 8 * 0x7) as u8,
        (u >> 8 * 0x8) as u8,
        (u >> 8 * 0x9) as u8,
        (u >> 8 * 0xa) as u8,
        (u >> 8 * 0xb) as u8,
        (u >> 8 * 0xc) as u8,
        (u >> 8 * 0xd) as u8,
        (u >> 8 * 0xe) as u8,
        (u >> 8 * 0xf) as u8,
    ]
}




mod block;
pub use crate::block::Block;