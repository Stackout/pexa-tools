 
// extern crate crypto;
 
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use rand::Rng;
use std::thread;

const CHARSET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ\
                        abcdefghijkmnopqrstuvwxyz\
                        123456789";

const DIGITS58: [char; 58] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
 
fn main() {
    
    let vanity = String::from("XPexaAsset");
    let place_holder = "X";

    // Generate string with amount of Xs depending on vanity size
    let remainder = 28 - vanity.chars().count();

    let mut place_holder_string = "".to_string();
    for _ in 0..remainder {
        place_holder_string += place_holder;
    }
    
    let vanity = vanity.clone();
    let place_holder_string = place_holder_string.clone();

    let thread = thread::spawn(move || {
        let mut i = 0;
        let mut valid = false;

        while !valid { 
            let checksum = random_base58(6);
            let address = format!("{}{}{}", vanity, place_holder_string, checksum);
            valid = validate_address(&address);
            i += 1;
            println!("[{}]: {} | Is valid: {}", i, address, valid);
        }
    });
    thread.join().unwrap();

    println!("12 Threads spawned!");
}

fn random_base58(length: usize) -> String {

    let mut rng = rand::thread_rng();

    let string: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    return string
}
 
fn validate_address(address: &str) -> bool {
    let decoded = match from_base58(address, 25) {
        Ok(x) => x,
        Err(_) => return false
    };
    if decoded[0] != 0 {
        return false;
    }
    let mut sha = Sha256::new();
    sha.input(&decoded[0..21]);
    let mut first_round = vec![0u8; sha.output_bytes()];
    sha.result(&mut first_round);
    sha.reset();
 
    sha.input(&first_round);
    let mut second_round = vec![0u8; sha.output_bytes()];
    sha.result(&mut second_round);
    if second_round[0..4] != decoded[21..25] {
        return false
    }
    true
}
 
fn from_base58(encoded: &str, size: usize) -> Result<Vec<u8>, String> {
    let mut res: Vec<u8> = vec![0; size];
    for base58_value in encoded.chars() {
        let mut value: u32 = match DIGITS58
            .iter()
            .position(|x| *x == base58_value){
            Some(x) => x as u32,
            None => return Err(String::from("Invalid character found in encoded string."))
        };
        for result_index in (0..size).rev() {
            value += 58 * res[result_index] as u32;
            res[result_index] = (value % 256) as u8;
            value /= 256;
        }
    }
    Ok(res)
}
 