use crate::types::{Error, Point};
use num_bigint::BigUint;

/// Trait for group operations
pub trait GroupOps {
    /// Get the prime modulus of the group
    fn prime(&self) -> BigUint;
    
    /// Get the order of the group
    fn order(&self) -> BigUint;
    
    /// Get the generator point of the group
    fn generator(&self) -> Point;
    
    /// Get the second generator point of the group
    fn second_generator(&self) -> Point;
}

/// Trait for curve operations
pub trait CurveGroup: GroupOps {
    /// Get the curve name
    fn curve_name(&self) -> &'static str;
    
    /// Get the curve parameters
    fn curve_params(&self) -> (BigUint, BigUint, BigUint); // (a, b, p)
    
    /// Check if a point is on the curve
    fn is_on_curve(&self, point: &Point) -> bool;
    
    /// Add two points on the curve
    fn add_points(&self, p1: &Point, p2: &Point) -> Result<Point, Error>;
    
    /// Multiply a point by a scalar
    fn scalar_mul(&self, point: &Point, scalar: &BigUint) -> Result<Point, Error>;
}

/// Trait for zero-knowledge proof operations
pub trait ZkpOps {
    /// Generate a random challenge
    fn generate_challenge(&self) -> Result<BigUint, Error>;
    
    /// Solve the zero-knowledge challenge
    fn solve_challenge(
        &self,
        secret: &BigUint,
        random: &BigUint,
        challenge: &BigUint,
    ) -> BigUint;
    
    /// Verify a zero-knowledge proof
    fn verify_proof(
        &self,
        params: &crate::types::VerificationParams,
    ) -> Result<bool, Error>;
}

/// Trait for point operations
pub trait PointOps {
    /// Serialize a point to bytes
    fn serialize(&self) -> Vec<u8>;
    
    /// Deserialize a point from bytes
    fn deserialize(bytes: Vec<u8>, group: &crate::types::Group) -> Result<Point, Error>;
    
    /// Check if a point is on the curve
    fn is_on_curve(&self) -> bool;
    
    /// Double a point
    fn double(&self) -> Point;
    
    /// Scale a point by a scalar
    fn scale(&self, scalar: BigUint) -> Point;
} 