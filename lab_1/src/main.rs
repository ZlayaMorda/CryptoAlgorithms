use std::{env, fs, thread};
use byteorder::{BigEndian, ByteOrder};

const S_BOX: [[u8; 16]; 8] = [
    [0x01, 0x0B, 0x0C, 0x02, 0x09, 0x0D, 0x00, 0x0F, 0x04, 0x05, 0x08, 0x0E, 0x0A, 0x07, 0x06, 0x03],
    [0x00, 0x01, 0x07, 0x0D, 0x0B, 0x04, 0x05, 0x02, 0x08, 0x0E, 0x0F, 0x0C, 0x09, 0x0A, 0x06, 0x03],
    [0x08, 0x02, 0x05, 0x00, 0x04, 0x09, 0x0F, 0x0A, 0x03, 0x07, 0x0C, 0x0D, 0x06, 0x0E, 0x01, 0x0B],
    [0x03, 0x06, 0x00, 0x01, 0x05, 0x0D, 0x0A, 0x08, 0x0B, 0x02, 0x09, 0x07, 0x0E, 0x0F, 0x0C, 0x04],
    [0x08, 0x0D, 0x0B, 0x00, 0x04, 0x05, 0x01, 0x02, 0x09, 0x03, 0x0C, 0x0E, 0x06, 0x0F, 0x0A, 0x07],
    [0x0C, 0x09, 0x0B, 0x01, 0x08, 0x0E, 0x02, 0x04, 0x07, 0x03, 0x06, 0x05, 0x0A, 0x00, 0x0F, 0x0D],
    [0x0A, 0x09, 0x06, 0x08, 0x0D, 0x0E, 0x02, 0x00, 0x0F, 0x03, 0x05, 0x0B, 0x04, 0x01, 0x0C, 0x07],
    [0x07, 0x04, 0x00, 0x05, 0x0A, 0x02, 0x0F, 0x0E, 0x0C, 0x06, 0x01, 0x0B, 0x0D, 0x09, 0x03, 0x08],
];

fn encrypt_block(mut block_low: u32,mut block_high: u32, key: &[u32]) -> (u32, u32) {
    for index in 0..24 {
        (block_low, block_high) = main_round(block_low, block_high, key[index%8]);
    }
    for index in (0..8).rev(){
        (block_low, block_high) = main_round(block_low, block_high, key[index%8]);
    }
    (block_high, block_low)
}

fn decrypt_block(mut block_low: u32,mut block_high: u32, key: &[u32]) -> (u32, u32) {
    for index in 0..8 {
        (block_low, block_high) = main_round(block_low, block_high, key[index%8]);
    }
    for index in (0..24).rev() {
        (block_low, block_high) = main_round(block_low, block_high, key[index%8]);
    }

    (block_high, block_low)
}

fn main_round(mut block_low: u32, block_high: u32, key: u32) -> (u32, u32) {
    let block_low_clone = block_low;
    block_low ^= key;
    for i in 0..8 {
        let value_to_change = get_4_bits(&block_low, (i * 4) as u8) as usize;
        set_4_bits(
            &mut block_low,
            S_BOX[i][value_to_change],
            (i * 4) as u8
        );
    }
    block_low = ((block_low >> 11) | (block_low << (32 - 11))) & 0xFFFFFFFF;
    block_low ^= block_high;

    (block_low, block_low_clone)
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

fn convert_to_u32(u8_block: &[u8]) -> u32 {
    match u8_block.len() {
        4 => BigEndian::read_u32(u8_block),
        3 => BigEndian::read_u32(&[u8_block[0], u8_block[1], u8_block[2], 0_u8]),
        2 => BigEndian::read_u32(&[u8_block[0], u8_block[1], 0_u8, 0_u8]),
        1 => BigEndian::read_u32(&[u8_block[0], 0_u8, 0_u8, 0_u8]),
        _value => panic!("Inaccurate block size {_value}",)
    }
}

fn set_4_bits(u32_val: &mut u32, u4_val: u8, position: u8) {
    *u32_val &= !(0b1111 << position);
    *u32_val |= (u4_val as u32) << position;
}

fn get_4_bits(u32_val: &u32, position: u8) -> u32 {
        let mask = 0b1111 << position;
        (u32_val & mask) >> position
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = &args[1];
    let key = args[2].as_bytes();
    let path = &args[3];
    let path_to_write = &args[4];
    let path_imito = &args[5];
    let mut imito = vec![0_u32, 0_u32];

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
    if converted_data.len() % 2 == 1 {
        converted_data.push(0_u32)
    }


    if mode == "enc" {
        for i in 0..=converted_data.len() / 2 - 1 {
            imito[0] ^= converted_data[2*i];
            imito[1] ^= converted_data[2*i + 1];
            thread::scope(|_| {
                for _ in 0..16 {
                    (converted_data[2*i], converted_data[2*i+1]) = encrypt_block(converted_data[2*i], converted_data[2*i+1], &converted_key);
                }
            });
            thread::scope(|_| {
                for _ in 0..16 {
                    (imito[0], imito[1]) = encrypt_block(imito[0], imito[1], &converted_key);
                }
            });
        }
        fs::write(path_imito, transform_u32_to_array_of_u8(&imito)).expect("Error while write to file");
    }
    else if mode == "dec" {
        for i in 0..=converted_data.len() / 2 - 1 {
            for _ in 0..16 {
                (converted_data[2 * i], converted_data[2 * i + 1]) = decrypt_block(converted_data[2 * i], converted_data[2 * i + 1], &converted_key);
            }
        }
        for i in 0..=converted_data.len() / 2 - 1 {
            imito[0] ^= converted_data[2*i];
            imito[1] ^= converted_data[2*i + 1];
            for _ in 0..16 {
                (imito[0], imito[1]) = encrypt_block(imito[0], imito[1], &converted_key);
            }
        }
        if imito != read_and_convert_data(path_imito) { panic!("Data has been modified") }
    }
    else {
        panic!("incorrect input set 'enc' or 'dec' values")
    }

    fs::write(path_to_write, transform_u32_to_array_of_u8(&converted_data)).expect("Error while write to file");
}

#[cfg(test)]
mod tests {
    use crate::{get_4_bits, S_BOX, set_4_bits};

    #[test]
    fn check_right_box_convert() {
        let mut a: u32 = 3975840786;
        for i in 0..8 {
            let value_to_change = get_4_bits(&a, (i * 4) as u8) as usize;
            set_4_bits(
                &mut a,
                S_BOX[i][value_to_change],
                (i * 4) as u8
            );
        }
        assert_eq!(a, 886879260)
    }
}
//hello mello pello kello tello sello tello dellos