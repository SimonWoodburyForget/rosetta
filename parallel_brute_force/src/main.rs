use parallel_brute_force::*;
use sha2::{Digest, Sha256};

fn main() {
    let (_, data, _) = unsafe { &[1].align_to() };
    println!("{:?}", Sha256::digest(data));

    let data = &[1];
    println!("{:?}", Sha256::digest(data));
}
