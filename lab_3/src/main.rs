use std::{env, fs};
use std::ops::Mul;
use byteorder::{BigEndian, ByteOrder};

fn extended_euclidean(p: u32, q: u32) -> (i128, i128) {
    if q == 0 {
        return (1, 0);
    }

    let (x1, y1) = extended_euclidean(q, p % q);
    let x = y1;
    let y = x1 - ((p / q) as i128 * y1);
    (x, y)
}

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 { return 0 }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
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


//367
//463
fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = &args[1];
    let p = args[2].parse::<u32>().expect("Incorrect p");
    let q = args[3].parse::<u32>().expect("Incorrect q");
    let path_read = &args[4];
    let path_to_write = &args[5];

    let n = p * q;

    if mode == "enc" {
        let mut converted_data = vec![];
        for i in fs::read(path_read).expect("File should exist and path should be right") {
            println!("{i}");
            converted_data.push(i as u32);
        }
        for i in 0..converted_data.len() {
            //println!("before {}", converted_data[i]);
            converted_data[i] = (converted_data[i] * converted_data[i]) % n;
            println!("after {}", converted_data[i]);
        }
        fs::write(path_to_write, transform_u32_to_array_of_u8(&converted_data)).expect("Error while writing to file");
    }
    else if mode == "dec" {
        let converted_data : Vec<u32> = read_and_convert_data(path_read);
        let mut data : Vec<u8> = Vec::with_capacity(converted_data.len());
        let (y_p, y_q) = extended_euclidean(p, q);
        for i in 0..converted_data.len() {

            let m_p = mod_pow(converted_data[i] as u64, ((p+1)/4) as u64, p as u64) as i128;
            let m_q = mod_pow(converted_data[i] as u64, ((q+1)/4) as u64, q as u64) as i128;
            let r_1 = {
                let value = y_p * p as i128 * m_q + y_q * q as i128 * m_p;
                ((value % n as i128) + n as i128) % n as i128
            };
            let r_2 = n as i128 - r_1;
            let r_3 = {
                let value = y_p * p as i128 * m_q - y_q * q as i128 * m_p;
                ((value % n as i128) + n as i128) % n as i128
            };
            let r_4 = n as i128 - r_3;
            //
            if r_1 <= u8::MAX as i128 { data.push(r_1 as u8) }
            else if r_2 <= u8::MAX as i128 { data.push(r_2 as u8) }
            else if r_3 <= u8::MAX as i128 { data.push(r_3 as u8) }
            else if r_4 <= u8::MAX as i128 { data.push(r_4 as u8) }
            println!("r1 {}", r_1);
            println!("r2 {}", r_2);
            println!("r3 {}", r_3);
            println!("r4 {}", r_4);

        }
        fs::write(path_to_write, data).expect("Error while writing to file");
    }
    else {
        panic!("incorrect input set 'enc' or 'dec' values")
    }
}
