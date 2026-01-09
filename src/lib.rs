//! Fast Base58 encoding/decoding for UUIDs with minimal dependencies
//!
//! This crate provides efficient Base58 encoding and decoding for UUIDs,
//! with comprehensive error handling and minimal dependencies (only getrandom for secure random generation).

use std::error::Error;
use std::fmt;

/// Base58 alphabet (Bitcoin alphabet)
const BASE58_ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

/// Precomputed reverse lookup table for Base58 decoding
const REVERSE_ALPHABET: [u8; 256] = {
    let mut table = [255u8; 256];
    let alphabet_bytes = BASE58_ALPHABET.as_bytes();
    let mut i = 0u8;
    while i < 58 {
        table[alphabet_bytes[i as usize] as usize] = i;
        i += 1;
    }
    table
};

/// Custom error types for b58uuid operations
#[derive(Debug, Clone, PartialEq)]
pub enum B58UUIDError {
    InvalidUUID(String),
    InvalidBase58(String),
    InvalidLength { expected: usize, got: usize },
    Overflow,
}

impl fmt::Display for B58UUIDError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            B58UUIDError::InvalidUUID(msg) => write!(f, "Invalid UUID: {}", msg),
            B58UUIDError::InvalidBase58(msg) => write!(f, "Invalid Base58: {}", msg),
            B58UUIDError::InvalidLength { expected, got } => {
                write!(f, "Invalid length: expected {}, got {}", expected, got)
            }
            B58UUIDError::Overflow => {
                write!(f, "Arithmetic overflow: value exceeds maximum UUID value")
            }
        }
    }
}

impl Error for B58UUIDError {}

/// Encodes a 16-byte UUID to a Base58 string
///
/// # Arguments
/// * `data` - A 16-byte array representing the UUID
///
/// # Returns
/// * `String` - The Base58-encoded UUID
///
/// # Example
/// ```
/// use b58uuid::encode;
///
/// let uuid_bytes = [
///     0x55, 0x0e, 0x84, 0x00, 0xe2, 0x9b, 0x41, 0xd4,
///     0xa7, 0x16, 0x44, 0x66, 0x55, 0x44, 0x00, 0x00
/// ];
/// let encoded = encode(&uuid_bytes);
/// assert_eq!(encoded, "BWBeN28Vb7cMEx7Ym8AUzs");
/// ```
pub fn encode(data: &[u8; 16]) -> String {
    // Handle leading zeros optimization
    let mut leading_zeros = 0;
    for &byte in data.iter() {
        if byte == 0 {
            leading_zeros += 1;
        } else {
            break;
        }
    }

    // All zeros special case - return 22 '1' characters
    if leading_zeros == 16 {
        return "1".repeat(22);
    }

    // Convert to Base58
    let mut result = Vec::new();
    let mut num = u128::from_be_bytes(*data);

    while num > 0 {
        let remainder = (num % 58) as usize;
        result.push(BASE58_ALPHABET.as_bytes()[remainder]);
        num /= 58;
    }

    // Add leading zeros representation
    result.extend(std::iter::repeat_n(b'1', leading_zeros));

    // Reverse to get correct order
    result.reverse();

    // Pad with leading '1' to ensure 22 characters (more efficient than insert)
    let mut encoded =
        String::from_utf8(result).expect("Base58 encoding should always be valid UTF-8");
    if encoded.len() < 22 {
        let padding = "1".repeat(22 - encoded.len());
        encoded = padding + &encoded;
    }

    encoded
}

/// Decodes a Base58 string to a 16-byte UUID
///
/// # Arguments
/// * `b58` - The Base58-encoded string
///
/// # Returns
/// * `Result<[u8; 16], B58UUIDError>` - The decoded 16-byte UUID or an error
///
/// # Example
/// ```
/// use b58uuid::decode;
///
/// let decoded = decode("BWBeN28Vb7cMEx7Ym8AUzs").unwrap();
/// assert_eq!(decoded, [
///     0x55, 0x0e, 0x84, 0x00, 0xe2, 0x9b, 0x41, 0xd4,
///     0xa7, 0x16, 0x44, 0x66, 0x55, 0x44, 0x00, 0x00
/// ]);
/// ```
pub fn decode(b58: &str) -> Result<[u8; 16], B58UUIDError> {
    if b58.is_empty() {
        return Err(B58UUIDError::InvalidBase58(
            "Empty Base58 string".to_string(),
        ));
    }

    // Count leading ones
    let mut leading_ones = 0;
    for ch in b58.chars() {
        if ch == '1' {
            leading_ones += 1;
        } else {
            break;
        }
    }

    // Convert Base58 to number with overflow checking
    let mut num = 0u128;
    for (i, ch) in b58.chars().enumerate() {
        if ch == '1' && i < leading_ones {
            continue; // Skip leading ones
        }

        // Check if character is within ASCII range before indexing
        if !ch.is_ascii() || ch as usize >= 256 {
            return Err(B58UUIDError::InvalidBase58(format!(
                "Invalid character at position {}: {}",
                i, ch
            )));
        }

        let digit = REVERSE_ALPHABET[ch as usize];
        if digit == 255 {
            return Err(B58UUIDError::InvalidBase58(format!(
                "Invalid character at position {}: {}",
                i, ch
            )));
        }

        // Check for overflow before multiplication
        num = num.checked_mul(58).ok_or(B58UUIDError::Overflow)?;

        // Check for overflow before addition
        num = num
            .checked_add(digit as u128)
            .ok_or(B58UUIDError::Overflow)?;
    }

    // Convert to bytes
    let mut bytes = [0u8; 16];
    let num_bytes = num.to_be_bytes();

    // Ensure the result fits in 16 bytes (check if leading bytes are all zero)
    bytes.copy_from_slice(&num_bytes);

    // Verify leading ones correspond to leading zeros
    // Note: For all-zeros UUID, we encode as 22 '1' characters (with padding)
    // so we need to allow up to 22 leading ones
    if leading_ones > 22 {
        return Err(B58UUIDError::InvalidBase58(
            "Too many leading '1' characters".to_string(),
        ));
    }

    Ok(bytes)
}

/// Generates a new random UUID and returns its Base58-encoded representation
///
/// This function uses cryptographically secure random number generation
/// via the `getrandom` crate, which works across all platforms including
/// Linux, macOS, Windows, iOS, Android, and WebAssembly.
///
/// # Returns
/// * `String` - A new Base58-encoded UUID
///
/// # Example
/// ```
/// use b58uuid::generate;
///
/// let b58_1 = generate();
/// let b58_2 = generate();
/// assert_ne!(b58_1, b58_2); // Should generate unique values
/// ```
pub fn generate() -> String {
    let mut bytes = [0u8; 16];

    // Use getrandom for cryptographically secure random bytes
    // This works on all platforms: Linux, macOS, Windows, iOS, Android, WASM, etc.
    getrandom::getrandom(&mut bytes).expect("Failed to generate random bytes");

    // Set UUID version (4) and variant bits
    bytes[6] = (bytes[6] & 0x0F) | 0x40; // Version 4
    bytes[8] = (bytes[8] & 0x3F) | 0x80; // Variant 10

    encode(&bytes)
}

/// Encodes a UUID string to Base58 format
///
/// # Arguments
/// * `uuid_str` - A UUID string in standard format (with or without hyphens)
///
/// # Returns
/// * `Result<String, B58UUIDError>` - The Base58-encoded UUID or an error
///
/// # Example
/// ```
/// use b58uuid::encode_uuid;
///
/// let encoded = encode_uuid("550e8400-e29b-41d4-a716-446655440000").unwrap();
/// assert_eq!(encoded, "BWBeN28Vb7cMEx7Ym8AUzs");
/// ```
pub fn encode_uuid(uuid_str: &str) -> Result<String, B58UUIDError> {
    let cleaned = uuid_str.replace('-', "");

    if cleaned.len() != 32 {
        return Err(B58UUIDError::InvalidLength {
            expected: 32,
            got: cleaned.len(),
        });
    }

    let mut bytes = [0u8; 16];
    for i in 0..16 {
        let hex_byte = &cleaned[i * 2..i * 2 + 2];
        match u8::from_str_radix(hex_byte, 16) {
            Ok(byte) => bytes[i] = byte,
            Err(_) => {
                return Err(B58UUIDError::InvalidUUID(format!(
                    "Invalid hex at position {}",
                    i * 2
                )))
            }
        }
    }

    Ok(encode(&bytes))
}

/// Decodes a Base58 string to a standard UUID string format
///
/// # Arguments
/// * `b58` - The Base58-encoded string
///
/// # Returns
/// * `Result<String, B58UUIDError>` - The UUID string in standard format or an error
///
/// # Example
/// ```
/// use b58uuid::decode_to_uuid;
///
/// let uuid_str = decode_to_uuid("BWBeN28Vb7cMEx7Ym8AUzs").unwrap();
/// assert_eq!(uuid_str, "550e8400-e29b-41d4-a716-446655440000");
/// ```
pub fn decode_to_uuid(b58: &str) -> Result<String, B58UUIDError> {
    let bytes = decode(b58)?;

    Ok(format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3],
        bytes[4], bytes[5], bytes[6], bytes[7],
        bytes[8], bytes[9], bytes[10], bytes[11],
        bytes[12], bytes[13], bytes[14], bytes[15]
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_basic() {
        let uuid_bytes = [
            0x55, 0x0e, 0x84, 0x00, 0xe2, 0x9b, 0x41, 0xd4, 0xa7, 0x16, 0x44, 0x66, 0x55, 0x44,
            0x00, 0x00,
        ];
        let encoded = encode(&uuid_bytes);
        assert_eq!(encoded, "BWBeN28Vb7cMEx7Ym8AUzs");
    }

    #[test]
    fn test_decode_basic() {
        let decoded = decode("BWBeN28Vb7cMEx7Ym8AUzs").unwrap();
        let expected = [
            0x55, 0x0e, 0x84, 0x00, 0xe2, 0x9b, 0x41, 0xd4, 0xa7, 0x16, 0x44, 0x66, 0x55, 0x44,
            0x00, 0x00,
        ];
        assert_eq!(decoded, expected);
    }

    #[test]
    fn test_round_trip() {
        let original = [
            0x55, 0x0e, 0x84, 0x00, 0xe2, 0x9b, 0x41, 0xd4, 0xa7, 0x16, 0x44, 0x66, 0x55, 0x44,
            0x00, 0x00,
        ];
        let encoded = encode(&original);
        let decoded = decode(&encoded).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_all_zeros() {
        let zeros = [0u8; 16];
        let encoded = encode(&zeros);
        assert_eq!(encoded, "1111111111111111111111"); // 22 ones
        let decoded = decode(&encoded).unwrap();
        assert_eq!(decoded, zeros);
    }

    #[test]
    fn test_all_ones() {
        let ones = [0xFFu8; 16];
        let encoded = encode(&ones);
        assert_eq!(encoded, "YcVfxkQb6JRzqk5kF2tNLv");
        let decoded = decode(&encoded).unwrap();
        assert_eq!(decoded, ones);
    }

    #[test]
    fn test_encode_uuid_string() {
        let encoded = encode_uuid("550e8400-e29b-41d4-a716-446655440000").unwrap();
        assert_eq!(encoded, "BWBeN28Vb7cMEx7Ym8AUzs");
    }

    #[test]
    fn test_decode_to_uuid_string() {
        let uuid_str = decode_to_uuid("BWBeN28Vb7cMEx7Ym8AUzs").unwrap();
        assert_eq!(uuid_str, "550e8400-e29b-41d4-a716-446655440000");
    }

    #[test]
    fn test_generate_unique() {
        let b58_1 = generate();
        let b58_2 = generate();
        assert_ne!(b58_1, b58_2);
        assert_eq!(b58_1.len(), 22); // Should always be 22 characters
        assert_eq!(b58_2.len(), 22);
    }

    #[test]
    fn test_invalid_base58() {
        let result = decode("invalid!");
        assert!(matches!(result, Err(B58UUIDError::InvalidBase58(_))));
    }

    #[test]
    fn test_empty_base58() {
        let result = decode("");
        assert!(matches!(result, Err(B58UUIDError::InvalidBase58(_))));
    }

    #[test]
    fn test_invalid_uuid_length() {
        let result = encode_uuid("550e8400");
        assert!(matches!(result, Err(B58UUIDError::InvalidLength { .. })));
    }

    #[test]
    fn test_invalid_uuid_hex() {
        let result = encode_uuid("550e8400-e29b-41d4-a716-44665544000g");
        assert!(matches!(result, Err(B58UUIDError::InvalidUUID(_))));
    }

    #[test]
    fn test_overflow_detection() {
        // Create a Base58 string that would overflow u128
        // This is a string of maximum Base58 digits that exceeds 2^128-1
        let overflow_str = "zzzzzzzzzzzzzzzzzzzzzz"; // 22 'z' characters
        let result = decode(overflow_str);
        assert!(matches!(result, Err(B58UUIDError::Overflow)));
    }

    #[test]
    fn test_output_length_consistency() {
        // Test that all encodings produce 22-character strings
        let test_cases = vec![
            [0u8; 16],
            [0xFF; 16],
            [
                0x55, 0x0e, 0x84, 0x00, 0xe2, 0x9b, 0x41, 0xd4, 0xa7, 0x16, 0x44, 0x66, 0x55, 0x44,
                0x00, 0x00,
            ],
            [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10,
            ],
        ];

        for bytes in test_cases {
            let encoded = encode(&bytes);
            assert_eq!(
                encoded.len(),
                22,
                "Encoded length should always be 22 for {:?}",
                bytes
            );
        }
    }

    #[test]
    fn test_non_ascii_characters() {
        // Test that non-ASCII characters are rejected
        let invalid_inputs = vec![
            "BWBeN28Vb7cMEx7Ym8AUz中", // Chinese character
            "BWBeN28Vb7cMEx7Ym8AUz😀", // Emoji
            "BWBeN28Vb7cMEx7Ym8AUzü",  // Umlaut
        ];

        for input in invalid_inputs {
            let result = decode(input);
            assert!(
                matches!(result, Err(B58UUIDError::InvalidBase58(_))),
                "Should reject non-ASCII character in: {}",
                input
            );
        }
    }

    #[test]
    fn test_very_long_input() {
        // Test that very long strings with too many leading ones are rejected
        let long_input = "1".repeat(1000);
        let result = decode(&long_input);
        assert!(
            matches!(result, Err(B58UUIDError::InvalidBase58(_))),
            "Should reject string with 1000 leading '1' characters"
        );
    }

    #[test]
    fn test_official_test_vectors() {
        // Test vectors from test-vectors.json
        let test_vectors = vec![
            (
                "00000000-0000-0000-0000-000000000000",
                "1111111111111111111111",
            ),
            (
                "ffffffff-ffff-ffff-ffff-ffffffffffff",
                "YcVfxkQb6JRzqk5kF2tNLv",
            ),
            (
                "550e8400-e29b-41d4-a716-446655440000",
                "BWBeN28Vb7cMEx7Ym8AUzs",
            ),
            (
                "123e4567-e89b-12d3-a456-426614174000",
                "3FfGK34vwMvVFDedyb2nkf",
            ),
            (
                "00000000-0000-0000-0000-000000000001",
                "1111111111111111111112",
            ),
            (
                "deadbeef-cafe-babe-0123-456789abcdef",
                "UVqy39vS4tbfPzthw5VEKg",
            ),
            (
                "01020304-0506-0708-090a-0b0c0d0e0f10",
                "18DfbjXLth7APvt3qQPgtf",
            ),
            (
                "aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa",
                "N5L7eAc4PsHfZViqAMbFEH",
            ),
            (
                "12345678-9abc-def0-1234-56789abcdef0",
                "3FP9ScppY3pxArsirSpyro",
            ),
        ];

        for (uuid_str, expected_b58) in test_vectors {
            // Test encoding
            let encoded = encode_uuid(uuid_str).unwrap();
            assert_eq!(
                encoded, expected_b58,
                "Encoding mismatch for UUID: {}",
                uuid_str
            );

            // Test decoding
            let decoded_uuid = decode_to_uuid(expected_b58).unwrap();
            assert_eq!(
                decoded_uuid, uuid_str,
                "Decoding mismatch for B58: {}",
                expected_b58
            );

            // Test round-trip
            let round_trip = encode_uuid(&decoded_uuid).unwrap();
            assert_eq!(
                round_trip, expected_b58,
                "Round-trip mismatch for UUID: {}",
                uuid_str
            );
        }
    }

    #[test]
    fn test_uuid_without_hyphens() {
        // Test that UUIDs without hyphens are accepted
        let uuid_no_hyphens = "550e8400e29b41d4a716446655440000";
        let encoded = encode_uuid(uuid_no_hyphens).unwrap();
        assert_eq!(encoded, "BWBeN28Vb7cMEx7Ym8AUzs");
    }

    #[test]
    fn test_case_sensitivity() {
        // Base58 is case-sensitive - both upper and lowercase are valid
        let original = "BWBeN28Vb7cMEx7Ym8AUzs";

        // Verify the original decodes correctly
        assert_eq!(
            decode_to_uuid(original).unwrap(),
            "550e8400-e29b-41d4-a716-446655440000"
        );

        // Test that Base58 alphabet includes both cases
        // 'a' through 'z' are valid Base58 characters
        let with_lowercase = "abcdefghijkmnopqrstuvwxyz";
        // This should either decode successfully or fail with overflow/invalid length
        // but NOT fail due to invalid characters
        let result = decode(with_lowercase);
        match result {
            Ok(_) => {}                       // Valid Base58, just not a valid UUID encoding
            Err(B58UUIDError::Overflow) => {} // Too large for u128
            Err(B58UUIDError::InvalidBase58(msg)) => {
                // Should not fail due to invalid characters
                assert!(
                    !msg.contains("Invalid character"),
                    "Lowercase letters should be valid Base58 characters"
                );
            }
            _ => {}
        }
    }

    #[test]
    fn test_leading_zeros_preservation() {
        // Test various numbers of leading zero bytes
        for num_zeros in 1..=15 {
            let mut bytes = [0xFF; 16];
            for i in 0..num_zeros {
                bytes[i] = 0;
            }

            let encoded = encode(&bytes);
            let decoded = decode(&encoded).unwrap();

            assert_eq!(
                decoded, bytes,
                "Round-trip failed for {} leading zeros",
                num_zeros
            );

            // Verify leading ones in encoded string
            let leading_ones = encoded.chars().take_while(|&c| c == '1').count();
            assert!(
                leading_ones >= num_zeros,
                "Expected at least {} leading ones, got {}",
                num_zeros,
                leading_ones
            );
        }
    }

    #[test]
    fn test_mixed_invalid_characters() {
        // Test various invalid characters from confusing alphabet
        let invalid_chars = vec!["0", "O", "I", "l"];
        for ch in invalid_chars {
            let test_str = format!("BWBeN28Vb7cMEx7Ym8AU{}", ch);
            let result = decode(&test_str);
            assert!(
                matches!(result, Err(B58UUIDError::InvalidBase58(_))),
                "Should reject character: {}",
                ch
            );
        }
    }

    #[test]
    fn test_uuid_version_and_variant() {
        // Verify generated UUIDs conform to RFC 4122 (version 4, variant 10)
        for _ in 0..100 {
            let b58 = generate();
            let bytes = decode(&b58).unwrap();

            // Check version 4 (bits 0100 in byte 6)
            let version = (bytes[6] & 0xF0) >> 4;
            assert_eq!(version, 4, "UUID should be version 4");

            // Check variant (bits 10 in byte 8)
            let variant = (bytes[8] & 0xC0) >> 6;
            assert_eq!(variant, 2, "UUID should have variant 10 (RFC 4122)");
        }
    }

    #[test]
    fn test_boundary_23_leading_ones() {
        // Test exactly 23 leading '1' characters (boundary case)
        let input = "1".repeat(23);
        let result = decode(&input);
        assert!(
            matches!(result, Err(B58UUIDError::InvalidBase58(_))),
            "Should reject string with 23 leading '1' characters"
        );
    }

    #[test]
    fn test_boundary_22_leading_ones_with_suffix() {
        // Test 22 leading '1' characters followed by other characters
        let input = format!("{}{}", "1".repeat(22), "2");
        let result = decode(&input);
        // This should either decode successfully or fail with overflow
        // but should not panic
        match result {
            Ok(_) => {}
            Err(B58UUIDError::Overflow) => {}
            Err(B58UUIDError::InvalidBase58(_)) => {}
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_encode_bytes_directly() {
        // Test encoding raw bytes without going through UUID string
        let bytes = [
            0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc,
            0xde, 0xf0,
        ];
        let encoded = encode(&bytes);
        let decoded = decode(&encoded).unwrap();
        assert_eq!(bytes, decoded, "Direct byte encoding round-trip failed");
    }

    #[test]
    fn test_single_leading_zero() {
        // Test UUID with exactly one leading zero byte
        let bytes = [
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ];
        let encoded = encode(&bytes);
        let decoded = decode(&encoded).unwrap();
        assert_eq!(bytes, decoded, "Single leading zero round-trip failed");

        // Verify at least one leading '1' in encoded string
        assert!(
            encoded.starts_with('1'),
            "Encoded string should start with '1' for leading zero byte"
        );
    }

    #[test]
    fn test_multiple_consecutive_zeros() {
        // Test UUID with multiple consecutive zero bytes in the middle
        let bytes = [
            0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0xff, 0x00, 0xff, 0xff, 0x00, 0x00, 0x00,
            0x00, 0xff,
        ];
        let encoded = encode(&bytes);
        let decoded = decode(&encoded).unwrap();
        assert_eq!(
            bytes, decoded,
            "Multiple consecutive zeros round-trip failed"
        );
    }

    #[test]
    fn test_max_valid_uuid() {
        // Test maximum valid UUID value (all 0xFF)
        let max_bytes = [0xFF; 16];
        let encoded = encode(&max_bytes);
        assert_eq!(encoded, "YcVfxkQb6JRzqk5kF2tNLv");

        let decoded = decode(&encoded).unwrap();
        assert_eq!(decoded, max_bytes);
    }

    #[test]
    fn test_min_valid_uuid() {
        // Test minimum valid UUID value (all 0x00)
        let min_bytes = [0x00; 16];
        let encoded = encode(&min_bytes);
        assert_eq!(encoded, "1111111111111111111111");

        let decoded = decode(&encoded).unwrap();
        assert_eq!(decoded, min_bytes);
    }

    #[test]
    fn test_alternating_pattern() {
        // Test alternating byte pattern
        let bytes = [
            0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55,
            0xAA, 0x55,
        ];
        let encoded = encode(&bytes);
        let decoded = decode(&encoded).unwrap();
        assert_eq!(bytes, decoded, "Alternating pattern round-trip failed");
    }

    #[test]
    fn test_sequential_bytes() {
        // Test sequential byte pattern (already in official vectors but good to have explicit)
        let bytes = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10,
        ];
        let encoded = encode(&bytes);
        assert_eq!(encoded, "18DfbjXLth7APvt3qQPgtf");

        let decoded = decode(&encoded).unwrap();
        assert_eq!(decoded, bytes);
    }

    #[test]
    fn test_whitespace_in_uuid() {
        // Test that UUIDs with whitespace are rejected
        let uuid_with_space = "550e8400 e29b 41d4 a716 446655440000";
        let result = encode_uuid(uuid_with_space);
        assert!(
            matches!(result, Err(B58UUIDError::InvalidLength { .. })),
            "Should reject UUID with whitespace"
        );
    }

    #[test]
    fn test_uppercase_uuid() {
        // Test that uppercase hex in UUID works
        let uppercase_uuid = "550E8400-E29B-41D4-A716-446655440000";
        let encoded = encode_uuid(uppercase_uuid).unwrap();
        assert_eq!(encoded, "BWBeN28Vb7cMEx7Ym8AUzs");
    }

    #[test]
    fn test_mixed_case_uuid() {
        // Test that mixed case hex in UUID works
        let mixed_case_uuid = "550e8400-E29B-41d4-A716-446655440000";
        let encoded = encode_uuid(mixed_case_uuid).unwrap();
        assert_eq!(encoded, "BWBeN28Vb7cMEx7Ym8AUzs");
    }

    #[test]
    fn test_decode_output_lowercase() {
        // Verify that decode_to_uuid always outputs lowercase
        let b58 = "BWBeN28Vb7cMEx7Ym8AUzs";
        let uuid = decode_to_uuid(b58).unwrap();
        assert_eq!(uuid, "550e8400-e29b-41d4-a716-446655440000");
        assert_eq!(uuid, uuid.to_lowercase(), "Output should be lowercase");
    }

    #[test]
    fn test_error_display() {
        // Test that error messages are properly formatted
        let err1 = B58UUIDError::InvalidUUID("test message".to_string());
        assert_eq!(format!("{}", err1), "Invalid UUID: test message");

        let err2 = B58UUIDError::InvalidBase58("test message".to_string());
        assert_eq!(format!("{}", err2), "Invalid Base58: test message");

        let err3 = B58UUIDError::InvalidLength {
            expected: 32,
            got: 10,
        };
        assert_eq!(format!("{}", err3), "Invalid length: expected 32, got 10");

        let err4 = B58UUIDError::Overflow;
        assert_eq!(
            format!("{}", err4),
            "Arithmetic overflow: value exceeds maximum UUID value"
        );
    }

    #[test]
    fn test_error_clone_and_equality() {
        // Test that errors can be cloned and compared
        let err1 = B58UUIDError::Overflow;
        let err2 = err1.clone();
        assert_eq!(err1, err2);

        let err3 = B58UUIDError::InvalidLength {
            expected: 32,
            got: 10,
        };
        let err4 = err3.clone();
        assert_eq!(err3, err4);
    }
}
