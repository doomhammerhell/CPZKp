//! Integration tests for the session module
//! 
//! This module contains integration tests that verify the interaction
//! between different components of the library, particularly focusing
//! on the session management and multi-round proof functionality.

use cpzkp::*;
use num_bigint::BigUint;

#[test]
fn test_multi_round_session() {
    // Initialize a new session
    let mut session = Session::new(Group::Scalar).unwrap();
    
    // Start first round
    let (r1_1, r2_1) = session.next_round().unwrap();
    assert!(r1_1.is_on_curve());
    assert!(r2_1.is_on_curve());
    
    // Generate challenge for first round
    let challenge1 = session.generate_challenge(0).unwrap();
    
    // Solve first round
    let s1 = session.solve_challenge(0, &challenge1).unwrap();
    
    // Verify first round
    assert!(session.verify_round(0).unwrap());
    
    // Start second round
    let (r1_2, r2_2) = session.next_round().unwrap();
    assert!(r1_2.is_on_curve());
    assert!(r2_2.is_on_curve());
    
    // Generate challenge for second round
    let challenge2 = session.generate_challenge(1).unwrap();
    
    // Solve second round
    let s2 = session.solve_challenge(1, &challenge2).unwrap();
    
    // Verify second round
    assert!(session.verify_round(1).unwrap());
    
    // Finalize session
    assert!(session.finalize().unwrap());
}

#[test]
fn test_session_serialization() {
    // Create and initialize a session
    let mut session = Session::new(Group::Scalar).unwrap();
    session.next_round().unwrap();
    let challenge = session.generate_challenge(0).unwrap();
    session.solve_challenge(0, &challenge).unwrap();
    
    // Serialize the session
    let serialized = session.serialize().unwrap();
    
    // Deserialize the session
    let deserialized = Session::deserialize(&serialized).unwrap();
    
    // Verify the deserialized session
    assert!(deserialized.verify_round(0).unwrap());
}

#[test]
#[should_panic(expected = "Invalid round index")]
fn test_invalid_round_index() {
    let mut session = Session::new(Group::Scalar).unwrap();
    session.next_round().unwrap();
    
    // This should panic
    session.generate_challenge(1).unwrap();
}

#[test]
#[should_panic(expected = "Session not finalized")]
fn test_unfinalized_session() {
    let mut session = Session::new(Group::Scalar).unwrap();
    session.next_round().unwrap();
    
    // This should panic
    session.serialize().unwrap();
}

#[test]
fn test_session_with_different_groups() {
    // Test with scalar group
    let mut scalar_session = Session::new(Group::Scalar).unwrap();
    scalar_session.next_round().unwrap();
    let scalar_challenge = scalar_session.generate_challenge(0).unwrap();
    scalar_session.solve_challenge(0, &scalar_challenge).unwrap();
    assert!(scalar_session.verify_round(0).unwrap());
    
    // Test with elliptic curve group
    let mut ecc_session = Session::new(Group::EllipticCurve).unwrap();
    ecc_session.next_round().unwrap();
    let ecc_challenge = ecc_session.generate_challenge(0).unwrap();
    ecc_session.solve_challenge(0, &ecc_challenge).unwrap();
    assert!(ecc_session.verify_round(0).unwrap());
} 