extern crate byteorder;
extern crate sha2;
extern crate ring;

use std::any::type_name;
use rand::{RngCore};
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};
use byteorder::ByteOrder;
use std::net::{SocketAddr, IpAddr, Ipv4Addr, Ipv6Addr};


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

// Parse a socket address string to a corresponding SocketAddr struct.
pub fn parse_net_address(ip_str: &str) -> SocketAddr {
    let segments = ip_str.split(":").collect::<Vec<&str>>();

    if segments.len() == 2 {
        return parse_ipv4(ip_str);
    }
    else {
        return parse_ipv6(ip_str);
    }
}

fn parse_ipv4(addr: &str) -> SocketAddr {
    let split = addr.split(":").collect::<Vec<&str>>();
    let port = split[1];
    let seg_str = split[0].split(".").collect::<Vec<&str>>();
    let num: Vec<u8> = seg_str.into_iter().map(|x| x.parse::<u8>().unwrap()).collect();

    return SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(num[0], num[1], num[2], num[3])),
        port.parse::<u16>().unwrap()
    );
}

fn parse_ipv6(ip_str: &str) -> SocketAddr {
    let mut seg_str = ip_str.split(":").collect::<Vec<&str>>();
    let port = seg_str[8].parse::<u16>().unwrap();
    seg_str.pop();
    let num: Vec<u16> = seg_str.into_iter().map(|x| u16::from_str_radix(x, 16).unwrap()).collect();

    return SocketAddr::new(
        IpAddr::V6(Ipv6Addr::new(num[0], num[1], num[2], num[3], num[4], num[5], num[6], num[7])),
        port as u16
    );
}