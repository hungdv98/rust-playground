use rand::{rng, RngCore};
use hex;
fn main() {
    const KEY_LENGTH: usize = 32;

    let mut key_bytes = [0u8; KEY_LENGTH];
    let mut rng = rng();

    rng.fill_bytes(&mut key_bytes);
    
    println!("HS256 (bytes): {:?}", key_bytes);
    println!("HS256 (hex): {:}", hex::encode(&key_bytes));
}