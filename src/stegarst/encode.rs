//! Module to provide ability to hide a message inside an image, using LSB steganography

use crate::stegarst::bit_utils::BitUtils;
extern crate png;
use png::{Decoder, Encoder};
use std::{
    fs::{File, read},
    io::{BufReader, BufWriter},
};

macro_rules! info {
    ($($arg:tt)*) => {{
        println!("[INFO] {}", format_args!($($arg)*));
    }};
}
macro_rules! success {
    ($($arg:tt)*) => {{
        println!("[SUCCESS] {}", format_args!($($arg)*));
    }};
}
macro_rules! error {
    ($($arg:tt)*) => {{
        eprintln!("[ERROR] {}", format_args!($($arg)*));
    }};
}

///
/// Hides a message inside an image using LSB steganography
///
/// ## Arguments:
///   - **src:** `&str` -  path to the source image
///   - **msg_src:** `&str`
///     -  path to the message file to hide
///   - **dest:** `&str`
///     - path to the destination image to save the result
/// ## Returns:
/// - `void`
///
/// ## Example:
/// ```rust
///   encode("input.png", "message.txt", "output.png");
/// ```
pub fn encode(src: &str, msg_src: &str, dest: &str) {
    info!("Transforming message to bytes");
    let message_bytes = read(msg_src).unwrap();
    let message_bits = BitUtils::make_bits(message_bytes);
    let message_size = BitUtils::byte_u32_to_bit(message_bits.len() as u32);
    info!("Message size {} bits", message_bits.len());
    info!("Embedding message size to message header");
    let mut complete_message = Vec::new();
    complete_message.extend_from_slice(&message_size);
    complete_message.extend_from_slice(&message_bits);

    info!("Opening image {}", &src);
    let decoder = Decoder::new(BufReader::new(File::open(src).unwrap()));

    let mut binding = decoder.read_info();
    let reader = binding.as_mut().unwrap();

    info!("Image capacity: {}", reader.output_buffer_size().unwrap());
    if complete_message.len() > reader.output_buffer_size().unwrap() {
        error!(
            "Image is too small: message size is {} and image allows for {}",
            complete_message.len(),
            reader.output_buffer_size().unwrap()
        );
        return;
    }

    let mut data = vec![0; reader.output_buffer_size().unwrap()];
    reader.next_frame(&mut data).unwrap();

    let info = reader.info();

    info!("Saving information in the LSB");
    for (i, bit) in complete_message.iter().enumerate() {
        if *bit == 1 && data[i] % 2 == 0 {
            data[i] += 1;
        } else if *bit == 0 && data[i] % 2 != 0 {
            data[i] -= 1;
        }
    }

    let encoded_img = File::create(dest).unwrap();

    let mut image_encoder = Encoder::new(BufWriter::new(encoded_img), info.width, info.height);

    image_encoder.set_color(info.color_type);
    image_encoder.set_depth(info.bit_depth);
    info!("Saving generated image to {}", &dest);
    image_encoder
        .write_header()
        .unwrap()
        .write_image_data(&data)
        .unwrap();
    success!("Succesfully saved message on image {}", &dest);
}

#[cfg(test)]
mod tests {
    use super::*;
    use png::{BitDepth, ColorType, Encoder};
    use std::fs;
    use std::io::BufWriter;
    use std::io::Write;

    #[test]
    fn test_encode_creates_file() {
        let temp_dir = std::env::temp_dir();
        let src_path = temp_dir.join("test_input.png");
        let msg_path = temp_dir.join("test_message.txt");
        let dest_path = temp_dir.join("test_output.png");

        // Create a simple PNG image (10x10 for sufficient capacity)
        {
            let file = fs::File::create(&src_path).unwrap();
            let w = BufWriter::new(file);
            let mut encoder = Encoder::new(w, 10, 10);
            encoder.set_color(ColorType::Rgb);
            encoder.set_depth(BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            let data = vec![255; 10 * 10 * 3]; // 300 bytes of data
            writer.write_image_data(&data).unwrap();
        }

        // Create message file
        {
            let mut msg_file = fs::File::create(&msg_path).unwrap();
            msg_file.write_all(b"Hi").unwrap();
        }

        // Encode
        encode(
            src_path.to_str().unwrap(),
            msg_path.to_str().unwrap(),
            dest_path.to_str().unwrap(),
        );

        // Check if dest exists
        assert!(dest_path.exists());

        // Cleanup
        let _ = fs::remove_file(&src_path);
        let _ = fs::remove_file(&msg_path);
        let _ = fs::remove_file(&dest_path);
    }
}
