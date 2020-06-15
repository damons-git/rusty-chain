extern crate byteorder;
extern crate sha2;
extern crate ring;

use std::any::type_name;
use rand::{RngCore};
use sha2::{Sha256, Digest};
use std::time::{SystemTime, Duration, UNIX_EPOCH};
use std::mem::size_of_val;
use byteorder::ByteOrder;


// Return the type of a variable as a string.
pub fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

// Generate and random 32-bit value.
pub fn generate_rand_data() -> [u8; 4] {
    let mut rng = rand::thread_rng();
    let mut id: [u8; 4] = [0; 4];
    rng.fill_bytes(&mut id);

    return id;
}

// Perform SHA256 hash of arbitrary binary data.
pub fn hash(binary: &Vec<u8>) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(binary);
    let result = hasher.finalize();

    let mut hash: [u8; 32] = Default::default();
    hash.copy_from_slice(&result[0..32]);
    return hash;
}

// Return 64-bit UNIX timestamp formatted as a u8 array.
pub fn get_timestamp() -> [u8; 8] {
    let duration = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Err(_) => panic!("Unable to fetch UNIX timestamp."),
        Ok(t) => t
    };

    let timestamp: u64 = duration.as_secs();
    let mut buf = [0; 8];
    byteorder::BigEndian::write_u64(&mut buf, timestamp);

    return buf;
}