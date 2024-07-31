use base64::prelude::*;
use hex;
use std::{env, process};
use sha2::{Sha256, Digest};

fn main() {
    let args: Vec<String> = env::args().collect();
    let seed: String = args[1].clone();
    if seed.chars().count() != 1 {
        println!("Seed must be 1 character long!");
        process::exit(1);
    }
    let mut hasher = Sha256::new();
    hasher.update(seed);
    let mut result = format!("{:x}", hasher.finalize());

    result = result.chars().rev().collect();
    result = BASE64_STANDARD.encode(result);
    result = hex::encode(result).chars().rev().collect();

    println!("{}", result);
}
