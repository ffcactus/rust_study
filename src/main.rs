use aes::cipher::{KeyIvInit, StreamCipher};

type Aes128Ctr64LE = ctr::Ctr64LE<aes::Aes128>;
use hex_literal::hex;

fn main() {


    let key = [0x42; 16];
    let iv = [0x24; 16];
    let plaintext = *b"hello world! this is my plaintext.";
    let ciphertext = hex!(
    "3357121ebb5a29468bd861467596ce3da59bdee42dcc0614dea955368d8a5dc0cad4"
);

// encrypt in-place
    let mut buf = plaintext.to_vec();
    let mut cipher = Aes128Ctr64LE::new(&key.into(), &iv.into());
    cipher.apply_keystream(&mut buf);

    // Convert the key, IV, and ciphertext to Base64 encoding
    let key_base64 = base64::encode(&key);
    let iv_base64 = base64::encode(&iv);
    let ciphertext_base64 = base64::encode(&buf);

    // Print the key, IV, and ciphertext in Base64 encoding
    println!("Key: {}", key_base64);
    println!("IV: {}", iv_base64);
    println!("Ciphertext: {}", ciphertext_base64);

    assert_eq!(buf[..], ciphertext[..]);
}