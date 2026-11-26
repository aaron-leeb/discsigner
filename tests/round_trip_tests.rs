use discsigner::{audio, wallet};

const TEST_XPUB: &str = "xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8";

#[test]
fn test_xpub_audio_round_trip() {
    let original = wallet::parse_xpub(TEST_XPUB).unwrap();

    let serialized = wallet::serialize_xpub(&original);

    let samples = audio::encode(&serialized);

    let recovered_bytes = audio::decode(&samples).unwrap();

    let recovered = wallet::deserialize_xpub(&recovered_bytes).unwrap();

    assert_eq!(serialized, recovered_bytes);
    assert_eq!(original, recovered);
}