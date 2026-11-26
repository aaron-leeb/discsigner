use discsigner::audio::{encode, decode};

#[test]
fn test_single_byte_round_trip() {
    let original = b"x";

    let samples = encode(original);
    let recovered = decode(&samples).unwrap();

    assert_eq!(original.as_slice(), recovered.as_slice());
}

#[test]
fn test_message_round_trip() {
    let original = b"Hello, Bitcoin!";

    let samples = encode(original);
    let recovered = decode(&samples).unwrap();

    assert_eq!(original.as_slice(), recovered.as_slice());
}

#[test]
fn test_all_byte_values() {
    let original: Vec<u8> = (0..=255).collect();

    let samples = encode(&original);
    let recovered = decode(&samples).unwrap();

    assert_eq!(original, recovered);
}

#[test]
fn test_invalid_sample_length() {
    assert!(decode(&[0; 100]).is_err());
}

#[test]
fn test_empty_input() {
    assert!(encode(&[]).is_empty());
    assert!(decode(&[]).unwrap().is_empty());
}

#[test]
fn test_encoded_sample_count() {
    let samples = encode(b"x");

    assert_eq!(samples.len(), 3528);
}