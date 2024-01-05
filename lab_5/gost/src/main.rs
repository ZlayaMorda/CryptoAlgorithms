mod encrypt;

use std::{env, fs};
use std::fs::{File, OpenOptions};
use std::io::Write;
use byteorder::{BigEndian, ByteOrder};
use sha1::{Sha1, Digest};

const C: [[u32; 8]; 3] = [
    [0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32],
    [0xff00ffff_u32, 0x000000ff_u32, 0xff0000ff_u32, 0x00ffff00_u32, 0x00ff00ff_u32, 0x00ff00ff_u32, 0xff00ff00_u32, 0xff00ff00_u32],
    [0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32]
];

fn a_y(data: &[u32]) -> Vec<u32> {
    vec![data[7] ^ data[5], data[6] ^ data[4], data[0], data[1], data[2], data[3], data[4], data[5]]
}

fn p_y(data: &[u32]) -> Vec<u32> {
    let mut new_data: Vec<u8> = Vec::with_capacity(32);
    for i in 0..32 {
        new_data.push(0);
    }
    let data_u8 = transform_u32_to_array_of_u8(data);

    for temp in 1..=32 {
        let k = temp % 8;
        new_data[temp-1] = data_u8[((temp - k) + k - 1)];
    }
    let mut data_return = Vec::with_capacity(8);
    for i in 0..8 {
        data_return.push(BigEndian::read_u32(&[new_data[i*4], new_data[i*4+1], new_data[i*4+2], new_data[i*4+3]]));
    }
    data_return
}

fn xor_u32(H_in: &[u32], m: &[u32]) -> Vec<u32> {
    let mut return_value = Vec::with_capacity(8);
    for i in 0..8 {
        return_value.push(H_in[i] ^ m[i]);
    }
    return_value
}

fn xor_u16(H_in: &mut [u16], m: &[u16]) {
    for i in 0..16 {
        H_in[i] ^= m[i];
    }
}


fn generate_keys(H_in: &[u32], m: &[u32]) -> Vec<Vec<u32>> {
    let mut keys: Vec<Vec<u32>> = Vec::with_capacity(4);
    let mut U = H_in.to_vec();
    let mut V = m.to_vec();
    for i in 0..=3 {
        match i {
            0 => {
                keys.push(p_y(&xor_u32(&U, &V)));
            }
            value => {
                U = xor_u32(&a_y(&U), &C[value-1]);
                V = a_y(&a_y(&V));
                keys.push(p_y(&xor_u32(&U, &V)))
            }

        }
    }
    vec![keys[0].clone(), keys[1].clone(), keys[2].clone(), keys[3].clone()]
}

fn psi(data: &mut Vec<u16>) {
    let temp = data[15] ^ data[14] ^ data[13] ^ data[12] ^ data[3] ^ data[0];
    for i in 1..=15 {
        data[i] = data[i-1]
    }
    data[0] = temp;
}

fn hash_f(H_in: Vec<u32>, m: Vec<u32>, keys: Vec<Vec<u32>>) -> Vec<u32> {
    let save_H = transform_u32_to_array_of_u16(&H_in);
    let mut s: Vec<u32> = Vec::with_capacity(8);
    for i in 0..8 { s.push(0); }
    (s[0], s[1]) = encrypt::encrypt::encrypt_block(H_in[0], H_in[1], &keys[0]);
    (s[2], s[3]) = encrypt::encrypt::encrypt_block(H_in[2], H_in[3], &keys[1]);
    (s[4], s[5]) = encrypt::encrypt::encrypt_block(H_in[4], H_in[5], &keys[2]);
    (s[6], s[7]) = encrypt::encrypt::encrypt_block(H_in[6], H_in[7], &keys[3]);

    let mut data_u16 = transform_u32_to_array_of_u16(&s);
    let m_u16 = transform_u32_to_array_of_u16(&m);

    for i in 0..12 {
        psi(&mut data_u16);
    }
    xor_u16(&mut data_u16, &m_u16);
    xor_u16(&mut data_u16, &save_H);
    for i in 0..61 {
        psi(&mut data_u16);
    }

    convert_u16_to_u32(data_u16)
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let path_to_write = &args[2];

    let (mut converted_data, last_size) = read_and_convert_data(path);
    let data_len = converted_data.len() * 32 - (32 - last_size);
    let temp_len = converted_data.len();
    inline(&mut converted_data, temp_len % 8);

    let mut H_hash = vec![0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32];

    for i in 0..(temp_len / 8) {
        let mut m = Vec::with_capacity(8);
        for j in 0..8 { m.push(converted_data[8*i+j]) }
        let keys = generate_keys(&H_hash, &m);
        H_hash = hash_f(H_hash, m, keys);
    }

    let L = transform_u64_to_array_of_u32(data_len as u64);
    let keys = generate_keys(&H_hash, &L);
    H_hash = hash_f(H_hash, L, keys);
    let control_sum = control_sum(converted_data);
    for i in 0..H_hash.len() {
        H_hash[i] ^= control_sum[i];
    }
    File::create(path_to_write).unwrap();
    let mut data_file = OpenOptions::new()
        .append(true)
        .open(path_to_write)
        .expect("cannot open file");
    for i in transform_u32_to_array_of_u8(&H_hash) {
            data_file.write(format!("{:02X}", i).as_ref()).expect("Error while writing to file");
        }
}

fn inline(data: &mut Vec<u32>, count: usize) {
    if count == 0 { }
    else {
        for i in 0..(8 - count) {
            data.push(0_u32);
        }
    }
}

fn control_sum(data: Vec<u32>) -> Vec<u32> {
    let mut return_value = vec![0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32];
    for i in 0..(data.len() / 8) {
        let mut m = Vec::with_capacity(8);
        for j in 0..8 { m.push(data[8*i+j]) }
        return_value = xor_u32(&return_value, &m);
    }
    return_value
}

fn convert_u32_to_u64(u32_block: &[u32]) -> u64 {
    BigEndian::read_u64(&transform_u32_to_array_of_u8(&u32_block))
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

fn convert_u16_to_u32(data: Vec<u16>) -> Vec<u32> {
    let mut return_data = Vec::with_capacity(8);
    for i in 0..8 {
        return_data.push(BigEndian::read_u32(&transform_u16_to_array_of_u8(&[data[i*2], data[i*2+1]])))
    }
    return_data
}

fn transform_u32_to_array_of_u16(x:&[u32]) -> Vec<u16> {
    let mut x_u16 = Vec::with_capacity(x.len() * 2);
    for x in x {
        let b1 : u16 = ((x >> 16) & 0xffff) as u16;
        let b2 : u16 = (x & 0xffff) as u16;
        x_u16.push(b1);
        x_u16.push(b2);
    }
    x_u16
}

fn transform_u64_to_array_of_u32(x:u64) -> Vec<u32> {
    let mut x_u32 = Vec::with_capacity(8);
    let b1 : u32 = ((x >> 32) & 0xffffffff) as u32;
    let b2 : u32 = (x & 0xffffffff) as u32;
    x_u32.push(b1);
    x_u32.push(b2);
    for i in 0..6 {
        x_u32.push(0_u32)
    }
    x_u32
}

fn transform_u16_to_array_of_u8(x:&[u16]) -> Vec<u8> {
    let mut x_u8 = Vec::with_capacity(x.len() * 2);
    for x in x {
        let b1 : u8 = ((x >> 8) & 0xff) as u8;
        let b2 : u8 = (x & 0xff) as u8;
        x_u8.push(b1);
        x_u8.push(b2);
    }
    x_u8
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

fn read_and_convert_data(path: &str) -> (Vec<u32>, usize) {
    let data = fs::read(path).expect("File should exist and path should be right");
    let len_in_blocks = data.len() / 4;
    let mut converted_data:Vec<u32> = Vec::with_capacity(len_in_blocks + 1);
    let mut last_size = 0;
    for index in 0..=len_in_blocks {
        if index == len_in_blocks {
            if data[index*4..].len().eq(&0_usize) {
                last_size = 32;
                break
            }
            last_size = data[index*4..].len() * 8;
            converted_data.push(convert_to_u32(&data[index*4..]));
            break;
        }
        converted_data.push(convert_to_u32(&data[index*4..=index*4+3]));
    }
    (converted_data, last_size)
}