use crypt_xor;

extern crate rand;
use rand::prelude::*;

fn encrypt_random_data_test(
    block_size: usize,
    crypt_func: &mut dyn FnMut(&mut Vec<u8>, &Vec<u8>, usize),
) {
    let mut buf = vec![0u8; block_size];
    let mut key = vec![0u8; block_size];

    // Fill both vectors with random numbers
    rand::thread_rng().fill_bytes(&mut buf);
    rand::thread_rng().fill_bytes(&mut key);

    // Make a copy of the original plain text so we can compare it to the buffer after it is "encrypted"
    let original_plaintext = buf.to_vec();

    // Encrypt `buf`
    crypt_func(&mut buf, &key, block_size);

    // Iterate over both original plaintext and ciphertext and compare each byte
    for (x, y) in original_plaintext.iter().zip(buf.iter()) {
        assert!(x == y);
    }
}

/// Test the `crypt_double_xor_in_place` function
#[test]
fn test_crypt_double_xor_in_place() {
    let block_size = 64;

    // Run the test a few times
    encrypt_random_data_test(block_size, &mut crypt_xor::crypt_double_xor_in_place);
    encrypt_random_data_test(block_size, &mut crypt_xor::crypt_double_xor_in_place);
    encrypt_random_data_test(block_size, &mut crypt_xor::crypt_double_xor_in_place);
    encrypt_random_data_test(block_size, &mut crypt_xor::crypt_double_xor_in_place);
    encrypt_random_data_test(block_size, &mut crypt_xor::crypt_double_xor_in_place);
}

/// Test the `crypt_triple_double_xor_in_place` function
#[test]
fn test_crypt_triple_double_xor_in_place() {
    let block_size = 64;

    // Run the test a few times
    encrypt_random_data_test(block_size, &mut crypt_xor::crypt_triple_double_xor_in_place);
    encrypt_random_data_test(block_size, &mut crypt_xor::crypt_triple_double_xor_in_place);
    encrypt_random_data_test(block_size, &mut crypt_xor::crypt_triple_double_xor_in_place);
    encrypt_random_data_test(block_size, &mut crypt_xor::crypt_triple_double_xor_in_place);
    encrypt_random_data_test(block_size, &mut crypt_xor::crypt_triple_double_xor_in_place);
}
