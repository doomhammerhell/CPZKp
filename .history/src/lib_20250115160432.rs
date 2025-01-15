//! Zero-Knowledge Proof Authentication Library
//!
//! This library implements a zero-knowledge proof authentication system that supports
//! both scalar (multiplicative group) and elliptic curve groups. It provides functionality
//! for both the prover and verifier in the ZKP protocol.
//!
//! # Features
//! - Support for scalar (multiplicative) group operations
//! - Support for secp256k1 elliptic curve operations
//! - Serialization/deserialization of group elements
//! - Command-line argument parsing for group selection
//!
//! # Example
//! ```rust
//! use zkp_auth::{Group, Point, get_constants, solve_zk_challenge_s};
//! use num_bigint::BigUint;
//!
//! // Select the group type
//! let group = Group::Scalar;
//!
//! // Get the system parameters
//! let (p, q, g, h) = get_constants(&group);
//!
//! // Generate a random secret
//! let x_secret = BigUint::from(1234u32);
//! let k = BigUint::from(5678u32);
//! let c = BigUint::from(910u32);
//!
//! // Solve the ZK challenge
//! let s = solve_zk_challenge_s(&x_secret, &k, &c, &q);
//! ```

mod secp256k1;

use num::traits::One;
use num_bigint::BigUint;
use rand::{distributions::Alphanumeric, Rng};
use secp256k1::Secp256k1Point;

/// The possible kind of errors returned by this library.
#[derive(Debug)]
pub enum Error {
    InvalidArguments,
    PointTypeMismatch,
    InvalidSerialization(String),
    EllipticCurveError(String),
    InvalidGroupType,
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

/// Detects if any argument is --scalar or --elliptic and returns the
/// corresponding cyclic group to use.
///
/// * `args` - Vector of command line arguments.
pub fn parse_group_from_command_line(args: Vec<String>) -> Group {
    match args.len() {
        2 => match args[1].trim() {
            "--elliptic" => Group::EllipticCurve,
            "--scalar" | "" => Group::Scalar,
            _ => panic!("Invalid argument [--scalar(default)|--elliptic] available."),
        },
        _ => Group::Scalar,
    }
}

/// Returns the default constants to use in both server and clients. Note that
/// they should be the same for both for the ZK algorithm to work.
///
/// * `group` - The cyclic group to use.
pub fn get_constants(group: &Group) -> Result<(BigUint, BigUint, Point, Point), Error> {
    match group {
        Group::Scalar => Ok(get_constants_scalar()),
        Group::EllipticCurve => get_constants_elliptic_curve(),
    }
}

pub fn get_constants_scalar() -> (BigUint, BigUint, Point, Point) {
    (
        BigUint::from(10009u32),
        BigUint::from(5004u32),
        Point::Scalar(BigUint::from(3u32)),
        Point::Scalar(BigUint::from(2892u32)),
    )
}

pub fn get_constants_elliptic_curve() -> Result<(BigUint, BigUint, Point, Point), Error> {
    let g = Secp256k1Point::generator();
    let h = g.clone().scale(BigUint::from(13u32));
    Ok((
        Secp256k1Point::prime(),
        Secp256k1Point::n(),
        Point::from_secp256k1(&g)?,
        Point::from_secp256k1(&h)?,
    ))
}

impl Point {
    /// Serializes the Point structure to an array of bytes for network transfer.
    pub fn serialize(&self) -> Vec<u8> {
        match self {
            Point::Scalar(x) => x.to_bytes_be(),
            Point::ECPoint(x, y) => {
                let mut x = x.to_bytes_be();
                let mut y = y.to_bytes_be();
                let diff = (x.len() as i32) - (y.len() as i32);
                if diff > 0 {
                    y.resize(y.len() + diff as usize, 0);
                    y.rotate_right(diff as usize);
                } else {
                    x.resize(x.len() + (-diff as usize), 0);
                    x.rotate_right((-diff) as usize);
                }
                x.append(&mut y);
                x
            }
        }
    }

    /// Deserializes a Point structure from an array of bytes.
    pub fn deserialize(v: Vec<u8>, group: &Group) -> Result<Point, Error> {
        match group {
            Group::Scalar => Ok(Point::deserialize_into_scalar(v)),
            Group::EllipticCurve => Point::deserialize_into_ecpoint(v),
        }
    }

    pub fn deserialize_into_scalar(v: Vec<u8>) -> Point {
        Point::Scalar(BigUint::from_bytes_be(&v))
    }

    pub fn deserialize_into_ecpoint(v: Vec<u8>) -> Result<Point, Error> {
        let len = v.len();

        if len % 2 != 0 {
            return Err(Error::InvalidSerialization(
                "The length of the serialized object must be even".to_string(),
            ));
        }

        Ok(Point::ECPoint(
            BigUint::from_bytes_be(&v[..len / 2]),
            BigUint::from_bytes_be(&v[len / 2..]),
        ))
    }

    /// Converts a point from the `secp256k1` library into a Point
    pub fn from_secp256k1(point: &Secp256k1Point) -> Result<Point, Error> {
        match point {
            Secp256k1Point::Coor { x, y, .. } => {
                Ok(Point::ECPoint(x.number.clone(), y.number.clone()))
            }
            _ => Err(Error::EllipticCurveError(
                "Cannot convert Zero point".to_string(),
            )),
        }
    }
}

/// Exponenciates two points g & h:
///  - For the integer or scalar group the new ones are: g^exp & h^exp
///  - For the elliptic curve group the new ones are: exp * g & exp * h
pub fn exponentiates_points(
    exp: &BigUint,
    g: &Point,
    h: &Point,
    p: &BigUint,
) -> Result<(Point, Point), Error> {
    match (g, h) {
        (Point::Scalar(g), Point::Scalar(h)) => Ok(exponentiates_points_scalar(exp, g, h, p)),
        (Point::ECPoint(gx, gy), Point::ECPoint(hx, hy)) => {
            exponentiates_points_elliptic_curve(exp, gx, gy, hx, hy)
        }
        _ => Err(Error::PointTypeMismatch),
    }
}

pub fn exponentiates_points_scalar(
    exp: &BigUint,
    g: &BigUint,
    h: &BigUint,
    p: &BigUint,
) -> (Point, Point) {
    (
        Point::Scalar(g.modpow(exp, p)),
        Point::Scalar(h.modpow(exp, p)),
    )
}

pub fn exponentiates_points_elliptic_curve(
    exp: &BigUint,
    gx: &BigUint,
    gy: &BigUint,
    hx: &BigUint,
    hy: &BigUint,
) -> Result<(Point, Point), Error> {
    let g = Secp256k1Point::from_bigint(&gx, &gy);
    let h = Secp256k1Point::from_bigint(&hx, &hy);

    let g = g.scale(exp.clone());
    let h = h.scale(exp.clone());

    let g_new = match g {
        secp256k1::Point::Coor { x, y, .. } => Point::ECPoint(x.number, y.number),
        _ => {
            return Err(Error::EllipticCurveError(
                "Zero point reached in multiplication".to_string(),
            ))
        }
    };

    let h_new = match h {
        secp256k1::Point::Coor { x, y, .. } => Point::ECPoint(x.number, y.number),
        _ => {
            return Err(Error::EllipticCurveError(
                "Zero point reached in multiplication".to_string(),
            ))
        }
    };

    Ok((g_new, h_new))
}

/// This function solves the ZK challenge `s` proposed by the verifier.
///
/// s = (k - c * x) mod q
///
/// * `x_secret` - secret password.
/// * `k` - random number selected by the prover.
/// * `c` - random number selected by the verifier.
/// * `q` - the order of the cyclic group
pub fn solve_zk_challenge_s(x_secret: &BigUint, k: &BigUint, c: &BigUint, q: &BigUint) -> BigUint {
    let cx = c * x_secret;
    if *k > cx {
        (k - cx).modpow(&BigUint::one(), q)
    } else {
        q - (cx - k).modpow(&BigUint::one(), q)
    }
}

pub fn verify(
    r1: &Point,
    r2: &Point,
    y1: &Point,
    y2: &Point,
    g: &Point,
    h: &Point,
    c: &BigUint,
    s: &BigUint,
    p: &BigUint,
) -> Result<bool, Error> {
    match (r1, r2, y1, y2, g, h) {
        (
            Point::Scalar(r1),
            Point::Scalar(r2),
            Point::Scalar(y1),
            Point::Scalar(y2),
            Point::Scalar(g),
            Point::Scalar(h),
        ) => Ok(verify_scalar(r1, r2, y1, y2, g, h, c, s, p)),
        (
            Point::ECPoint(r1x, r1y),
            Point::ECPoint(r2x, r2y),
            Point::ECPoint(y1x, y1y),
            Point::ECPoint(y2x, y2y),
            Point::ECPoint(gx, gy),
            Point::ECPoint(hx, hy),
        ) => Ok(verify_ecpoint(
            r1x, r1y, r2x, r2y, y1x, y1y, y2x, y2y, gx, gy, hx, hy, c, s,
        )),
        _ => Err(Error::InvalidArguments),
    }
}

/// This function verifies that the challenge `s` was properly solved by the
/// prover for integer cyclic groups.
/// r1 = g^s * y1^c && r2 = h^s * y2^c
///
/// * `r1` - g^k generated by the prover.
/// * `r2` - h^k generated by the prover.
/// * `y1` - g^x generated by the prover.
/// * `y2` - h^x generated by the prover.
/// * `g` - predefined element of the cyclic group.
/// * `h` - predefined element of the cyclic group (g^n mod q ?).
/// * `c` - random number generated by the verifier.
/// * `s` - solution to the challenge computed by the prover.
/// * `p` - the prime number used to defined the cyclic group.
pub fn verify_scalar(
    r1: &BigUint,
    r2: &BigUint,
    y1: &BigUint,
    y2: &BigUint,
    g: &BigUint,
    h: &BigUint,
    c: &BigUint,
    s: &BigUint,
    p: &BigUint,
) -> bool {
    let condition_1 = *r1 == (g.modpow(s, p) * y1.modpow(c, p)).modpow(&BigUint::one(), p);
    let condition_2 = *r2 == (h.modpow(s, p) * y2.modpow(c, p)).modpow(&BigUint::one(), p);
    condition_1 && condition_2
}

/// This function verifies that the challenge `s` was properly solved by the
/// prover for elliptic curves cyclic group (secp256k1).
/// r1 = s * g + c * y1 && r2 = s * h + c * y2
///
/// * `r1x` - x coordinate of k * g generated by the prover.
/// * `r1y` - y coordinate of k * g generated by the prover.
/// * `r2x` - x coordinate of k * h generated by the prover.
/// * `r2y` - y coordinate of k * h generated by the prover.
/// * `y1x` - x coordinate of x * g generated by the prover.
/// * `y1y` - y coordinate of x * g generated by the prover.
/// * `y2x` - x coordinate of x * h generated by the prover.
/// * `y2y` - x coordinate of x * h generated by the prover.
/// * `gx` - x coordinate of predefined point of the cyclic group.
/// * `gy` - y coordinate of predefined point of the cyclic group.
/// * `hx` - x coordinate of predefined point of the cyclic group.
/// * `hy` - y coordinate of predefined point of the cyclic group.
/// * `c` - random number generated by the verifier.
/// * `s` - solution to the challenge computed by the prover.
/// * `p` - the prime number used to defined the cyclic group.
pub fn verify_ecpoint(
    r1x: &BigUint,
    r1y: &BigUint,
    r2x: &BigUint,
    r2y: &BigUint,
    y1x: &BigUint,
    y1y: &BigUint,
    y2x: &BigUint,
    y2y: &BigUint,
    gx: &BigUint,
    gy: &BigUint,
    hx: &BigUint,
    hy: &BigUint,
    c: &BigUint,
    s: &BigUint,
) -> bool {
    let g = Secp256k1Point::from_bigint(&gx, &gy);
    let h = Secp256k1Point::from_bigint(&hx, &hy);
    let y1 = Secp256k1Point::from_bigint(&y1x, &y1y);
    let y2 = Secp256k1Point::from_bigint(&y2x, &y2y);
    let r1 = Secp256k1Point::from_bigint(&r1x, &r1y);
    let r2 = Secp256k1Point::from_bigint(&r2x, &r2y);

    let sg = g.scale(s.clone());
    let sh = h.scale(s.clone());
    let cy1 = y1.scale(c.clone());
    let cy2 = y2.scale(c.clone());

    (r1 == sg + cy1) && (r2 == sh + cy2)
}

/// Generates a cryptographically secure random array of bytes.
///
/// This function uses the system's cryptographically secure random number generator.
pub fn get_random_array<const BYTES: usize>() -> Result<[u8; BYTES], Error> {
    use rand::RngCore;
    let mut arr = [0u8; BYTES];
    rand::thread_rng()
        .try_fill_bytes(&mut arr)
        .map_err(|e| Error::RandomGenerationError(e.to_string()))?;
    Ok(arr)
}

/// Generates a cryptographically secure 32-byte random number.
pub fn get_random_number() -> Result<BigUint, Error> {
    Ok(BigUint::from_bytes_be(&get_random_array::<32>()?))
}

/// Generates a cryptographically secure random string of specified length.
/// Useful for generating user or session IDs.
pub fn get_random_string(n: usize) -> Result<String, Error> {
    use rand::Rng;
    let rng = rand::thread_rng();
    Ok(rng
        .sample_iter(&Alphanumeric)
        .take(n)
        .map(char::from)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_get_random_array() {
        let a = get_random_array::<32>().unwrap();
        let b = get_random_array::<32>().unwrap();
        let c = get_random_array::<32>().unwrap();
        let d = get_random_array::<32>().unwrap();
        assert_ne!(a, b);
        assert_ne!(b, c);
        assert_ne!(c, d);
    }

    #[test]
    fn test_get_random_number() {
        let a = get_random_number().unwrap();
        let b = get_random_number().unwrap();
        let c = get_random_number().unwrap();
        let d = get_random_number().unwrap();
        assert_ne!(a, b);
        assert_ne!(b, c);
        assert_ne!(c, d);
    }

    #[test]
    fn test_get_random_string() {
        let a = get_random_string(10).unwrap();
        let b = get_random_string(10).unwrap();
        assert_ne!(a, b);
        assert_eq!(a.len(), 10);
        assert_eq!(b.len(), 10);
    }

    #[test]
    fn test_random_string_length() {
        for len in [0, 1, 10, 100] {
            let s = get_random_string(len).unwrap();
            assert_eq!(s.len(), len);
        }
    }

    proptest! {
        #[test]
        fn test_serialization_roundtrip_scalar(x in 1u32..1000u32) {
            let point = Point::Scalar(BigUint::from(x));
            let serialized = point.serialize();
            let deserialized = Point::deserialize(serialized, &Group::Scalar).unwrap();
            prop_assert_eq!(point, deserialized);
        }

        #[test]
        fn test_zk_challenge_solution_range(
            x in 1u32..1000u32,
            k in 1u32..1000u32,
            c in 1u32..1000u32,
            q in 1001u32..2000u32
        ) {
            let x = BigUint::from(x);
            let k = BigUint::from(k);
            let c = BigUint::from(c);
            let q = BigUint::from(q);

            let s = solve_zk_challenge_s(&x, &k, &c, &q);
            prop_assert!(s < q);
        }

        #[test]
        fn test_scalar_exponentiation_properties(
            exp in 1u32..1000u32,
            base in 1u32..1000u32,
            p in 1001u32..2000u32
        ) {
            let exp = BigUint::from(exp);
            let base = BigUint::from(base);
            let p = BigUint::from(p);

            let g = Point::Scalar(base.clone());
            let h = Point::Scalar(base.clone());

            let (g_exp, h_exp) = exponentiates_points(&exp, &g, &h, &p).unwrap();

            if let (Point::Scalar(g_val), Point::Scalar(h_val)) = (g_exp, h_exp) {
                prop_assert_eq!(g_val, h_val);
                prop_assert!(g_val < p);
            }
        }
    }
}
