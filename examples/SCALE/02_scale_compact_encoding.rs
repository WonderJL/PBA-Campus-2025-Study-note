// Example 02: SCALE Compact Encoding
// Demonstrates the SCALE Compact encoding scheme for integers.
// This scheme optimizes storage by using a variable number of bytes
// based on the magnitude of the integer, indicated by the least
// significant 2 bits of the first byte.

// Encoding Rules (based on the least significant 2 bits of the first byte):
// - 0b00: 1 byte, for values 0 to 63 (2^6 - 1)
//   Value stored in 6 most significant bits: [VVVVVV00]
// - 0b01: 2 bytes, for values 64 to 16383 (2^14 - 1)
//   Value stored in 14 bits: [VVVVVV01, VVVVVVVV] (little-endian)
// - 0b10: 4 bytes, for values 16384 to 1073741823 (2^30 - 1)
//   Value stored in 30 bits: [VVVVVV10, VVVVVVVV, VVVVVVVV, VVVVVVVV]
// - 0b11: Variable bytes, for values 1073741824 to 2^536 - 1
//   Remaining 6 bits indicate length (+4): [LLLLLL11, VVVVVVVV, ...]

/// Encodes a u128 integer into SCALE Compact bytes.
/// Supports the full range up to u128 (subset of the full 2^536 range).
fn encode_compact(value: u128) -> Vec<u8> {
    if value <= 63 {
        // 0b00 case: 1 byte
        // Value fits in 6 bits, tag is 00
        vec![((value as u8) << 2) | 0b00]
    } else if value <= 16383 {
        // 0b01 case: 2 bytes
        // Value needs 14 bits, tag is 01
        let mut bytes = Vec::with_capacity(2);
        // First byte: 6 LSBs of value + 0b01 tag
        bytes.push(((value & 0x3F) as u8) << 2 | 0b01);
        // Second byte: next 8 bits of value
        bytes.push(((value >> 6) & 0xFF) as u8);
        bytes
    } else if value <= 1073741823 {
        // 0b10 case: 4 bytes
        // Value needs 30 bits, tag is 10
        let mut bytes = Vec::with_capacity(4);
        // First byte: 6 LSBs of value + 0b10 tag
        bytes.push(((value & 0x3F) as u8) << 2 | 0b10);
        // Subsequent bytes: next 8 bits each
        bytes.push(((value >> 6) & 0xFF) as u8);
        bytes.push(((value >> 14) & 0xFF) as u8);
        bytes.push(((value >> 22) & 0xFF) as u8);
        bytes
    } else {
        // 0b11 case: Variable bytes
        // For u128, max bytes needed for value is 16
        let mut value_bytes = value.to_le_bytes().to_vec();
        // Remove trailing zeros
        while value_bytes.len() > 1 && *value_bytes.last().unwrap() == 0 {
            value_bytes.pop();
        }

        let num_value_bytes = value_bytes.len();
        if num_value_bytes > 67 { // Max for 0b11 is 63+4 = 67
            panic!("Value too large for SCALE Compact encoding");
        }

        let length_indicator = num_value_bytes - 4; // L in LLLLLL11
        if length_indicator > 63 { // Must fit in 6 bits
            panic!("Length indicator too large");
        }

        let mut encoded_bytes = Vec::with_capacity(1 + num_value_bytes);
        // First byte: length_indicator in 6 MSBs + 0b11 tag
        encoded_bytes.push(((length_indicator as u8) << 2) | 0b11);
        // Append value bytes (little-endian)
        encoded_bytes.extend_from_slice(&value_bytes);
        encoded_bytes
    }
}

/// Decodes SCALE Compact bytes into a u128 integer.
fn decode_compact(bytes: &[u8]) -> Result<u128, &'static str> {
    if bytes.is_empty() {
        return Err("Input bytes cannot be empty");
    }

    let first_byte = bytes[0];
    let tag = first_byte & 0b11;
    let initial_value_part = (first_byte >> 2) as u128;

    match tag {
        0b00 => {
            // 1 byte encoding
            Ok(initial_value_part)
        }
        0b01 => {
            // 2 bytes encoding
            if bytes.len() < 2 {
                return Err("Not enough bytes for 0b01 encoding");
            }
            let value = initial_value_part | ((bytes[1] as u128) << 6);
            Ok(value)
        }
        0b10 => {
            // 4 bytes encoding
            if bytes.len() < 4 {
                return Err("Not enough bytes for 0b10 encoding");
            }
            let value = initial_value_part
                | ((bytes[1] as u128) << 6)
                | ((bytes[2] as u128) << 14)
                | ((bytes[3] as u128) << 22);
            Ok(value)
        }
        0b11 => {
            // Variable bytes encoding
            let length_indicator = initial_value_part as usize;
            let num_value_bytes = length_indicator + 4;
            if bytes.len() < 1 + num_value_bytes {
                return Err("Not enough bytes for 0b11 encoding");
            }

            let mut value_bytes = [0u8; 16]; // Max u128 needs 16 bytes
            if num_value_bytes > 16 {
                return Err("Decoded value exceeds u128 capacity");
            }
            value_bytes[..num_value_bytes].copy_from_slice(&bytes[1..1 + num_value_bytes]);

            Ok(u128::from_le_bytes(value_bytes))
        }
        _ => Err("Invalid SCALE Compact tag"),
    }
}

/// Helper function to print binary representation
fn print_binary(bytes: &[u8]) {
    for (i, &byte) in bytes.iter().enumerate() {
        print!("Byte {}: {:08b} (0x{:02x})", i, byte, byte);
        if i == 0 {
            let tag = byte & 0b11;
            let value_part = byte >> 2;
            print!(" [Tag: {:02b}, Value part: {:06b}]", tag, value_part);
        }
        println!();
    }
}

fn main() {
    println!("=== SCALE Compact Encoding Examples ===");
    println!("Based on the encoding scheme from the image\n");

    // Test cases covering all encoding modes
    let test_values = vec![
        (0, "0b00 case: minimum value"),
        (1, "0b00 case: small value"),
        (63, "0b00 case: maximum value (2^6 - 1)"),
        (64, "0b01 case: minimum value"),
        (100, "0b01 case: typical value"),
        (16383, "0b01 case: maximum value (2^14 - 1)"),
        (16384, "0b10 case: minimum value"),
        (100_000, "0b10 case: typical value"),
        (1073741823, "0b10 case: maximum value (2^30 - 1)"),
        (1073741824, "0b11 case: minimum value (4 value bytes)"),
        (u32::MAX as u128 + 1, "0b11 case: requires 5 value bytes"),
        (0x123456789ABCDEF0_u64 as u128, "0b11 case: 8 value bytes"),
        (u128::MAX, "0b11 case: maximum u128 value (16 value bytes)"),
    ];

    for (value, description) in test_values {
        println!("--- {} ---", description);
        println!("Value: {}", value);
        
        let encoded = encode_compact(value);
        println!("Encoded bytes: {:?} ({} bytes)", encoded, encoded.len());
        print!("Binary representation: ");
        print_binary(&encoded);
        
        let decoded = decode_compact(&encoded);
        match decoded {
            Ok(d_value) => {
                if d_value == value {
                    println!("✅ Decoded: {} (CORRECT)", d_value);
                } else {
                    println!("❌ Decoded: {} (MISMATCH! Expected {}, Got {})", d_value, value, d_value);
                }
            }
            Err(e) => {
                println!("❌ Decoding Error: {}", e);
            }
        }
        println!();
    }

    println!("=== Error Handling Examples ===");
    
    // Test invalid inputs
    let error_cases = vec![
        (vec![], "Empty input"),
        (vec![0b00000001], "0b01 tag but only 1 byte"),
        (vec![0b00000010, 0x00, 0x00], "0b10 tag but only 3 bytes"),
        (vec![0b00000011, 0x00, 0x00, 0x00], "0b11 tag, length=0 but only 3 value bytes"),
    ];

    for (bytes, description) in error_cases {
        println!("--- {} ---", description);
        println!("Input: {:?}", bytes);
        match decode_compact(&bytes) {
            Ok(value) => println!("Unexpected success: {}", value),
            Err(e) => println!("✅ Expected error: {}", e),
        }
        println!();
    }

    println!("=== Encoding Analysis ===");
    println!("This demonstrates how SCALE Compact optimizes storage:");
    println!("- Small values (0-63): 1 byte (vs 8 bytes for u64)");
    println!("- Medium values (64-16383): 2 bytes (vs 8 bytes for u64)");
    println!("- Large values (16384-1073741823): 4 bytes (vs 8 bytes for u64)");
    println!("- Very large values: Variable bytes (efficient for any size)");
}
