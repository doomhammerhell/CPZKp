//! Property tests for the scalar module
//! 
//! This module contains property-based tests using proptest to verify
//! mathematical properties and invariants of scalar operations.

use cpzkp::scalar::*;
use cpzkp::types::*;
use num_bigint::BigUint;
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_scalar_commutativity(a in any::<u32>(), b in any::<u32>()) {
        let group = ScalarGroup::new();
        let point = group.generator();
        
        let a_big = BigUint::from(a);
        let b_big = BigUint::from(b);
        
        let ab = point.scale(a_big.clone()).scale(b_big.clone());
        let ba = point.scale(b_big).scale(a_big);
        
        assert_eq!(ab, ba);
    }
    
    #[test]
    fn test_scalar_distributivity(a in any::<u32>(), b in any::<u32>()) {
        let group = ScalarGroup::new();
        let point = group.generator();
        
        let a_big = BigUint::from(a);
        let b_big = BigUint::from(b);
        let sum = a_big.clone() + b_big.clone();
        
        let left = point.scale(sum);
        let right = point.scale(a_big).add_points(&point.scale(b_big)).unwrap();
        
        assert_eq!(left, right);
    }
    
    #[test]
    fn test_zkp_soundness(secret in any::<u32>(), random in any::<u32>()) {
        let group = ScalarGroup::new();
        let challenge = group.generate_challenge().unwrap();
        
        let secret_big = BigUint::from(secret);
        let random_big = BigUint::from(random);
        
        let s = group.solve_challenge(&secret_big, &random_big, &challenge);
        
        let params = VerificationParams {
            r1: group.generator().scale(random_big.clone()),
            r2: group.second_generator().scale(random_big),
            y1: group.generator().scale(secret_big.clone()),
            y2: group.second_generator().scale(secret_big),
            g: group.generator(),
            h: group.second_generator(),
            c: challenge,
            s: s,
        };
        
        assert!(group.verify_proof(&params).unwrap());
    }
    
    #[test]
    fn test_point_serialization(point_x in any::<u32>(), point_y in any::<u32>()) {
        let group = ScalarGroup::new();
        let point = Point {
            x: BigUint::from(point_x),
            y: BigUint::from(point_y),
        };
        
        let serialized = point.serialize();
        let deserialized = Point::deserialize(serialized, &Group::Scalar).unwrap();
        
        assert_eq!(point, deserialized);
    }
} 