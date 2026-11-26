use bitcoin::Network;
use discsigner::wallet;
use discsigner::audio;
use std::env;

fn main() {
    let network = Network::Signet;
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("help") | None => print_help(),
        Some("network") => {
            println!("Network: {}", network);
        }
        Some("inspect-xpub") => match args.get(2) {
            Some(key) => inspect_xpub(key),
            None => eprintln!("Usage: inspect-xpub <key>"),
        },
        Some("encode") => {
            match (args.get(2), args.get(3)) {
                (Some(data), Some(path)) => encode_audio(data, path),
                _ => eprintln!("Usage: encode <data> <output.wav>"),
            }
        }
        Some("decode") => {
            match args.get(2) {
                Some(path) => decode_audio(path),
                None => eprintln!("Usage: decode <input.wav>"),
            }
        }
        Some(command) => {
            eprintln!("Unknown command: {}", command);
            print_help();
        }
    }
}

fn print_help() {
    println!("DiscSigner v0.1.0");
    println!("\nAvailable commands:");
    println!("  help                        Display available commands");
    println!("  network                     Displaythe current network");
    println!("  inspect-xpub <key>          Inspect an extended public key");
    println!("  encode <data> <output.wav>  Encode data into audio");
    println!("  decode <input.wav>          Decode audio into data");
}

fn inspect_xpub(input: &str) {
    match wallet::parse_xpub(input) {
        Ok(xpub) => {
            println!("Network: {:?}", xpub.network);
            println!("Depth: {}", xpub.depth);
            println!("Parent Fingerprint: {:?}", xpub.parent_fingerprint);
            println!("Child Number: {}", xpub.child_number);
            println!("Chain Code: {:?}", xpub.chain_code);
            println!("Public Key: {:?}", xpub.public_key);
        }
        Err(error) => eprintln!("Invalid extended public key: {}", error),
    }
}

fn encode_audio(data: &str, path: &str) {
    let samples = audio::encode(data.as_bytes());

    match audio::write_wav(path, &samples) {
        Ok(()) => {
            println!("Encoded {} bytes", data.len());
            println!("Generated {} samples", samples.len());
            println!("Duration: {:.2} seconds",
                samples.len() as f64 / audio::SAMPLE_RATE as f64
            );
            println!("Audio exported to {}", path);
        }
        Err(error) => eprintln!("Export failed: {}", error),
    }
}

fn decode_audio(path: &str) {
    let result = audio::read_wav(path)
        .map_err(|e| e.to_string())
        .and_then(|samples| {
            audio::decode(&samples).map_err(|e| e.to_string())
        })
        .and_then(|bytes| {
            String::from_utf8(bytes).map_err(|e| e.to_string())
        });

    match result {
        Ok(data) => println!("Decoded data: {}", data),
        Err(error) => eprintln!("Decoding failed: {}", error),
    }
}