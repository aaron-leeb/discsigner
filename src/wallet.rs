use bitcoin::bip32::{Error, Xpub};

pub fn parse_xpub(input: &str) -> Result<Xpub, Error> {
    input.parse::<Xpub>()
}

pub fn serialize_xpub(xpub: &Xpub) -> Vec<u8> {
    xpub.to_string().into_bytes()
}

pub fn deserialize_xpub(data: &[u8],) -> Result<Xpub, Box<dyn std::error::Error>> {
    let input = std::str::from_utf8(data)?;
    Ok(parse_xpub(input)?)
}