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

// Retrieves a hidden message from an image using LSB steganography
// src: path to the source image
// dest: path to the destination file to save the hidden message
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
