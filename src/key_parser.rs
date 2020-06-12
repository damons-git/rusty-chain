// Parse and extract public key from keyfile and return in DER format.
pub fn get_public_der(key_data: &Vec<u8>) -> Vec<u8> {

    let _asn1_seq = &key_data[0..2];
    let _seq_len  = &key_data[2..4];
    let mut pos: usize = 4; // Start index of first data segment.

    // Function to return the next section of ASN.1 encoded data.
    fn next_section<'a>(key_data: &'a Vec<u8>, pos: &mut usize) -> &'a [u8] {
        let separator = parse_separator(key_data, *pos);
        let end_pos = separator.0 + separator.1 as usize;
        let val = &key_data[(separator.0)..end_pos];
        *pos = end_pos;
        return val;
    }

    let _alg = next_section(key_data, &mut pos);
    let modulus = next_section(key_data, &mut pos);
    let pub_exp = next_section(key_data, &mut pos);
    let _priv_exp = next_section(key_data, &mut pos);
    let _prime_one = next_section(key_data, &mut pos);
    let _prime_two = next_section(key_data, &mut pos);
    let _exp_one = next_section(key_data, &mut pos);
    let _exp_two = next_section(key_data, &mut pos);
    let _coefficient = next_section(key_data, &mut pos);

    return export_public_der(&modulus.to_vec(), &pub_exp.to_vec());
}

// Parse an ASN.1 separator.
// Returns a tuple containing data start index and length in bytes.
fn parse_separator(data: &Vec<u8>, pos: usize) -> (usize, u32) {
    assert!(data[pos] == 0x02);

    let multibyte_seq: bool = if data[pos + 1] >= 128 { true } else { false };
    let mut total_bytes = 0;

    let data_length: u32 = if !multibyte_seq {
        data[pos + 1] as u32
    } else {
        let mut acc: u32 = 0;
        total_bytes = data[pos + 1] - 128;
        for i in 1..(total_bytes + 1) {
            let val: u32 = data[pos + 1 + i as usize] as u32;
            acc = acc << 8;
            acc += val;
        }
        acc
    };

    let data_pos = pos + (2 + total_bytes as u32) as usize;
    return (data_pos, data_length);
}

// Converts required key components into valid RSA ASN.1 DER encoding.
fn export_public_der(modulus: &Vec<u8>, pub_exp: &Vec<u8>) -> Vec<u8> {
    let mut public_der: Vec<u8> = vec![];

    let header: [u8; 1] = [0x30];

    let pub_exp_seg: Vec<u8> = if pub_exp.len() >= 128 {
        let mut separator: Vec<u8> = vec![0x2, 0x82];
        let len = (pub_exp.len() as u16).to_be_bytes();
        separator.extend_from_slice(&len);
        separator
    } else {
        vec![2, pub_exp.len() as u8]
    };

    let modulus_seg: Vec<u8> = if modulus.len() >= 128 {
        let mut separator: Vec<u8> = vec![0x2, 0x82];
        let len = (modulus.len() as u16).to_be_bytes();
        separator.extend_from_slice(&len);
        separator
    } else {
        vec![2, modulus.len() as u8]
    };

    let total_len: u16 = (modulus_seg.len() + modulus.len() + pub_exp_seg.len() + pub_exp.len()) as u16;

    let total_size_seg = if total_len >= 128 {
        let mut separator: Vec<u8> = vec![0x82];
        separator.push((total_len >> 8) as u8);
        separator.push(total_len as u8);
        separator
    } else {
        vec![total_len as u8]
    };

    public_der.extend_from_slice(&header);
    public_der.extend_from_slice(&total_size_seg);
    public_der.extend_from_slice(&modulus_seg);
    public_der.extend_from_slice(&modulus);
    public_der.extend_from_slice(&pub_exp_seg);
    public_der.extend_from_slice(&pub_exp);

    return public_der;
}