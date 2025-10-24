mod stegarst;

use clap::Parser;
use stegarst::{cli::Cli, decode::decode, encode::encode};

macro_rules! info {
    ($($arg:tt)*) => {{
        println!("[INFO] {}", format_args!($($arg)*));
    }};
}

fn main() {
    let args = Cli::parse();

    match args.option.as_str() {
        "read" => {
            info!("Starting to read file {}", &args.image);
            decode(&args.image, &args.output)
        }
        "write" => match args.file {
            Some(file) => {
                info!("Starting to write file {}", &args.output);
                encode(&args.image, &file, &args.output)
            }
            _none => eprintln!("ERROR: File not passed!"),
        },
        _ => panic!("No valid option given: please try to use --help to see the valid options"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use png::{BitDepth, ColorType, Decoder, Encoder};
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::io::{BufReader, BufWriter};

    // Helper function to create a test PNG image
    fn create_test_image(path: &str, width: u32, height: u32) {
        let file = File::create(path).unwrap();
        let writer = BufWriter::new(file);
        let mut encoder = Encoder::new(writer, width, height);
        encoder.set_color(ColorType::Rgba);
        encoder.set_depth(BitDepth::Eight);

        // Create image data with some pattern
        let data_size = (width * height * 4) as usize;
        let data: Vec<u8> = (0..data_size).map(|i| (i % 256) as u8).collect();

        encoder
            .write_header()
            .unwrap()
            .write_image_data(&data)
            .unwrap();
    }

    // Helper function to create a test message file
    fn create_test_message(path: &str, content: &[u8]) {
        let mut file = File::create(path).unwrap();
        file.write_all(content).unwrap();
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let test_image = "test_encode_image.png";
        let test_message = "test_message.txt";
        let encoded_image = "test_encoded.png";
        let decoded_message = "test_decoded.txt";

        // Setup
        create_test_image(test_image, 100, 100);
        let message_content = b"Hello, World! This is a test message.";
        create_test_message(test_message, message_content);

        // Encode
        encode(test_image, test_message, encoded_image);

        // Decode
        decode(encoded_image, decoded_message);

        // Verify
        let decoded_content = fs::read(decoded_message).unwrap();
        assert_eq!(decoded_content, message_content);

        // Cleanup
        fs::remove_file(test_image).ok();
        fs::remove_file(test_message).ok();
        fs::remove_file(encoded_image).ok();
        fs::remove_file(decoded_message).ok();
    }

    #[test]
    fn test_encode_empty_message() {
        let test_image = "test_empty_image.png";
        let test_message = "test_empty_message.txt";
        let encoded_image = "test_empty_encoded.png";
        let decoded_message = "test_empty_decoded.txt";

        // Setup
        create_test_image(test_image, 50, 50);
        create_test_message(test_message, b"");

        // Encode empty message
        encode(test_image, test_message, encoded_image);

        // Decode
        decode(encoded_image, decoded_message);

        // Verify
        let decoded_content = fs::read(decoded_message).unwrap();
        assert_eq!(decoded_content, b"");

        // Cleanup
        fs::remove_file(test_image).ok();
        fs::remove_file(test_message).ok();
        fs::remove_file(encoded_image).ok();
        fs::remove_file(decoded_message).ok();
    }

    #[test]
    fn test_encode_binary_data() {
        let test_image = "test_binary_image.png";
        let test_message = "test_binary_message.bin";
        let encoded_image = "test_binary_encoded.png";
        let decoded_message = "test_binary_decoded.bin";

        // Setup
        create_test_image(test_image, 100, 100);
        let binary_data: Vec<u8> = (0..256).map(|i| i as u8).collect();
        create_test_message(test_message, &binary_data);

        // Encode
        encode(test_image, test_message, encoded_image);

        // Decode
        decode(encoded_image, decoded_message);

        // Verify
        let decoded_content = fs::read(decoded_message).unwrap();
        assert_eq!(decoded_content, binary_data);

        // Cleanup
        fs::remove_file(test_image).ok();
        fs::remove_file(test_message).ok();
        fs::remove_file(encoded_image).ok();
        fs::remove_file(decoded_message).ok();
    }

    #[test]
    fn test_encode_preserves_image_dimensions() {
        let test_image = "test_dims_image.png";
        let test_message = "test_dims_message.txt";
        let encoded_image = "test_dims_encoded.png";

        // Setup
        create_test_image(test_image, 80, 60);
        create_test_message(test_message, b"test");

        // Encode
        encode(test_image, test_message, encoded_image);

        // Verify dimensions
        let decoder = Decoder::new(BufReader::new(File::open(encoded_image).unwrap()));
        let reader = decoder.read_info().unwrap();
        let info = reader.info();
        assert_eq!(info.width, 80);
        assert_eq!(info.height, 60);

        // Cleanup
        fs::remove_file(test_image).ok();
        fs::remove_file(test_message).ok();
        fs::remove_file(encoded_image).ok();
    }

    #[test]
    fn test_encode_large_message() {
        let test_image = "test_large_image.png";
        let test_message = "test_large_message.txt";
        let encoded_image = "test_large_encoded.png";
        let decoded_message = "test_large_decoded.txt";

        // Setup - create a larger image to accommodate more data
        create_test_image(test_image, 200, 200);
        let large_message: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();
        create_test_message(test_message, &large_message);

        // Encode
        encode(test_image, test_message, encoded_image);

        // Decode
        decode(encoded_image, decoded_message);

        // Verify
        let decoded_content = fs::read(decoded_message).unwrap();
        assert_eq!(decoded_content, large_message);

        // Cleanup
        fs::remove_file(test_image).ok();
        fs::remove_file(test_message).ok();
        fs::remove_file(encoded_image).ok();
        fs::remove_file(decoded_message).ok();
    }

    #[test]
    fn test_encode_special_characters() {
        let test_image = "test_special_image.png";
        let test_message = "test_special_message.txt";
        let encoded_image = "test_special_encoded.png";
        let decoded_message = "test_special_decoded.txt";

        // Setup
        create_test_image(test_image, 100, 100);
        let special_content = b"!@#$%^&*()_+-=[]{}|;':\",./<>?\n\t\r";
        create_test_message(test_message, special_content);

        // Encode
        encode(test_image, test_message, encoded_image);

        // Decode
        decode(encoded_image, decoded_message);

        // Verify
        let decoded_content = fs::read(decoded_message).unwrap();
        assert_eq!(decoded_content, special_content);

        // Cleanup
        fs::remove_file(test_image).ok();
        fs::remove_file(test_message).ok();
        fs::remove_file(encoded_image).ok();
        fs::remove_file(decoded_message).ok();
    }
}
