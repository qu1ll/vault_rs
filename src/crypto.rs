use sodiumoxide::crypto::{pwhash, secretbox};
use std::{
    fs,
    io::{self},
};

fn derive_key(passwd: &[u8]) -> io::Result<secretbox::Key>{

    // Read salt to derive key from
    let data = fs::read(".env")?;
    let salt_slice = &data;
    let salt = pwhash::Salt::from_slice(salt_slice).unwrap();

    // Derive key from password and salt
    let mut key = secretbox::Key([0; secretbox::KEYBYTES]);
    {
        let secretbox::Key(ref mut kb) = key;
        pwhash::derive_key(
            kb,
            passwd,
            &salt,
            pwhash::OPSLIMIT_INTERACTIVE,
            pwhash::MEMLIMIT_INTERACTIVE,
        )
        .unwrap();
    }

    Ok(key) // Return derived key
}

pub(crate) fn encrypt_data(text: String) -> io::Result<Vec<u8>> {
    let passwd = b"YOUR PASSWORD HERE"; // Temp password for testing

    let key = derive_key(passwd)?; // Derive key from password

    let nonce = secretbox::gen_nonce(); // Generate a random nonce (similar to an IV)

    let plaintext = text.as_bytes(); // Convert input text into bytes
    let ciphertext = secretbox::seal(plaintext, &nonce, &key); // Convert input text into ciphertext

    // Combine nonce and ciphertext to be stored
    let mut combined = Vec::with_capacity(secretbox::NONCEBYTES + ciphertext.len());
    combined.extend_from_slice(nonce.as_ref());
    combined.extend_from_slice(&ciphertext);

    Ok(combined) // Return combined nonce and ciphertext
}

pub(crate) fn decrypt_data(text: String) -> io::Result<Vec<u8>> {
    let passwd = b"YOUR PASSWORD HERE"; // Temp password for testing

    let key = derive_key(passwd)?; // Derive key from password

    let data = fs::read(format!("passwords/{}", text))?; // Path to encrypted data

    let (_nonce, ciphertext) = data.split_at(secretbox::NONCEBYTES); // Extract the ciphertext from the end of the data

    // Extract the nonce from the end of the data
    let nonce_slice = &data[..secretbox::NONCEBYTES];
    let nonce = secretbox::Nonce::from_slice(nonce_slice).unwrap();

    let decrypted = secretbox::open(&ciphertext, &nonce, &key).unwrap();

    Ok(decrypted) // Return decrypted data
}
