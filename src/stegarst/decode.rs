//! Module for decoding messages hidden in images using LSB steganography
use crate::stegarst::bit_utils::BitUtils;
use png::Decoder;
use std::{
    fs::File,
    io::{BufReader, Write},
    path::Path,
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

/// Retrieves a message hidden inside an image using LSB steganography
///
/// ## Arguments:
///   - **src:** `&str`
///     -  path to the source image
///   - **dest:** `&str`
///     - path to the destination file to save the message
/// ## Returns:
/// - `void`
///
/// ## Example:
/// ```rust
///   decode("input.png", "output_message.txt");
/// ```
pub fn decode(src: &str, dest: &str) {
    info!("Getting image data");
    let decoder = Decoder::new(BufReader::new(File::open(src).unwrap()));

    let mut binding = decoder.read_info();
    let reader = binding.as_mut().unwrap();

    let mut data = vec![0; reader.output_buffer_size().unwrap()];
    reader.next_frame(&mut data).unwrap();

    let (message_len, image_data) = data.split_at(32);
    let message_len = BitUtils::byte_u32_to_decimal(BitUtils::read_lsb(message_len.to_vec()));
    info!("Message size of {} bits", &message_len);
    let (bytes_message, _): (&[u8], &[u8]) = image_data.split_at(message_len as usize);
    let message_bits = BitUtils::read_lsb(bytes_message.to_vec());

    let message_retrived = BitUtils::bits_to_bytes(message_bits);

    let mut output_file = File::create(Path::new(dest)).unwrap();

    info!("Writing message found to file {}", &dest);
    output_file.write_all(&message_retrived).unwrap();

    success!("Succesfully retrived message to {}", &dest);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stegarst::encode;
    use png::{BitDepth, ColorType, Encoder};
    use std::fs;
    use std::io::BufWriter;
    use std::io::Write;

    #[test]
    fn test_decode_retrieves_message() {
        let temp_dir = std::env::temp_dir();
        let src_path = temp_dir.join("test_input.png");
        let encoded_path = temp_dir.join("test_encoded.png");
        let decoded_path = temp_dir.join("test_decoded.txt");
        let msg_path = temp_dir.join("test_message.txt");

        // Create a small PNG image
        {
            let file = fs::File::create(&src_path).unwrap();
            let w = BufWriter::new(file);
            let mut encoder = Encoder::new(w, 10, 10);
            encoder.set_color(ColorType::Rgb);
            encoder.set_depth(BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            let data = vec![255; 10 * 10 * 3];
            writer.write_image_data(&data).unwrap();
        }

        // Create message file
        let message = b"Hello, world!";
        {
            let mut msg_file = fs::File::create(&msg_path).unwrap();
            msg_file.write_all(message).unwrap();
        }

        // Encode
        encode::encode(
            src_path.to_str().unwrap(),
            msg_path.to_str().unwrap(),
            encoded_path.to_str().unwrap(),
        );

        // Decode
        decode(
            encoded_path.to_str().unwrap(),
            decoded_path.to_str().unwrap(),
        );

        // Check the decoded message
        let decoded_message = fs::read(&decoded_path).unwrap();
        assert_eq!(decoded_message, message);

        // Cleanup
        let _ = fs::remove_file(&src_path);
        let _ = fs::remove_file(&msg_path);
        let _ = fs::remove_file(&encoded_path);
        let _ = fs::remove_file(&decoded_path);
    }
}
