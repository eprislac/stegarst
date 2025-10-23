pub struct BitUtils {}

impl BitUtils {
    // Transforms a decimal represented byte into its bit representation
    // byte: input byte
    // returns: bit array
    pub fn byte_to_bit(byte: u8) -> Vec<u8> {
        (0..8).rev().map(|i| (byte >> i) & 1).collect()
    }

    // Transforms a decimal represented 4 byte into its bits representation
    // byte: input byte
    // returns: bit array
    pub fn byte_u32_to_bit(byte: u32) -> Vec<u8> {
        let mut bits = Vec::new();
        for i in (0..32).rev() {
            bits.push(((byte >> i) & 1) as u8);
        }
        bits
    }

    // Transforms a byte in its bit form into its decimal representation
    // byte: input bit array
    // returns: decimal representation
    pub fn byte_to_decimal(byte: Vec<u8>) -> u8 {
        let mut value: u8 = 0;
        for (i, bit) in byte.iter().enumerate() {
            value += bit << (7 - i);
        }
        value
    }

    // Transforms 4 bytes in its bit form into its decimal representation
    pub fn byte_u32_to_decimal(byte: Vec<u8>) -> u32 {
        byte.iter()
            .enumerate()
            .fold(0, |acc, (i, bit)| acc + ((*bit as u32) << (31 - i)))
    }

    // Reads the least significant bit (LSB) from a byte array
    // bytes: input byte array
    // returns: vector of LSBs
    pub fn read_lsb(bytes: Vec<u8>) -> Vec<u8> {
        bytes.iter().map(|byte| byte % 2).collect()
    }

    // Takes bits and transforms them into bytes
    // bits: input bit array
    // returns: vector of bytes
    pub fn bits_to_bytes(bits: Vec<u8>) -> Vec<u8> {
        bits.chunks(8)
            .map(|chunk| Self::byte_to_decimal(chunk.to_vec()))
            .collect()
    }

    // Takes bytes and transforms them into a bit array
    // bytes: input byte array
    // returns: vector of bits
    pub fn make_bits(bytes: Vec<u8>) -> Vec<u8> {
        bytes
            .iter()
            .flat_map(|byte| Self::byte_to_bit(*byte))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_to_bit() {
        assert_eq!(BitUtils::byte_to_bit(0), vec![0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(BitUtils::byte_to_bit(255), vec![1, 1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(BitUtils::byte_to_bit(1), vec![0, 0, 0, 0, 0, 0, 0, 1]);
        assert_eq!(BitUtils::byte_to_bit(128), vec![1, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(BitUtils::byte_to_bit(170), vec![1, 0, 1, 0, 1, 0, 1, 0]);
        assert_eq!(BitUtils::byte_to_bit(85), vec![0, 1, 0, 1, 0, 1, 0, 1]);
    }

    #[test]
    fn test_byte_u32_to_bit() {
        let result = BitUtils::byte_u32_to_bit(0);
        assert_eq!(result.len(), 32);
        assert_eq!(result, vec![0; 32]);

        let result = BitUtils::byte_u32_to_bit(1);
        let mut expected = vec![0; 31];
        expected.push(1);
        assert_eq!(result, expected);

        let result = BitUtils::byte_u32_to_bit(255);
        let mut expected = vec![0; 24];
        expected.extend_from_slice(&[1, 1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(result, expected);

        let result = BitUtils::byte_u32_to_bit(u32::MAX);
        assert_eq!(result, vec![1; 32]);
    }

    #[test]
    fn test_byte_to_decimal() {
        assert_eq!(BitUtils::byte_to_decimal(vec![0, 0, 0, 0, 0, 0, 0, 0]), 0);
        assert_eq!(BitUtils::byte_to_decimal(vec![1, 1, 1, 1, 1, 1, 1, 1]), 255);
        assert_eq!(BitUtils::byte_to_decimal(vec![0, 0, 0, 0, 0, 0, 0, 1]), 1);
        assert_eq!(BitUtils::byte_to_decimal(vec![1, 0, 0, 0, 0, 0, 0, 0]), 128);
        assert_eq!(BitUtils::byte_to_decimal(vec![1, 0, 1, 0, 1, 0, 1, 0]), 170);
        assert_eq!(BitUtils::byte_to_decimal(vec![0, 1, 0, 1, 0, 1, 0, 1]), 85);
    }

    #[test]
    fn test_byte_u32_to_decimal() {
        assert_eq!(BitUtils::byte_u32_to_decimal(vec![0; 32]), 0);
        assert_eq!(BitUtils::byte_u32_to_decimal(vec![1; 32]), u32::MAX);
        
        let mut bits = vec![0; 31];
        bits.push(1);
        assert_eq!(BitUtils::byte_u32_to_decimal(bits), 1);
        
        let mut bits = vec![0; 24];
        bits.extend_from_slice(&[1, 1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(BitUtils::byte_u32_to_decimal(bits), 255);
        
        let mut bits = vec![1];
        bits.extend_from_slice(&vec![0; 31]);
        assert_eq!(BitUtils::byte_u32_to_decimal(bits), 2147483648);
    }

    #[test]
    fn test_byte_to_bit_and_back() {
        for byte in 0..=255u8 {
            let bits = BitUtils::byte_to_bit(byte);
            let result = BitUtils::byte_to_decimal(bits);
            assert_eq!(result, byte);
        }
    }

    #[test]
    fn test_byte_u32_to_bit_and_back() {
        let test_values = vec![0, 1, 255, 256, 65535, 16777215, u32::MAX];
        for value in test_values {
            let bits = BitUtils::byte_u32_to_bit(value);
            let result = BitUtils::byte_u32_to_decimal(bits);
            assert_eq!(result, value);
        }
    }

    #[test]
    fn test_read_lsb() {
        assert_eq!(BitUtils::read_lsb(vec![]), vec![]);
        assert_eq!(BitUtils::read_lsb(vec![0, 1, 2, 3]), vec![0, 1, 0, 1]);
        assert_eq!(BitUtils::read_lsb(vec![4, 5, 6, 7]), vec![0, 1, 0, 1]);
        assert_eq!(BitUtils::read_lsb(vec![255, 254, 128, 127]), vec![1, 0, 0, 1]);
        assert_eq!(BitUtils::read_lsb(vec![10, 11, 12, 13]), vec![0, 1, 0, 1]);
    }

    #[test]
    fn test_bits_to_bytes() {
        assert_eq!(BitUtils::bits_to_bytes(vec![]), vec![]);
        assert_eq!(
            BitUtils::bits_to_bytes(vec![0, 0, 0, 0, 0, 0, 0, 0]),
            vec![0]
        );
        assert_eq!(
            BitUtils::bits_to_bytes(vec![1, 1, 1, 1, 1, 1, 1, 1]),
            vec![255]
        );
        assert_eq!(
            BitUtils::bits_to_bytes(vec![0, 0, 0, 0, 0, 0, 0, 1]),
            vec![1]
        );
        assert_eq!(
            BitUtils::bits_to_bytes(vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1]),
            vec![0, 255]
        );
    }

    #[test]
    fn test_bits_to_bytes_partial() {
        assert_eq!(
            BitUtils::bits_to_bytes(vec![1, 0, 1, 0, 1, 0, 1]),
            vec![170]
        );
        assert_eq!(
            BitUtils::bits_to_bytes(vec![1, 1, 1]),
            vec![224]
        );
    }

    #[test]
    fn test_make_bits() {
        assert_eq!(BitUtils::make_bits(vec![]), vec![]);
        assert_eq!(
            BitUtils::make_bits(vec![0]),
            vec![0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            BitUtils::make_bits(vec![255]),
            vec![1, 1, 1, 1, 1, 1, 1, 1]
        );
        assert_eq!(
            BitUtils::make_bits(vec![1]),
            vec![0, 0, 0, 0, 0, 0, 0, 1]
        );
        assert_eq!(
            BitUtils::make_bits(vec![0, 255]),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1]
        );
        assert_eq!(
            BitUtils::make_bits(vec![170, 85]),
            vec![1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1]
        );
    }

    #[test]
    fn test_make_bits_and_back() {
        let test_bytes = vec![0, 1, 127, 128, 255, 42, 170, 85];
        let bits = BitUtils::make_bits(test_bytes.clone());
        let result = BitUtils::bits_to_bytes(bits);
        assert_eq!(result, test_bytes);
    }

    #[test]
    fn test_roundtrip_all_bytes() {
        let all_bytes: Vec<u8> = (0..=255).collect();
        let bits = BitUtils::make_bits(all_bytes.clone());
        let result = BitUtils::bits_to_bytes(bits);
        assert_eq!(result, all_bytes);
    }

    #[test]
    fn test_lsb_steganography_simulation() {
        let mut carrier = vec![100, 101, 102, 103, 104, 105, 106, 107];
        let message_bits = vec![1, 0, 1, 1, 0, 1, 0, 0];
        
        for (i, bit) in message_bits.iter().enumerate() {
            if *bit == 1 && carrier[i] % 2 == 0 {
                carrier[i] += 1;
            } else if *bit == 0 && carrier[i] % 2 != 0 {
                carrier[i] -= 1;
            }
        }
        
        let extracted_bits = BitUtils::read_lsb(carrier);
        assert_eq!(extracted_bits, message_bits);
    }
}
