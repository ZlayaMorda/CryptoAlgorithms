use byteorder::{BigEndian, ByteOrder};

use std::{env, fs};
use std::time::SystemTime;

const S_BOX: [[u8; 16]; 16] = [
    [0xB1, 0x94, 0xBA, 0xC8, 0x0A, 0x08, 0xF5, 0x3B, 0x36, 0x6D, 0x00, 0x8E, 0x58, 0x4A, 0x5D, 0xE4],
    [0x85, 0x04, 0xFA, 0x9D, 0x1B, 0xB6, 0xC7, 0xAC, 0x25, 0x2E, 0x72, 0xC2, 0x02, 0xFD, 0xCE, 0x0D],
    [0x5B, 0xE3, 0xD6, 0x12, 0x17, 0xB9, 0x61, 0x81, 0xFE, 0x67, 0x86, 0xAD, 0x71, 0x6B, 0x89, 0x0B],
    [0x5C, 0xB0, 0xC0, 0xFF, 0x33, 0xC3, 0x56, 0xB8, 0x35, 0xC4, 0x05, 0xAE, 0xD8, 0xE0, 0x7F, 0x99],
    [0xE1, 0x2B, 0xDC, 0x1A, 0xE2, 0x82, 0x57, 0xEC, 0x70, 0x3F, 0xCC, 0xF0, 0x95, 0xEE, 0x8D, 0xF1],
    [0xC1, 0xAB, 0x76, 0x38, 0x9F, 0xE6, 0x78, 0xCA, 0xF7, 0xC6, 0xF8, 0x60, 0xD5, 0xBB, 0x9C, 0x4F],
    [0xF3, 0x3C, 0x65, 0x7B, 0x63, 0x7C, 0x30, 0x6A, 0xDD, 0x4E, 0xA7, 0x79, 0x9E, 0xB2, 0x3D, 0x31],
    [0x3E, 0x98, 0xB5, 0x6E, 0x27, 0xD3, 0xBC, 0xCF, 0x59, 0x1E, 0x18, 0x1F, 0x4C, 0x5A, 0xB7, 0x93],
    [0xE9, 0xDE, 0xE7, 0x2C, 0x8F, 0x0C, 0x0F, 0xA6, 0x2D, 0xDB, 0x49, 0xF4, 0x6F, 0x73, 0x96, 0x47],
    [0x06, 0x07, 0x53, 0x16, 0xED, 0x24, 0x7A, 0x37, 0x39, 0xCB, 0xA3, 0x83, 0x03, 0xA9, 0x8B, 0xF6],
    [0x92, 0xBD, 0x9B, 0x1C, 0xE5, 0xD1, 0x41, 0x01, 0x54, 0x45, 0xFB, 0xC9, 0x5E, 0x4D, 0x0E, 0xF2],
    [0x68, 0x20, 0x80, 0xAA, 0x22, 0x7D, 0x64, 0x2F, 0x26, 0x87, 0xF9, 0x34, 0x90, 0x40, 0x55, 0x11],
    [0xBE, 0x32, 0x97, 0x13, 0x43, 0xFC, 0x9A, 0x48, 0xA0, 0x2A, 0x88, 0x5F, 0x19, 0x4B, 0x09, 0xA1],
    [0x7E, 0xCD, 0xA4, 0xD0, 0x15, 0x44, 0xAF, 0x8C, 0xA5, 0x84, 0x50, 0xBF, 0x66, 0xD2, 0xE8, 0x8A],
    [0xA2, 0xD7, 0x46, 0x52, 0x42, 0xA8, 0xDF, 0xB3, 0x69, 0x74, 0xC5, 0x51, 0xEB, 0x23, 0x29, 0x21],
    [0xD4, 0xEF, 0xD9, 0xB4, 0x3A, 0x62, 0x28, 0x75, 0x91, 0x14, 0x10, 0xEA, 0x77, 0x6C, 0xDA, 0x1D],
];

fn RotHi(u: u32, k: u8) -> u32 {
    (u << k) | (u >> (32 - k))
}

fn sum_u32(u_1: u32, u_2: u32) -> u32 {
    (u_1 as u64 + u_2 as u64) as u32
}

fn sub_u32(u_1: u32, u_2: u32) -> u32 {
    let sub = u_1 as i128 - u_2 as i128;
    sub as u32
}
fn set_8_bits(u32_val: &mut u32, u8_val: u8, position: u8) {
    *u32_val &= !(0b11111111 << position);
    *u32_val |= (u8_val as u32) << position;
}

fn get_8_bits(u32_val: &u32, position: u8) -> u32 {
        let mask = 0b11111111 << position;
        (u32_val & mask) >> position
}

fn get_4_bits(u32_val: u8, position: u8) -> u8 {
        let mask = 0b1111 << position;
        (u32_val & mask) >> position
}

fn Gr(block: &mut u32, k: u8) -> u32 {
    for i in 0..4 {
        let byte = get_8_bits(block, i * 8) as u8;
        set_8_bits(
            block,
            S_BOX[get_4_bits(byte, 0) as usize][get_4_bits(byte, 4) as usize],
            i * 8
        )
    }
    RotHi(*block, k)
}

fn encrypt_block(mut a: u32, mut b: u32, mut c: u32, mut d: u32, key: &[u32]) -> (u32, u32, u32, u32) {
    for i in 1..=8 {
        // step 1
        b ^= Gr(&mut sum_u32(a , key[(7*i - 7) % 8]), 5);
        //step 2
        c ^= Gr(&mut sum_u32(d, key[(7*i - 6) % 8]), 21);
        //step 3
        a = sub_u32(a, Gr(&mut sum_u32(b , key[(7*i - 5) % 8]), 13));
        //step 4
        let e = Gr( &mut (sum_u32(sum_u32(b , c) , key[(7*i - 4) % 8])),21) ^ (i as u32);
        //step 5
        b = sum_u32(b, e);
        //step 6
        c = sub_u32(c, e);
        //step 7
        d = sum_u32(d, Gr(&mut sum_u32(c, key[(7*i - 3) % 8]), 13));
        //step 8
        b ^= Gr(&mut sum_u32(a, key[(7*i - 2) % 8]), 21);
        //step 9
        c ^= Gr(&mut sum_u32(d, key[(7*i - 1) % 8]), 5);
    }
    (d, c, b, a)
}

fn decrypt_block(mut a: u32, mut b: u32, mut c: u32, mut d: u32, key: &[u32]) -> (u32, u32, u32, u32) {
    for i in (1..=8).rev() {
        // step 1
        b ^= Gr(&mut sum_u32(a , key[(7*i - 1) % 8]), 5);
        //step 2
        c ^= Gr(&mut sum_u32(d, key[(7*i - 2) % 8]), 21);
        //step 3
        a = sub_u32(a, Gr(&mut sum_u32(b , key[(7*i - 3) % 8]), 13));
        //step 4
        let e = Gr( &mut (sum_u32(sum_u32(b , c) , key[(7*i - 4) % 8])),21) ^ (i as u32);
        //step 5
        b = sum_u32(b, e);
        //step 6
        c = sub_u32(c, e);
        //step 7
        d = sum_u32(d, Gr(&mut sum_u32(c, key[(7*i - 5) % 8]), 13));
        //step 8
        b ^= Gr(&mut sum_u32(a, key[(7*i - 6) % 8]), 21);
        //step 9
        c ^= Gr(&mut sum_u32(d, key[(7*i - 7) % 8]), 5);
    }
    (d, c, b, a)
}

fn create_sync_sending() -> u128 {
    let mut time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
    time * time
}

fn counter_enc(sync_sending: &mut u128, converted_data: &mut Vec<u32>, converted_key: & Vec<u32>) {
    for i in 0..converted_data.len()/4 {
        *sync_sending += 1;
        let mut sync_sending_u32 = convert_128_to_32(*sync_sending);
        (
            sync_sending_u32[0],
            sync_sending_u32[1],
            sync_sending_u32[2],
            sync_sending_u32[3]) = encrypt_block(
            sync_sending_u32[0],
            sync_sending_u32[1],
            sync_sending_u32[2],
            sync_sending_u32[3],
            &converted_key
        );

        (
            converted_data[4*i],
            converted_data[4*i+1],
            converted_data[4*i+2],
            converted_data[4*i+3]) = (
            converted_data[4*i] ^ sync_sending_u32[0],
            converted_data[4*i+1] ^ sync_sending_u32[1],
            converted_data[4*i+2] ^ sync_sending_u32[2],
            converted_data[4*i+3] ^ sync_sending_u32[3]
        )
    }
}

fn convert_128_to_32(u: u128) -> Vec<u32> {
    let mut x_u32 = Vec::<u32>::with_capacity(4);
    x_u32.push((u >> 96) as u32);
    x_u32.push((u >> 64) as u32);
    x_u32.push((u >> 32) as u32);
    x_u32.push(u as u32);
    x_u32
}

fn convert_to_u32(u8_block: &[u8]) -> u32 {
    match u8_block.len() {
        4 => BigEndian::read_u32(u8_block),
        3 => BigEndian::read_u32(&[u8_block[0], u8_block[1], u8_block[2], 0_u8]),
        2 => BigEndian::read_u32(&[u8_block[0], u8_block[1], 0_u8, 0_u8]),
        1 => BigEndian::read_u32(&[u8_block[0], 0_u8, 0_u8, 0_u8]),
        _value => panic!("Inaccurate block size {_value}",)
    }
}

fn convert_to_u128(u32_block: &[u32]) -> u128 {
    match u32_block.len() {
        4 => BigEndian::read_u128(&transform_u32_to_array_of_u8(u32_block)),
        3 => BigEndian::read_u128(&transform_u32_to_array_of_u8(&[u32_block[0], u32_block[1], u32_block[2], 0_u32])),
        2 => BigEndian::read_u128(&transform_u32_to_array_of_u8(&[u32_block[0], u32_block[1], 0_u32, 0_u32])),
        1 => BigEndian::read_u128(&transform_u32_to_array_of_u8(&[u32_block[0], 0_u32, 0_u32, 0_u32])),
        _value => panic!("Inaccurate block size {_value}",)
    }
}

/// Get data and convert to vec of u32
fn read_and_convert_data(path: &str) -> Vec<u32> {
    let data = fs::read(path).expect("File should exist and path should be right");
    let len_in_blocks = data.len() / 4;
    let mut converted_data:Vec<u32> = Vec::with_capacity(len_in_blocks + 1);
    for index in 0..=len_in_blocks {
        if index == len_in_blocks {
            if data[index*4..].len().eq(&0_usize) { break }
            converted_data.push(convert_to_u32(&data[index*4..]));
            break;
        }
        converted_data.push(convert_to_u32(&data[index*4..=index*4+3]));
    }
    converted_data
}

fn transform_u32_to_array_of_u8(x:&[u32]) -> Vec<u8> {
    let mut x_u8 = Vec::with_capacity(x.len() * 4);
    for x in x {
        let b1 : u8 = ((x >> 24) & 0xff) as u8;
        let b2 : u8 = ((x >> 16) & 0xff) as u8;
        let b3 : u8 = ((x >> 8) & 0xff) as u8;
        let b4 : u8 = (x & 0xff) as u8;
        x_u8.push(b1);
        x_u8.push(b2);
        x_u8.push(b3);
        x_u8.push(b4);
    }
    x_u8
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = &args[1];
    let key = args[2].as_bytes();
    let path = &args[3];
    let path_to_write = &args[4];
    let path_sync = &args[5];

    // Check key size and convert to vec of u32
    if key.len() != 32 { panic!("Inaccurate key size") }
    let mut converted_key:Vec<u32> = Vec::with_capacity(8);
    for index in 0..8_usize {
        if index == 7 {
            converted_key.push(convert_to_u32(&key[index*4..]));
            break;
        }
        converted_key.push(convert_to_u32(&key[index*4..=index*4+3]));
    }

    let mut converted_data = read_and_convert_data(path);

    match converted_data.len() % 4 {
            0 => (),
            1 => {
                converted_data.push(0_u32);
                converted_data.push(0_u32);
                converted_data.push(0_u32)
            }
            2 => {
                converted_data.push(0_u32);
                converted_data.push(0_u32)
            },
            3 => {
                converted_data.push(0_u32)
        },
        _value => panic!("Unreachable len mod: {_value}",)
    }

    if mode == "enc" {
        let mut sync_sending = create_sync_sending();
        println!("{:?}", sync_sending);
        fs::write(path_sync, transform_u32_to_array_of_u8(&convert_128_to_32(sync_sending))).expect("Error while writing to file");
        let sync_sending_enc = encrypt_block(
            convert_128_to_32(sync_sending)[0],
            convert_128_to_32(sync_sending)[1],
            convert_128_to_32(sync_sending)[2],
            convert_128_to_32(sync_sending)[3],
            &converted_key
        );
        sync_sending = convert_to_u128(&[sync_sending_enc.0, sync_sending_enc.1, sync_sending_enc.2, sync_sending_enc.3]);

        counter_enc(&mut sync_sending, &mut converted_data, &converted_key)
    }
    else if mode == "dec" {
        let sync_sending= read_and_convert_data(path_sync);
        let sync_sending_enc = encrypt_block(
            sync_sending[0],
            sync_sending[1],
            sync_sending[2],
            sync_sending[3],
            &converted_key
        );
        let mut sync_sending = convert_to_u128(&[sync_sending_enc.0, sync_sending_enc.1, sync_sending_enc.2, sync_sending_enc.3]);

        counter_enc(&mut sync_sending, &mut converted_data, &converted_key)
    }
    else {
        panic!("incorrect input set 'enc' or 'dec' values")
    }

    fs::write(path_to_write, transform_u32_to_array_of_u8(&converted_data)).expect("Error while writing to file");
}

#[cfg(test)]
mod tests {

}
