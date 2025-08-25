// Example 01: Endianness Conversion
// Demonstrates how to convert between different byte orderings (endianness)
// in Rust using built-in methods.

fn main() {
    let num: u32 = 0x12345678;

    // Convert to little-endian bytes
    let le = num.to_le_bytes();
    println!("Little-endian bytes: {:x?}", le); // [78, 56, 34, 12]

    // Convert to big-endian bytes
    let be = num.to_be_bytes();
    println!("Big-endian bytes: {:x?}", be); // [12, 34, 56, 78]

    // Convert to native-endian bytes (matches your CPU)
    let ne = num.to_ne_bytes();
    println!("Native-endian bytes: {:x?}", ne);
}
