use num_bigint::BigUint;
use std::fmt;

/// The possible kind of errors returned by this library.
#[derive(Debug)]
pub enum Error {
    /// Invalid arguments provided to a function
    InvalidArguments,
    /// Mismatched point types in operation
    PointTypeMismatch,
    /// Error during serialization/deserialization
    InvalidSerialization(String),
    /// Error in elliptic curve operations
    EllipticCurveError(String),
    /// Invalid group type specified
    InvalidGroupType,
    /// Error during random number generation
    RandomGenerationError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidArguments => write!(f, "Invalid arguments provided"),
            Error::PointTypeMismatch => write!(f, "Mismatched point types in operation"),
            Error::InvalidSerialization(msg) => write!(f, "Serialization error: {}", msg),
            Error::EllipticCurveError(msg) => write!(f, "Elliptic curve error: {}", msg),
            Error::InvalidGroupType => write!(f, "Invalid group type specified"),
            Error::RandomGenerationError(msg) => write!(f, "Random generation error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

/// An enum used to select the cyclic group for the ZKP protocol.
///
/// The protocol can operate in either:
/// - A scalar (multiplicative) group of integers modulo a prime
/// - The secp256k1 elliptic curve group
#[derive(Debug, Default)]
pub enum Group {
    #[default]
    Scalar,
    EllipticCurve,
}

/// Structure to represent elements in the cyclic group.
///
/// Points can be either:
/// - Scalar: A single value representing an element in the multiplicative group
/// - ECPoint: An (x,y) coordinate pair representing a point on the elliptic curve
#[derive(Debug, Clone, PartialEq)]
pub enum Point {
    Scalar(BigUint),
    ECPoint(BigUint, BigUint),
}

/// Parameters for verification of a zero-knowledge proof
#[derive(Clone)]
pub struct VerificationParams {
    /// First commitment point
    pub r1: Point,
    /// Second commitment point
    pub r2: Point,
    /// First public key point
    pub y1: Point,
    /// Second public key point
    pub y2: Point,
    /// First generator point
    pub g: Point,
    /// Second generator point
    pub h: Point,
    /// Challenge value
    pub c: BigUint,
    /// Response value
    pub s: BigUint,
    /// Prime modulus
    pub p: BigUint,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Point::Scalar(x) => write!(f, "Point::Scalar({})", x),
            Point::ECPoint(x, y) => write!(f, "Point::ECPoint({}, {})", x, y),
        }
    }
} 