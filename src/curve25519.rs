use crate::types::{Error, Group, Point};
use crate::traits::{CurveGroup, GroupOps};
use curve25519_dalek::{
    constants::ED25519_BASEPOINT_POINT,
    edwards::{CompressedEdwardsY, EdwardsPoint},
    scalar::Scalar,
};
use num_bigint::BigUint;

/// Implementation of Curve25519 operations
pub struct Curve25519Group;

impl GroupOps for Curve25519Group {
    fn prime(&self) -> BigUint {
        // 2^255 - 19
        BigUint::from(2u32).pow(255) - BigUint::from(19u32)
    }

    fn order(&self) -> BigUint {
        // 2^252 + 27742317777372353535851937790883648493
        BigUint::from(2u32).pow(252) + BigUint::from_bytes_be(&[
            0x14, 0xde, 0xf9, 0xde, 0xa2, 0xf7, 0x9c, 0xd6, 0x58, 0x12, 0x63, 0x1a, 0x5c, 0xf5,
            0xd3, 0xed,
        ])
    }

    fn generator(&self) -> Point {
        let point = ED25519_BASEPOINT_POINT;
        Point::ECPoint(
            BigUint::from_bytes_be(&point.compress().to_bytes()),
            BigUint::from(0u32), // Curve25519 uses compressed points
        )
    }

    fn second_generator(&self) -> Point {
        // Using a fixed second generator point
        let point = ED25519_BASEPOINT_POINT * Scalar::from(2u32);
        Point::ECPoint(
            BigUint::from_bytes_be(&point.compress().to_bytes()),
            BigUint::from(0u32),
        )
    }
}

impl CurveGroup for Curve25519Group {
    fn curve_name(&self) -> &'static str {
        "Curve25519"
    }

    fn curve_params(&self) -> (BigUint, BigUint, BigUint) {
        // Curve25519: y² = x³ + 486662x² + x
        let a = BigUint::from(486662u32);
        let b = BigUint::from(1u32);
        let p = self.prime();
        (a, b, p)
    }

    fn is_on_curve(&self, point: &Point) -> bool {
        match point {
            Point::Scalar(_) => false,
            Point::ECPoint(x, _) => {
                // For Curve25519, we only need to check if the x-coordinate is valid
                let x_bytes = x.to_bytes_be();
                if x_bytes.len() != 32 {
                    return false;
                }
                let mut bytes = [0u8; 32];
                bytes.copy_from_slice(&x_bytes);
                CompressedEdwardsY::from_slice(&bytes).decompress().is_some()
            }
        }
    }

    fn add_points(&self, p1: &Point, p2: &Point) -> Result<Point, Error> {
        match (p1, p2) {
            (Point::ECPoint(x1, _), Point::ECPoint(x2, _)) => {
                let x1_bytes = x1.to_bytes_be();
                let x2_bytes = x2.to_bytes_be();
                if x1_bytes.len() != 32 || x2_bytes.len() != 32 {
                    return Err(Error::InvalidArguments);
                }

                let mut bytes1 = [0u8; 32];
                let mut bytes2 = [0u8; 32];
                bytes1.copy_from_slice(&x1_bytes);
                bytes2.copy_from_slice(&x2_bytes);

                let point1 = CompressedEdwardsY::from_slice(&bytes1)
                    .decompress()
                    .ok_or(Error::InvalidArguments)?;
                let point2 = CompressedEdwardsY::from_slice(&bytes2)
                    .decompress()
                    .ok_or(Error::InvalidArguments)?;

                let result = point1 + point2;
                Ok(Point::ECPoint(
                    BigUint::from_bytes_be(&result.compress().to_bytes()),
                    BigUint::from(0u32),
                ))
            }
            _ => Err(Error::PointTypeMismatch),
        }
    }

    fn scalar_mul(&self, point: &Point, scalar: &BigUint) -> Result<Point, Error> {
        match point {
            Point::ECPoint(x, _) => {
                let x_bytes = x.to_bytes_be();
                if x_bytes.len() != 32 {
                    return Err(Error::InvalidArguments);
                }

                let mut bytes = [0u8; 32];
                bytes.copy_from_slice(&x_bytes);

                let edwards_point = CompressedEdwardsY::from_slice(&bytes)
                    .decompress()
                    .ok_or(Error::InvalidArguments)?;

                let scalar_bytes = scalar.to_bytes_be();
                if scalar_bytes.len() > 32 {
                    return Err(Error::InvalidArguments);
                }

                let mut scalar_bytes_padded = [0u8; 32];
                scalar_bytes_padded[32 - scalar_bytes.len()..].copy_from_slice(&scalar_bytes);
                let scalar = Scalar::from_bytes_mod_order(scalar_bytes_padded);

                let result = edwards_point * scalar;
                Ok(Point::ECPoint(
                    BigUint::from_bytes_be(&result.compress().to_bytes()),
                    BigUint::from(0u32),
                ))
            }
            _ => Err(Error::PointTypeMismatch),
        }
    }
} 