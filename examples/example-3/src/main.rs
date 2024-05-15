use std::{time::Instant, u32::MAX};

use drillx::{
    difficulty,
    gpu::{drill_hash, gpu_init, set_noise},
    noise::NOISE,
};

fn main() {
    // Initialize gpu
    unsafe {
        gpu_init(MAX);
        set_noise(NOISE.as_usize_slice().as_ptr());
    }

    // Current challenge (255s for demo)
    let timer = Instant::now();
    let challenge = [255; 32];
    let mut nonce = [0; 8];
    let difficulty_up = 23;
    unsafe {
        drill_hash(challenge.as_ptr(), nonce.as_mut_ptr(), 0, difficulty_up);
    }
    println!("{nonce:?}");

    // Calculate hash
    let hx = drillx::hash(&challenge, &nonce);
    println!(
        "gpu found hash with difficulty {} in {} seconds: {}",
        difficulty(hx),
        timer.elapsed().as_secs(),
        bs58::encode(hx).into_string(),
    );
}
