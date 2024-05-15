use std::iter::Sum;

// use std::{intrinsics::discriminant_value, time::Instant};
use rand::Rng;
// const TARGET_DIFFICULTY: u32 = 4; // 12; // 8; //10;

fn main() {
    // Current challenge (255s for demo)
    let nonce_bound1 = 1000;
    let nonce_bound2 = 8000;
    let mut arr  = [0u32; 8];
    // Do work
    // let work_timer = Instant::now();
    for _ in 0..100{
        let mut rng = rand::thread_rng();
        let mut challenge = [0u8; 32];
        rng.fill(&mut challenge);
        for i in 0..8 {
            let mut challenge_sub = [0u8; 32];
            rng.fill(&mut challenge_sub);
            let difficulty_sub = do_work(challenge_sub, nonce_bound1);
            arr[i] = difficulty_sub;
        }
        let difficulty = do_work(challenge, nonce_bound2);
        let transformed: Vec<f64> = arr.iter().map(|&x| 2_u32.pow(x as u32) as f64).collect();
        let sum: f64 = transformed.iter().sum();
        let log_sum = sum.log2();
        println!("difficulty for 8000:{} max:{} average:{} 8000 is better:{} enum: {} {} {} {} {} {} {} {}", difficulty, arr.iter().max().unwrap(), 
        log_sum, difficulty as f64>log_sum, arr[0], arr[1], arr[2], arr[3], arr[4], arr[5], arr[6], arr[7] );
    
    }
    
    // Now proof
    // let proof_timer = Instant::now();
    // prove_work(challenge, nonce);
    // println!("proof done in {} nanos", proof_timer.elapsed().as_nanos());
    // println!(
    //     "work took {}x vs proof",
    //     work_timer.elapsed().as_nanos() / proof_timer.elapsed().as_nanos()
    // );
}

// TODO Parallelize
fn do_work(challenge: [u8; 32], nonce_bound: u64) -> u32 {
    let mut nonce: u64 = 0;
    let mut best_difficuty: u32 = 0;
    loop {
        // Calculate hash
        let hx = drillx::hash(&challenge, &nonce.to_le_bytes());
        let difficulty = drillx::difficulty(hx);
        if best_difficuty.le(&difficulty) {
            best_difficuty = difficulty;
        }
        // Return if difficulty was met
        // if drillx::difficulty(hx) >= TARGET_DIFFICULTY {
        //     break;
        // }
        if nonce.gt(&nonce_bound) {
            break;
        }

        // Increment nonce
        nonce += 1;
    }
    // nonce as u64
    best_difficuty
}

// fn prove_work(challenge: [u8; 32], nonce: u64) {
//     let candidate = drillx::hash(&challenge, &nonce.to_le_bytes());
//     println!("candidate hash {candidate:?}");
//     assert!(drillx::difficulty(candidate) >= TARGET_DIFFICULTY);
// }
