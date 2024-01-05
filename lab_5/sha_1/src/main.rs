use std::{env, fs};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use byteorder::{BigEndian, ByteOrder};

fn RotHi(u: u32, k: u8) -> u32 {
    (u << k) | (u >> (32 - k))
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

fn convert_64_to_32(u: u64) -> Vec<u32> {
    let mut x_u32 = Vec::<u32>::with_capacity(2);
    x_u32.push((u >> 32) as u32);
    x_u32.push(u as u32);
    x_u32
}

pub fn inline(data: &mut Vec<u32>, last_size: usize) {
    let data_len = data.len();
    let mut count = data_len;

    let all_len = (count * 32) - (32 - last_size);
    println!("{}", all_len);
    let len_u32 = convert_64_to_32(all_len as u64);

    if last_size != 32 && last_size != 0 {
        data[data_len-1] &= !(0b1 << (32 - last_size));
        data[data_len-1] |= (0b1) << (32 - last_size);
    }
    else if count != 0 {
        data.push(2147483648);
        count += 1;
    }

    if all_len < 448 {
        if (count * 32) < 448 {
            for i in 0..(448 - (count * 32)) / 32 {
                data.push(0_u32)
            }
        }
        data.push(len_u32[0]);
        data.push(len_u32[1]);
    }
    else {
        if count != 16 && count != 0 {
            data.push(0_u32)
        }
    }

    for i in 0..14 {
        data.push(0_u32)
    }
    data.push(len_u32[0]);
    data.push(len_u32[1]);
}

pub fn main_round(data_block: &[u32]) -> Vec<u32> {
    let mut values: Vec<u32> = vec!(0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0);
    let mut new_block:Vec<u32> = Vec::with_capacity(80);

    for i in 0..80 {
        if i < 16 {
            new_block.push(data_block[i])
        }
        else {
            new_block.push(
                RotHi(new_block[i-3] ^ new_block[i-8] ^ new_block[i-14] ^ new_block[i-16], 1)
            )
        }
    }

    for t in 0..80 {
        values[0] = RotHi(values[0], 5);
        let temp: u32;
        if t < 20 {
            temp = (values[0] as u64 + ((values[1] & values[2]) | (!values[1] & values[3])) as u64 + 0x5A827999 + values[4] as u64 + new_block[t] as u64) as u32
        }
        else if t < 40 {
            temp = (values[0] as u64 + (values[1] ^ values[2] ^ values[3]) as u64  + 0x6ED9EBA1 + values[4] as u64 + new_block[t] as u64) as u32
        }
        else if t < 60 {
            temp = (values[0] as u64 + ((values[1] & values[2]) | (values[1] & values[3]) | (values[2] & values[3])) as u64 + 0x8F1BBCDC + values[4] as u64 + new_block[t] as u64) as u32
        }
        else {
            temp = (values[0] as u64 + (values[1] ^ values[2] ^ values[3]) as u64 + 0xCA62C1D6 + values[4] as u64 + new_block[t] as u64) as u32
        }

        values[4] = values[3];
        values[3] = values[2];
        values[2] = RotHi(values[1], 30);
        values[1] = values[0];
        values[0] = temp;
    }

    values
}

pub fn print_data(data: &[u32]) {
    let mut counter = 0;
    for i in data {
        println!("{counter} : {:b}", i);
        counter += 1;
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let path_to_write = &args[2];

    let (mut converted_data, last_size) = read_and_convert_data(path);

    inline(&mut converted_data, last_size);

    File::create(path_to_write).unwrap();
    let mut data_file = OpenOptions::new()
        .append(true)
        .open(path_to_write)
        .expect("cannot open file");
    for i in 0..(converted_data.len() / 16) {
        let hash = main_round(&converted_data[i*16..(i+1)*16]);
        for i in transform_u32_to_array_of_u8(&hash) {
            //let str =
            data_file.write(format!("{:02X}", i).as_ref()).expect("Error while writing to file");
            println!("{:02X}", i)
        }
    }
}
