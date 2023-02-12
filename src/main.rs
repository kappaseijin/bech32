/**
 * This is へいほぅ's code. @kappaseijin is created cargo.
 * https://snort.social/e/note15a4jswlchgfs449xgx7pxgcgl25fps0evfp0yv0ad5gcundsxj4qwcd7wn
 */
use bech32::{ToBase32, Variant, encode}; // not-m
use hex::decode;

fn main() {
    let s = "your hex key"; // nostr's hex of pubkey.
    let encoded = encode(
        "npub", // nostr's public key's prefix.
        decode(s).unwrap().to_base32(),
        Variant::Bech32,
    ).unwrap();
    println!("{}", encoded);
}

