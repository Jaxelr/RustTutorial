use std::str;
use percent_encoding::{utf8_percent_encode, percent_decode, AsciiSet, CONTROLS};
use std::str::Utf8Error;
use base64::{encode, decode};
use data_encoding::{HEXUPPER, DecodeError};
use url::form_urlencoded::{byte_serialize, parse};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

fn main() -> Result<(), Box <dyn std::error::Error>> {
    percent_encoding()?;
    url_encoding();
    data_encoding()?;
    base_64_encoding()?;

    Ok(())
}

fn percent_encoding() -> Result<(), Utf8Error> {
    let input = "confident, productive systems programming";

    let iter = utf8_percent_encode(input, FRAGMENT);
    let encoded: String = iter.collect();
    assert_eq!(encoded, "confident,%20productive%20systems%20programming");

    let iter = percent_decode(encoded.as_bytes());
    let decoded = iter.decode_utf8()?;
    assert_eq!(decoded, "confident, productive systems programming");

    Ok(())
}

fn url_encoding() -> () {
    let urlencoded: String = byte_serialize("What is ❤?".as_bytes()).collect();
    assert_eq!(urlencoded, "What+is+%E2%9D%A4%3F");
    println!("urlencoded:'{}'", urlencoded);

    let decoded: String = parse(urlencoded.as_bytes())
        .map(|(key, val)| [key, val].concat())
        .collect();
    assert_eq!(decoded, "What is ❤?");
    println!("decoded:'{}'", decoded);
}

fn data_encoding() -> Result<(), DecodeError> {
    let original = b"The quick brown fox jumps over the lazy dog.";
    let expected = "54686520717569636B2062726F776E20666F78206A756D7073206F76\
        657220746865206C617A7920646F672E";

    let encoded = HEXUPPER.encode(original);
    assert_eq!(encoded, expected);

    let decoded = HEXUPPER.decode(&encoded.into_bytes())?;
    assert_eq!(&decoded[..], &original[..]);

    Ok(())
}

fn base_64_encoding() -> Result<(), Box <dyn std::error::Error>> {
    let hello = b"hello rustaceans";
    let encoded = encode(hello);
    let decoded = decode(&encoded)?;

    println!("origin: {}", str::from_utf8(hello)?);
    println!("base64 encoded: {}", encoded);
    println!("back to origin: {}", str::from_utf8(&decoded)?);

    Ok(())
}