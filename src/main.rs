#![allow(special_module_name)]
use std::env;
use std::io::{self};

mod lib;
use crate::lib::binary_data_lib::{generate_binary_file, read_binary_file};


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--generate" {
        generate_binary_file()?;
        println!("Binary file generated successfully.");
        return Ok(());
    }

    let current_dir: std::path::PathBuf = env::current_dir()?;
    let relative_path: &str = "data/binary/example.bin";
    let file_path: std::path::PathBuf = current_dir.join(relative_path);

    if !file_path.exists() {
        eprintln!("Error: Binary file not found. Please generate it using 'cargo run --bin binary_data_checker -- --generate'");
        return Ok(());
    }

    let mut binary_data: Vec<u8> = Vec::new();
    read_binary_file(&file_path, &mut binary_data)?;

    println!("Binary Data:");
    println!(" - Bytes: {:?}", binary_data);
    println!(" - Bits: {:?}", binary_data.iter().map(|byte| format!("{:08b}", byte)).collect::<Vec<String>>());

    let mut bit_count: u32 = 0;
    let mut byte_count: u32 = 0;
    let mut bit_sum: u32 = 0;
    let mut byte_sum: u32 = 0;
    let mut bit_max: u8 = 0;
    let mut bit_min: u8 = 255;
    let mut byte_max: u8 = 0;
    let mut byte_min: u8 = 255;
    for byte in binary_data.iter() {
        byte_count += 1;
        byte_sum += *byte as u32;
        if *byte > byte_max {
            byte_max = *byte;
        }
        if *byte < byte_min {
            byte_min = *byte;
        }
        for bit in 0..8 {
            bit_count += 1;
            bit_sum += (*byte >> bit & 1) as u32;
            if (*byte >> bit) & 1 > bit_max {
                bit_max = (*byte >> bit) & 1;
            }
            if (*byte >> bit) & 1 < bit_min {
                bit_min = (*byte >> bit) & 1;
            }
        }
    }
    let bit_avg: f32 = bit_sum as f32 / bit_count as f32;
    let byte_avg: f32 = byte_sum as f32 / byte_count as f32;

    println!("Analysis:");
    println!(" - Bits: {}", bit_count);
    println!(" - Bytes: {}", byte_count);
    println!(" - Bit Sum: {}", bit_sum);
    println!(" - Byte Sum: {}", byte_sum);
    println!(" - Bit Average: {}", bit_avg);
    println!(" - Byte Average: {}", byte_avg);
    println!(" - Bit Max: {}", bit_max);
    println!(" - Byte Max: {}", byte_max);
    println!(" - Bit Min: {}", bit_min);
    println!(" - Byte Min: {}", byte_min);

    Ok(())
}
