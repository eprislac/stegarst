use crate::stegarst::bit_utils::BitUtils;
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
