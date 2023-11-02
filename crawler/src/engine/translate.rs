use encoding_rs::*;
use std::io::Read;

pub async fn decode(input: &str, encoding: &'static Encoding) -> String {
    let buf = input.as_bytes();
    let mut dec = encoding_rs_io::DecodeReaderBytesBuilder::new()
        .encoding(Some(encoding))
        .build(buf);
    let mut res = String::new();
    dec.read_to_string(&mut res).unwrap();
    res
}

#[cfg(test)]
#[tokio::test]
async fn test_decode() {
    let input = "�D�O��R�W";
    let decoded_str = decode(input, encoding_rs::UTF_8).await;
    let decode_2 = decode(decoded_str.as_str(), encoding_rs::BIG5).await;
    assert_eq!(decode_2, "測試");
}
