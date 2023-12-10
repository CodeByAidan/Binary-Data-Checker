#![allow(special_module_name)]

use std::env;
use std::io;
use log::error;
use log::{info, LevelFilter};

mod lib;
use crate::lib::binary_data_lib::BinaryData;


fn main() -> io::Result<()> {
    // Initialize the logger without timestamp
    env_logger::builder()
        .format_timestamp(None) // Disable timestamp
        .filter_level(LevelFilter::Info)
        .init();

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--generate" {
        BinaryData::generate_binary_file()?;
        info!("Binary file generated successfully.");
        return Ok(());
    }

    let current_dir: std::path::PathBuf = env::current_dir()?;
    let relative_path: &str = "data/binary/example.bin";
    let file_path: std::path::PathBuf = current_dir.join(relative_path);

    if !file_path.exists() {
        error!(
            "Binary file '{}' does not exist. Please generate it using 'cargo run --bin binary_data_checker -- --generate' first.",
            file_path.display()
        );
        return Ok(());
    }

    let mut binary_data = BinaryData { data: Vec::new() };
    BinaryData::read_binary_file(&file_path, &mut binary_data.data)?;

    info!("Binary Data:");
    info!(" - Bytes: {:?}", binary_data.data);
    info!(
        " - Bits: {:?}",
        binary_data
            .data
            .iter()
            .map(|byte| format!("{:08b}", byte))
            .collect::<Vec<String>>()
    );

    binary_data.flip_bits();
    info!("Flipped Bits: {:?}", binary_data.data);

    if let Some(extracted_range) = binary_data.extract_range(2, 5) {
        info!("Extracted Range (2-5): {:?}", extracted_range);
    } else {
        info!("Invalid range specified for extraction.");
    }

    let hex_string = binary_data.to_hex_string();
    info!("Hex Representation: {}", hex_string);

    let mut bit_count: u32 = 0;
    let mut byte_count: u32 = 0;
    let mut bit_sum: u32 = 0;
    let mut byte_sum: u32 = 0;
    let mut bit_max: u8 = 0;
    let mut bit_min: u8 = 255;
    let mut byte_max: u8 = 0;
    let mut byte_min: u8 = 255;
    for byte in binary_data.data.iter() {
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

    info!("Analysis:");
    info!(" - Bits: {}", bit_count);
    info!(" - Bytes: {}", byte_count);
    info!(" - Bit Sum: {}", bit_sum);
    info!(" - Byte Sum: {}", byte_sum);
    info!(" - Bit Average: {}", bit_avg);
    info!(" - Byte Average: {}", byte_avg);
    info!(" - Bit Max: {}", bit_max);
    info!(" - Byte Max: {}", byte_max);
    info!(" - Bit Min: {}", bit_min);
    info!(" - Byte Min: {}", byte_min);

    Ok(())
}
