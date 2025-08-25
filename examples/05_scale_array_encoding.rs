// Example 05: SCALE Array Encoding
// Demonstrates the SCALE (Substrate Common Abstraction Layer) encoding
// for Array types. Arrays are fixed-size collections where the size is
// part of the type definition and is NOT encoded in the byte stream.
// This is different from Vectors, which encode their length.

use std::fmt;

// --- Array Encoding Functions ---

/// Encodes a fixed-size array of u8 values into SCALE format.
/// The size is NOT encoded - it's implicit from the type definition.
/// Each u8 value is encoded as a single byte.
fn encode_array_u8<const N: usize>(arr: &[u8; N]) -> Vec<u8> {
    arr.to_vec() // Direct byte representation
}

/// Decodes SCALE bytes into a fixed-size array of u8 values.
/// The size is NOT decoded - it's implicit from the type definition.
fn decode_array_u8<const N: usize>(bytes: &[u8]) -> Result<[u8; N], String> {
    if bytes.len() < N {
        return Err(format!(
            "Not enough bytes to decode array of u8. Expected {} bytes, got {}.",
            N, bytes.len()
        ));
    }
    
    let mut arr = [0u8; N];
    arr.copy_from_slice(&bytes[0..N]);
    Ok(arr)
}

/// Encodes a fixed-size array of u16 values into SCALE format.
/// The size is NOT encoded - it's implicit from the type definition.
/// Each u16 value is encoded as 2 bytes in little-endian format.
fn encode_array_u16<const N: usize>(arr: &[u16; N]) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(N * 2);
    for &value in arr {
        encoded.extend_from_slice(&value.to_le_bytes());
    }
    encoded
}

/// Decodes SCALE bytes into a fixed-size array of u16 values.
/// The size is NOT decoded - it's implicit from the type definition.
/// Each u16 value is decoded from 2 bytes in little-endian format.
fn decode_array_u16<const N: usize>(bytes: &[u8]) -> Result<[u16; N], String> {
    if bytes.len() < N * 2 {
        return Err(format!(
            "Not enough bytes to decode array of u16. Expected {} bytes, got {}.",
            N * 2, bytes.len()
        ));
    }
    
    let mut arr = [0u16; N];
    for i in 0..N {
        let start = i * 2;
        let end = start + 2;
        let mut bytes_u16 = [0u8; 2];
        bytes_u16.copy_from_slice(&bytes[start..end]);
        arr[i] = u16::from_le_bytes(bytes_u16);
    }
    Ok(arr)
}

// Helper function to print arrays in a readable format
fn print_array<T: fmt::Debug>(arr: &[T], name: &str) {
    println!("{}: {:?}", name, arr);
}

// Helper function to convert bytes to hex string
fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect::<Vec<String>>().join("")
}

fn main() {
    println!("=== SCALE Array Encoding Examples ===");
    println!("Based on the encoding scheme from the image\n");

    // --- Array<u8, 4> Examples ---
    println!("--- Array<u8, 4> Examples ---");

    // Example: [2, 1, 3, 0] => 0x 02 01 03 00
    let arr_u8_4: [u8; 4] = [2, 1, 3, 0];
    let encoded_u8_4 = encode_array_u8(&arr_u8_4);
    println!("Array<u8, 4> [2, 1, 3, 0]:");
    print_array(&arr_u8_4, "  Input");
    println!("  Encoded: 0x{}", bytes_to_hex(&encoded_u8_4));
    assert_eq!(bytes_to_hex(&encoded_u8_4), "02010300");
    
    let decoded_u8_4 = decode_array_u8::<4>(&encoded_u8_4).unwrap();
    print_array(&decoded_u8_4, "  Decoded");
    assert_eq!(decoded_u8_4, arr_u8_4);
    println!("  ✅ Encoding/decoding successful\n");

    // --- Array<u16, 2> Examples ---
    println!("--- Array<u16, 2> Examples ---");

    // Example: [258, 3] => 0x 0201 0300
    let arr_u16_2: [u16; 2] = [258, 3];
    let encoded_u16_2 = encode_array_u16(&arr_u16_2);
    println!("Array<u16, 2> [258, 3]:");
    print_array(&arr_u16_2, "  Input");
    println!("  Encoded: 0x{}", bytes_to_hex(&encoded_u16_2));
    assert_eq!(bytes_to_hex(&encoded_u16_2), "02010300");
    
    // Let's break down the encoding:
    // 258 (decimal) = 0x0102 (hex) -> little-endian: 0x02 0x01
    // 3 (decimal) = 0x0003 (hex) -> little-endian: 0x03 0x00
    // Combined: 0x02 0x01 0x03 0x00
    println!("  Breakdown:");
    println!("    258 (0x0102) -> little-endian: 0x02 0x01");
    println!("    3 (0x0003) -> little-endian: 0x03 0x00");
    
    let decoded_u16_2 = decode_array_u16::<2>(&encoded_u16_2).unwrap();
    print_array(&decoded_u16_2, "  Decoded");
    assert_eq!(decoded_u16_2, arr_u16_2);
    println!("  ✅ Encoding/decoding successful\n");

    // --- Array<u16, 4> Examples ---
    println!("--- Array<u16, 4> Examples ---");

    // Example: [2, 1, 3, 0] => 0x 0200 0100 0300 0000
    let arr_u16_4: [u16; 4] = [2, 1, 3, 0];
    let encoded_u16_4 = encode_array_u16(&arr_u16_4);
    println!("Array<u16, 4> [2, 1, 3, 0]:");
    print_array(&arr_u16_4, "  Input");
    println!("  Encoded: 0x{}", bytes_to_hex(&encoded_u16_4));
    assert_eq!(bytes_to_hex(&encoded_u16_4), "0200010003000000");
    
    // Let's break down the encoding:
    // 2 (decimal) = 0x0002 (hex) -> little-endian: 0x02 0x00
    // 1 (decimal) = 0x0001 (hex) -> little-endian: 0x01 0x00
    // 3 (decimal) = 0x0003 (hex) -> little-endian: 0x03 0x00
    // 0 (decimal) = 0x0000 (hex) -> little-endian: 0x00 0x00
    // Combined: 0x02 0x00 0x01 0x00 0x03 0x00 0x00 0x00
    println!("  Breakdown:");
    println!("    2 (0x0002) -> little-endian: 0x02 0x00");
    println!("    1 (0x0001) -> little-endian: 0x01 0x00");
    println!("    3 (0x0003) -> little-endian: 0x03 0x00");
    println!("    0 (0x0000) -> little-endian: 0x00 0x00");
    
    let decoded_u16_4 = decode_array_u16::<4>(&encoded_u16_4).unwrap();
    print_array(&decoded_u16_4, "  Decoded");
    assert_eq!(decoded_u16_4, arr_u16_4);
    println!("  ✅ Encoding/decoding successful\n");

    // --- Additional Test Cases ---
    println!("--- Additional Test Cases ---");

    // Test Array<u8, 1>
    let arr_u8_1: [u8; 1] = [42];
    let encoded_u8_1 = encode_array_u8(&arr_u8_1);
    println!("Array<u8, 1> [42]:");
    print_array(&arr_u8_1, "  Input");
    println!("  Encoded: 0x{}", bytes_to_hex(&encoded_u8_1));
    assert_eq!(bytes_to_hex(&encoded_u8_1), "2a");
    
    let decoded_u8_1 = decode_array_u8::<1>(&encoded_u8_1).unwrap();
    print_array(&decoded_u8_1, "  Decoded");
    assert_eq!(decoded_u8_1, arr_u8_1);
    println!("  ✅ Encoding/decoding successful\n");

    // Test Array<u16, 1>
    let arr_u16_1: [u16; 1] = [0x1234];
    let encoded_u16_1 = encode_array_u16(&arr_u16_1);
    println!("Array<u16, 1> [0x1234]:");
    print_array(&arr_u16_1, "  Input");
    println!("  Encoded: 0x{}", bytes_to_hex(&encoded_u16_1));
    assert_eq!(bytes_to_hex(&encoded_u16_1), "3412"); // 0x1234 in little-endian
    
    let decoded_u16_1 = decode_array_u16::<1>(&encoded_u16_1).unwrap();
    print_array(&decoded_u16_1, "  Decoded");
    assert_eq!(decoded_u16_1, arr_u16_1);
    println!("  ✅ Encoding/decoding successful\n");

    // --- Error Handling Tests ---
    println!("--- Error Handling Tests ---");

    // Test insufficient bytes for u8 array
    let insufficient_u8_bytes = vec![0x01, 0x02]; // Only 2 bytes for Array<u8, 4>
    match decode_array_u8::<4>(&insufficient_u8_bytes) {
        Ok(_) => println!("❌ Unexpected success with insufficient bytes"),
        Err(e) => println!("✅ Expected error: {}", e),
    }

    // Test insufficient bytes for u16 array
    let insufficient_u16_bytes = vec![0x01, 0x02, 0x03]; // Only 3 bytes for Array<u16, 2>
    match decode_array_u16::<2>(&insufficient_u16_bytes) {
        Ok(_) => println!("❌ Unexpected success with insufficient bytes"),
        Err(e) => println!("✅ Expected error: {}", e),
    }

    println!("\n=== SCALE Array Encoding Analysis ===");
    println!("Key differences between Arrays and Vectors:");
    println!("- Arrays: Fixed size, size NOT encoded in bytes");
    println!("- Vectors: Variable size, size IS encoded (compact-encoded)");
    println!("- Arrays: More efficient for fixed-size data");
    println!("- Arrays: Type safety through compile-time size checking");
    println!("- Both: Elements encoded the same way (u8 as 1 byte, u16 as 2 bytes little-endian)");
}
