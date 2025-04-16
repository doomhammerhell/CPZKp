//! Unit tests for the scalar module
//! 
//! This module contains tests for basic scalar operations, including:
//! - Group operations
//! - Point operations
//! - Zero-knowledge proof operations

use cpzkp::scalar::*;
use cpzkp::types::*;
use num_bigint::BigUint;

#[test]
fn test_group_ops() {
    let group = ScalarGroup::new();
    
    // Test prime
    let prime = group.prime();
    assert!(prime > BigUint::from(0u32));
    
    // Test order
    let order = group.order();
    assert!(order > BigUint::from(0u32));
    
    // Test generators
    let g = group.generator();
    let h = group.second_generator();
    assert!(g != h);
}

#[test]
fn test_point_ops() {
    let group = ScalarGroup::new();
    let point = group.generator();
    
    // Test point operations
    assert!(point.is_on_curve());
    
    let doubled = point.double();
    assert!(doubled.is_on_curve());
    
    let scaled = point.scale(BigUint::from(2u32));
    assert!(scaled.is_on_curve());
}

#[test]
fn test_zkp_ops() {
    let group = ScalarGroup::new();
    
    // Test challenge generation
    let challenge = group.generate_challenge().unwrap();
    assert!(challenge > BigUint::from(0u32));
    
    // Test proof generation and verification
    let secret = BigUint::from(1234u32);
    let random = BigUint::from(5678u32);
    let s = group.solve_challenge(&secret, &random, &challenge);
    
    let params = VerificationParams {
        r1: group.generator().scale(random.clone()),
        r2: group.second_generator().scale(random),
        y1: group.generator().scale(secret.clone()),
        y2: group.second_generator().scale(secret),
        g: group.generator(),
        h: group.second_generator(),
        c: challenge,
        s: s,
    };
    
    assert!(group.verify_proof(&params).unwrap());
}

#[test]
#[should_panic(expected = "Invalid point")]
fn test_invalid_point() {
    let group = ScalarGroup::new();
    let invalid_point = Point {
        x: BigUint::from(0u32),
        y: BigUint::from(0u32),
    };
    
    // This should panic
    invalid_point.is_on_curve();
}

#[test]
#[should_panic(expected = "Invalid scalar")]
fn test_invalid_scalar() {
    let group = ScalarGroup::new();
    let point = group.generator();
    
    // This should panic
    point.scale(BigUint::from(0u32));
} 