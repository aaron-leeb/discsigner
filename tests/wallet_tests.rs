use discsigner::wallet::{
    parse_xpub,
    serialize_xpub,
    deserialize_xpub,
};

const TEST_XPUB: &str = "xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8";

#[test]
fn test_xpub_round_trip() {
    let original = parse_xpub(TEST_XPUB).unwrap();
    let serialized = serialize_xpub(&original);
    let recovered = deserialize_xpub(&serialized).unwrap();

    assert_eq!(TEST_XPUB.as_bytes(), serialized.as_slice());
    assert_eq!(original, recovered);
}

#[test]
fn test_invalid_xpub() {
    assert!(parse_xpub("invalid").is_err());
}

#[test]
fn test_invalid_serialized_xpub() {
    assert!(deserialize_xpub(b"invalid").is_err());
}

#[test]
fn test_invalid_utf8() {
    assert!(deserialize_xpub(&[0xff, 0xfe]).is_err());
}