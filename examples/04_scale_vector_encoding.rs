// Example 04: SCALE Vector Encoding
// Demonstrates the SCALE (Substrate Common Abstraction Layer) encoding
// for Vector types. The length of the vector is always compact-encoded,
// followed by the SCALE-encoded elements of the vector.

use std::fmt;

// --- Helper functions for Compact Encoding (reused from Example 02) ---

/// Encodes a u64 value into SCALE Compact format.
/// This function is used for encoding both the vector length and
/// individual elements when the vector contains compact-encoded values.
fn encode_compact(value: u64) -> Vec<u8> {
    if value < 64 {
        // 0b00 variant: value in 6 bits, 00 suffix
        vec![(value as u8) << 2]
    } else if value < 16384 {
        // 0b01 variant: value in 14 bits, 01 suffix
        let val = (value << 2) | 0b01;
        let bytes = val.to_le_bytes();
        bytes[0..2].to_vec() // Take the first two bytes (little-endian)
    } else if value < 1073741824 {
        // 0b10 variant: value in 30 bits, 10 suffix
        let val = (value << 2) | 0b10;
        let bytes = val.to_le_bytes();
        bytes[0..4].to_vec() // Take the first four bytes (little-endian)
    } else {
        // 0b11 variant: value as u32, then length prefix
        // This case is for very large numbers, where the value itself is encoded
        // as a u32, and the length of the u32 is encoded in the prefix.
        // For simplicity and to match the image examples, we'll assume values
        // fit within the 4-byte compact encoding for this example.
        // A full implementation would handle arbitrary length encoding.
        panic!("Values larger than 2^30-1 are not fully implemented in this example's compact encoding.");
    }
}

/// Decodes SCALE Compact bytes into a u64 value and the number of bytes consumed.
fn decode_compact(bytes: &[u8]) -> Result<(u64, usize), String> {
    if bytes.is_empty() {
        return Err("Input bytes are empty for compact decoding.".to_string());
    }

    let first_byte = bytes[0];
    let mode = first_byte & 0b11; // Extract the last two bits

    match mode {
        0b00 => {
            // 1-byte encoding: value in 6 MSBs
            let value = (first_byte >> 2) as u64;
            Ok((value, 1))
        }
        0b01 => {
            // 2-byte encoding: value in 14 bits
            if bytes.len() < 2 {
                return Err("Not enough bytes for 2-byte compact decoding.".to_string());
            }
            let mut val_bytes = [0u8; 2];
            val_bytes.copy_from_slice(&bytes[0..2]);
            let val = u16::from_le_bytes(val_bytes);
            let value = (val >> 2) as u64;
            Ok((value, 2))
        }
        0b10 => {
            // 4-byte encoding: value in 30 bits
            if bytes.len() < 4 {
                return Err("Not enough bytes for 4-byte compact decoding.".to_string());
            }
            let mut val_bytes = [0u8; 4];
            val_bytes.copy_from_slice(&bytes[0..4]);
            let val = u32::from_le_bytes(val_bytes);
            let value = (val >> 2) as u64;
            Ok((value, 4))
        }
        0b11 => {
            // Multi-byte encoding (not fully implemented for arbitrary length in this example)
            // The first byte indicates the number of additional bytes (N) for the length.
            // The value is then encoded in N+4 bytes.
            Err("Multi-byte compact encoding (0b11) not fully implemented in this example.".to_string())
        }
        _ => unreachable!(), // Should not happen with 2-bit mode
    }
}

// --- Vector Encoding Functions ---

/// Encodes a vector of u8 values into SCALE format.
/// The length is compact-encoded, followed by raw u8 elements.
fn encode_vector_u8(vec: &[u8]) -> Vec<u8> {
    let mut encoded = encode_compact(vec.len() as u64);
    encoded.extend_from_slice(vec);
    encoded
}

/// Decodes SCALE bytes into a vector of u8 values.
fn decode_vector_u8(bytes: &[u8]) -> Result<Vec<u8>, String> {
    if bytes.is_empty() {
        return Ok(vec![]); // Empty vector case
    }

    let (len, len_bytes_consumed) = decode_compact(bytes)?;

    if bytes.len() < len_bytes_consumed + len as usize {
        return Err(format!(
            "Not enough bytes to decode vector of u8. Expected {} bytes, got {}.",
            len_bytes_consumed + len as usize, bytes.len()
        ));
    }

    let start_index = len_bytes_consumed;
    let end_index = start_index + len as usize;
    Ok(bytes[start_index..end_index].to_vec())
}

/// Encodes a vector of u64 values into SCALE format, where each u64 element
/// is itself compact-encoded.
fn encode_vector_compact(vec: &[u64]) -> Vec<u8> {
    let mut encoded = encode_compact(vec.len() as u64);
    for &item in vec {
        encoded.extend_from_slice(&encode_compact(item));
    }
    encoded
}

/// Decodes SCALE bytes into a vector of u64 values, where each u64 element
/// is compact-decoded.
fn decode_vector_compact(bytes: &[u8]) -> Result<Vec<u64>, String> {
    if bytes.is_empty() {
        return Ok(vec![]); // Empty vector case
    }

    let (len, len_bytes_consumed) = decode_compact(bytes)?;
    let mut decoded_vec = Vec::with_capacity(len as usize);
    let mut current_index = len_bytes_consumed;

    for _ in 0..len {
        if current_index >= bytes.len() {
            return Err("Not enough bytes to decode compact elements.".to_string());
        }
        let (item_value, item_bytes_consumed) = decode_compact(&bytes[current_index..])?;
        decoded_vec.push(item_value);
        current_index += item_bytes_consumed;
    }

    Ok(decoded_vec)
}

// --- Main function and Tests ---

fn main() {
    println!("=== SCALE Vector Encoding Examples ===");
    println!("Based on the encoding scheme from the image\n");

    // --- Vector(u8) Examples ---
    println!("--- Vector(u8) ---");

    // Example: [] => 0x00
    let vec_u8_empty: Vec<u8> = vec![];
    let encoded_u8_empty = encode_vector_u8(&vec_u8_empty);
    println!("Vector(u8) []: {:?} => 0x{}", vec_u8_empty, bytes_to_hex(&encoded_u8_empty));
    assert_eq!(bytes_to_hex(&encoded_u8_empty), "00");
    let decoded_u8_empty = decode_vector_u8(&encoded_u8_empty).unwrap();
    assert_eq!(decoded_u8_empty, vec_u8_empty);

    // Example: [1] => 0x04 01
    let vec_u8_single: Vec<u8> = vec![1];
    let encoded_u8_single = encode_vector_u8(&vec_u8_single);
    println!("Vector(u8) [1]: {:?} => 0x{}", vec_u8_single, bytes_to_hex(&encoded_u8_single));
    assert_eq!(bytes_to_hex(&encoded_u8_single), "0401");
    let decoded_u8_single = decode_vector_u8(&encoded_u8_single).unwrap();
    assert_eq!(decoded_u8_single, vec_u8_single);

    // Example: [1, 0] => 0x08 01 00
    let vec_u8_two: Vec<u8> = vec![1, 0];
    let encoded_u8_two = encode_vector_u8(&vec_u8_two);
    println!("Vector(u8) [1, 0]: {:?} => 0x{}", vec_u8_two, bytes_to_hex(&encoded_u8_two));
    assert_eq!(bytes_to_hex(&encoded_u8_two), "080100");
    let decoded_u8_two = decode_vector_u8(&encoded_u8_two).unwrap();
    assert_eq!(decoded_u8_two, vec_u8_two);

    // --- Vector(compact) Examples ---
    println!("\n--- Vector(compact) ---");

    // Example: [1, 0] => 0x08 04 00
    let vec_compact_two: Vec<u64> = vec![1, 0];
    let encoded_compact_two = encode_vector_compact(&vec_compact_two);
    println!("Vector(compact) [1, 0]: {:?} => 0x{}", vec_compact_two, bytes_to_hex(&encoded_compact_two));
    assert_eq!(bytes_to_hex(&encoded_compact_two), "080400");
    let decoded_compact_two = decode_vector_compact(&encoded_compact_two).unwrap();
    assert_eq!(decoded_compact_two, vec_compact_two);

    // Example: [1, 0, 64] => 0x0c 04 00 0101
    let vec_compact_three: Vec<u64> = vec![1, 0, 64];
    let encoded_compact_three = encode_vector_compact(&vec_compact_three);
    println!("Vector(compact) [1, 0, 64]: {:?} => 0x{}", vec_compact_three, bytes_to_hex(&encoded_compact_three));
    assert_eq!(bytes_to_hex(&encoded_compact_three), "0c04000101");
    let decoded_compact_three = decode_vector_compact(&encoded_compact_three).unwrap();
    assert_eq!(decoded_compact_three, vec_compact_three);

    println!("\nAll SCALE Vector encoding examples passed!");
}

// Helper for hex encoding (for display purposes)
fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
