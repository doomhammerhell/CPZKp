use cpzkp::{Group, Point, get_constants, solve_zk_challenge_s, verify};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::Signature;
use num_bigint::BigUint;
use secp256k1::{SecretKey, PublicKey};
use std::str::FromStr;

/// Example showing how to integrate CPZKp with Ethereum
/// This example demonstrates:
/// 1. Generating an Ethereum key pair
/// 2. Signing a message with the key
/// 3. Creating a ZKP about the key
/// 4. Verifying the proof
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate an Ethereum wallet
    let wallet = LocalWallet::new(&mut rand::thread_rng());
    let address = wallet.address();
    println!("Generated Ethereum address: {:?}", address);

    // Sign a message
    let message = "Hello, Ethereum!";
    let signature = wallet.sign_message(message).await?;
    println!("Message signature: {:?}", signature);

    // Convert the private key to a BigUint for ZKP
    let private_key = wallet.signer().to_bytes();
    let x_secret = BigUint::from_bytes_be(&private_key);

    // Generate ZKP parameters
    let group = Group::EllipticCurve;
    let (p, q, g, h) = get_constants(&group)?;

    // Generate public values
    let (y1, y2) = exponentiates_points(&x_secret, &g, &h, &p)?;

    // Generate proof
    let k = BigUint::from_bytes_be(&rand::random::<[u8; 32]>());
    let (r1, r2) = exponentiates_points(&k, &g, &h, &p)?;

    // Generate challenge
    let c = BigUint::from_bytes_be(&rand::random::<[u8; 32]>());

    // Solve challenge
    let s = solve_zk_challenge_s(&x_secret, &k, &c, &q);

    // Verify the proof
    let verification = verify(&r1, &r2, &y1, &y2, &g, &h, &c, &s, &p)?;
    println!("Proof verification: {}", verification);

    // Print the proof details
    println!("\nProof Details:");
    println!("Public Key (y1): {:?}", y1);
    println!("Public Key (y2): {:?}", y2);
    println!("Commitment (r1): {:?}", r1);
    println!("Commitment (r2): {:?}", r2);
    println!("Challenge (c): {:?}", c);
    println!("Response (s): {:?}", s);

    Ok(())
} 