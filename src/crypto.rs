use sodiumoxide::crypto::{pwhash, secretbox};
use std::{
    fs,
    io::{self, Write},
};

pub(crate) fn encrypt_data(text: String) -> io::Result<Vec<u8>> {
    let passwd = b"YOUR PASSWORD HERE";

    let data = fs::read(".env")?;
    let salt_slice = &data;
    let salt = pwhash::Salt::from_slice(salt_slice).unwrap();

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

    // Generate a random nonce (similar to an IV)
    let nonce = secretbox::gen_nonce();

    let plaintext = text.as_bytes();
    let ciphertext = secretbox::seal(plaintext, &nonce, &key);

    let mut combined = Vec::with_capacity(secretbox::NONCEBYTES + ciphertext.len());
    combined.extend_from_slice(nonce.as_ref());
    combined.extend_from_slice(&ciphertext);

    Ok(combined)
}

pub(crate) fn decrypt_data(text: String) -> io::Result<Vec<u8>> {
    let passwd = b"YOUR PASSWORD HERE";

    let data = fs::read(".env")?;
    let salt_slice = &data;
    let salt = pwhash::Salt::from_slice(salt_slice).unwrap();
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

    let data = fs::read(format!("passwords/{}", text))?; // Path to encrypted data

    let (_nonce, ciphertext) = data.split_at(secretbox::NONCEBYTES); // Extract the ciphertext from the end of the data

    // Extract the nonce from the end of the data
    let nonce_slice = &data[..secretbox::NONCEBYTES];
    let nonce = secretbox::Nonce::from_slice(nonce_slice).unwrap();

    let decrypted = secretbox::open(&ciphertext, &nonce, &key).unwrap();

    let mut file = fs::File::create("temp.txt")?;
    file.write_all(&decrypted)?;

    Ok(decrypted)
}
