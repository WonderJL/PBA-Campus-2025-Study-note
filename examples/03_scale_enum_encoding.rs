// Example 03: SCALE Enum Encoding
// Demonstrates the SCALE (Substrate Common Abstraction Layer) encoding
// for Enum types, where the first byte indicates the variant (tag)
// and subsequent bytes hold the associated data, if any.

use std::fmt;

// Define an Enum similar to the one in the image
#[derive(Debug, PartialEq)]
enum MyScaleEnum {
    Foo(u16),
    Bar(bool),
    Baz, // Represents _void
}

impl MyScaleEnum {
    /// Encodes the enum variant into SCALE bytes.
    ///
    /// Rules:
    /// - First byte is the tag (index of the variant).
    /// - `u16` values are encoded in little-endian.
    /// - `boolean` values are `0x00` for false, `0x01` for true.
    /// - `_void` variants have no additional bytes.
    fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        match self {
            MyScaleEnum::Foo(val) => {
                bytes.push(0x00); // Tag for Foo (index 0)
                bytes.extend_from_slice(&val.to_le_bytes()); // u16 little-endian
            }
            MyScaleEnum::Bar(val) => {
                bytes.push(0x01); // Tag for Bar (index 1)
                bytes.push(if *val { 0x01 } else { 0x00 }); // boolean
            }
            MyScaleEnum::Baz => {
                bytes.push(0x02); // Tag for Baz (index 2)
                // No additional bytes for _void
            }
        }
        bytes
    }

    /// Decodes SCALE bytes into an enum variant.
    /// Returns the decoded enum and the number of bytes consumed.
    fn decode(bytes: &[u8]) -> Result<(Self, usize), String> {
        if bytes.is_empty() {
            return Err("Input bytes are empty for SCALE Enum decoding".to_string());
        }

        let tag = bytes[0];
        let mut consumed_bytes = 1; // For the tag byte

        match tag {
            0x00 => { // Foo(u16)
                if bytes.len() < consumed_bytes + 2 {
                    return Err(format!("Not enough bytes for Foo(u16). Expected at least {}, got {}", consumed_bytes + 2, bytes.len()));
                }
                let u16_bytes: [u8; 2] = bytes[consumed_bytes..consumed_bytes + 2]
                    .try_into()
                    .map_err(|_| "Failed to convert bytes to u16 array".to_string())?;
                let value = u16::from_le_bytes(u16_bytes);
                consumed_bytes += 2;
                Ok((MyScaleEnum::Foo(value), consumed_bytes))
            }
            0x01 => { // Bar(bool)
                if bytes.len() < consumed_bytes + 1 {
                    return Err(format!("Not enough bytes for Bar(bool). Expected at least {}, got {}", consumed_bytes + 1, bytes.len()));
                }
                let bool_byte = bytes[consumed_bytes];
                let value = match bool_byte {
                    0x00 => false,
                    0x01 => true,
                    _ => return Err(format!("Invalid boolean byte for Bar: 0x{:02x}", bool_byte)),
                };
                consumed_bytes += 1;
                Ok((MyScaleEnum::Bar(value), consumed_bytes))
            }
            0x02 => { // Baz (_void)
                Ok((MyScaleEnum::Baz, consumed_bytes))
            }
            _ => Err(format!("Unknown SCALE Enum tag: 0x{:02x}", tag)),
        }
    }
}

// Helper for printing byte arrays in hex
fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter()
         .map(|b| format!("{:02x}", b))
         .collect::<Vec<String>>()
         .join("")
}

fn main() {
    println!("=== SCALE Enum Encoding Examples ===");
    println!("Based on the encoding scheme from the image\n");

    // Test cases covering all encoding modes
    let test_cases = vec![
        (MyScaleEnum::Foo(1), "Foo(1) - u16 value", vec![0x00, 0x01, 0x00]),
        (MyScaleEnum::Bar(false), "Bar(false) - boolean false", vec![0x01, 0x00]),
        (MyScaleEnum::Baz, "Baz() - void variant", vec![0x02]),
        (MyScaleEnum::Bar(true), "Bar(true) - boolean true", vec![0x01, 0x01]),
        (MyScaleEnum::Foo(0x1234), "Foo(0x1234) - larger u16", vec![0x00, 0x34, 0x12]),
    ];

    for (enum_value, description, expected_bytes) in test_cases {
        println!("--- {} ---", description);
        println!("Enum: {:?}", enum_value);
        
        let encoded = enum_value.encode();
        println!("Encoded bytes: {:x?} (Hex: {})", encoded, bytes_to_hex(&encoded));
        println!("Expected bytes: {:x?} (Hex: {})", expected_bytes, bytes_to_hex(&expected_bytes));
        
        if encoded == expected_bytes {
            println!("✅ Encoding matches expected");
        } else {
            println!("❌ Encoding mismatch!");
        }
        
        let decoded = MyScaleEnum::decode(&encoded);
        match decoded {
            Ok((decoded_enum, bytes_consumed)) => {
                if decoded_enum == enum_value {
                    println!("✅ Decoded: {:?} (CORRECT, consumed {} bytes)", decoded_enum, bytes_consumed);
                } else {
                    println!("❌ Decoded: {:?} (MISMATCH! Expected {:?})", decoded_enum, enum_value);
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
        (vec![0x00, 0x01], "0x00 tag but only 2 bytes (missing 1 byte for u16)"),
        (vec![0x01], "0x01 tag but only 1 byte (missing boolean byte)"),
        (vec![0x01, 0x05], "0x01 tag with invalid boolean byte (0x05)"),
        (vec![0xFF], "Unknown tag (0xFF)"),
    ];

    for (bytes, description) in error_cases {
        println!("--- {} ---", description);
        println!("Input: {:?} (Hex: {})", bytes, bytes_to_hex(&bytes));
        match MyScaleEnum::decode(&bytes) {
            Ok((value, consumed)) => println!("Unexpected success: {:?} (consumed {} bytes)", value, consumed),
            Err(e) => println!("✅ Expected error: {}", e),
        }
        println!();
    }

    println!("=== SCALE Enum Encoding Analysis ===");
    println!("This demonstrates how SCALE Enum encoding works:");
    println!("- First byte is always the tag (variant index)");
    println!("- Subsequent bytes contain the associated data");
    println!("- u16 values are stored in little-endian format");
    println!("- Boolean values use 0x00 for false, 0x01 for true");
    println!("- Void variants (_void) have no additional data");
    println!("- Total size = 1 byte (tag) + size of associated data");
}
