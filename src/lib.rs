#![forbid(unsafe_code)]

extern crate rand;
use rand::prelude::*;

use std::fs::File;
use std::io::prelude::*;

use indicatif::{ProgressBar, ProgressStyle};

/// "Encrypt the first `count` bytes of `plaintext` by xor'ing them with `key` twice."
/// 
/// The first thing to notice is that the function is marked as `pub`. This means that it is public and
/// can be called from outside the module
/// 
/// Arguments:
/// 
/// * `plaintext`: The plaintext to encrypt.
/// * `key`: The key to use for encryption.
/// * `count`: The number of bytes to encrypt.
pub fn crypt_double_xor_in_place(plaintext: &mut Vec<u8>, key: &Vec<u8>, count: usize) {
    assert!(plaintext.len() <= key.len());
    assert!(plaintext.len() >= count);

    // Encrypt once by xor'ing plaintext with key
    for (p, k) in plaintext.iter_mut().take(count).zip(key.iter()) {
        *p ^= *k;
    }

    // Xor again for extra security
    for (p, k) in plaintext.iter_mut().take(count).zip(key.iter()) {
        *p ^= *k;
    }
}

/// XOR the plaintext with the key three times.
/// 
/// Arguments:
/// 
/// * `plaintext`: The plaintext to be encrypted.
/// * `key`: The key to use for encryption.
/// * `count`: The number of times to encrypt the plaintext.
pub fn crypt_triple_double_xor_in_place(plaintext: &mut Vec<u8>, key: &Vec<u8>, count: usize) {
    crypt_double_xor_in_place(plaintext, key, count);
    crypt_double_xor_in_place(plaintext, key, count);
    crypt_double_xor_in_place(plaintext, key, count);
}

/// It reads a file in chunks of `block_size` bytes, encrypts each chunk using the `crypt_func`
/// function, and writes the encrypted chunk to the output file
/// 
/// Arguments:
/// 
/// * `input_path`: The path to the file to be encrypted/decrypted
/// * `output_path`: The path to the output file
/// * `block_size`: The size of the block to read and write at a time.
/// * `crypt_func`: A function that takes a buffer, a key, and a count, and crypts the buffer.
/// 
/// Returns:
/// 
/// A Result<(), std::io::Error>
pub fn read_and_crypt(
    input_path: &str,
    output_path: &str,
    block_size: usize,
    crypt_func: &mut dyn FnMut(&mut Vec<u8>, &Vec<u8>, usize),
) -> std::io::Result<()> {
    let mut input_file = File::open(input_path)?;
    let mut output_file = File::create(output_path)?;

    // Set up progress bar
    let total_size = match input_file.metadata() {
        Ok(metadata) => metadata.len(),
        Err(_) => 0, // TODO: Consider printing error message here
    };
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-"));

    // Random key for maximum security
    let mut buf = vec![0u8; block_size];
    let mut key = vec![0u8; block_size];
    rand::thread_rng().fill_bytes(&mut key);

    let mut bytes_processed = 0usize;
    loop {
        let count = input_file.read(&mut buf)?;
        if count == 0 {
            break;
        }
        crypt_func(&mut buf, &key, count);
        output_file.write_all(&buf[..count])?;

        if total_size > 0 {
            bytes_processed = bytes_processed + count;
            pb.set_position(bytes_processed as u64);
        }
    }

    Ok(())
}
