use crate::types::{Error, Group, Point, VerificationParams};
use crate::traits::{GroupOps, PointOps, ZkpOps};
use num::traits::One;
use num_bigint::BigUint;
use rand::RngCore;

/// Implementation of elliptic curve operations
pub struct EllipticCurveGroup;

impl GroupOps for EllipticCurveGroup {
    fn prime(&self) -> BigUint {
        BigUint::from(10009u32)
    }

    fn order(&self) -> BigUint {
        BigUint::from(5004u32)
    }

    fn generator(&self) -> Point {
        Point::ECPoint(BigUint::from(3u32), BigUint::from(2892u32))
    }

    fn second_generator(&self) -> Point {
        Point::ECPoint(BigUint::from(2892u32), BigUint::from(3u32))
    }
}

impl PointOps for Point {
    fn serialize(&self) -> Vec<u8> {
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

    fn deserialize(bytes: Vec<u8>, group: &Group) -> Result<Point, Error> {
        match group {
            Group::Scalar => Ok(Point::Scalar(BigUint::from_bytes_be(&bytes))),
            Group::EllipticCurve => {
                let len = bytes.len();
                if len % 2 != 0 {
                    return Err(Error::InvalidSerialization(
                        "The length of the serialized object must be even".to_string(),
                    ));
                }
                Ok(Point::ECPoint(
                    BigUint::from_bytes_be(&bytes[..len / 2]),
                    BigUint::from_bytes_be(&bytes[len / 2..]),
                ))
            }
        }
    }

    fn is_on_curve(&self) -> bool {
        match self {
            Point::Scalar(_) => false,
            Point::ECPoint(x, y) => {
                let p = BigUint::from(10009u32);
                let a = BigUint::from(0u32);
                let b = BigUint::from(7u32);
                let lhs = y.modpow(&BigUint::from(2u32), &p);
                let rhs = (x.modpow(&BigUint::from(3u32), &p) + a * x + b) % p;
                lhs == rhs
            }
        }
    }

    fn double(&self) -> Point {
        match self {
            Point::Scalar(_) => panic!("Cannot double Scalar in elliptic curve group"),
            Point::ECPoint(x, y) => {
                let p = BigUint::from(10009u32);
                let a = BigUint::from(0u32);
                let lambda = ((BigUint::from(3u32) * x.modpow(&BigUint::from(2u32), &p) + a)
                    * (BigUint::from(2u32) * y).modpow(&(p - BigUint::from(2u32)), &p))
                    % p;
                let x3 = (lambda.modpow(&BigUint::from(2u32), &p) - BigUint::from(2u32) * x) % p;
                let y3 = (lambda * (x - &x3) - y) % p;
                Point::ECPoint(x3, y3)
            }
        }
    }

    fn scale(&self, scalar: BigUint) -> Point {
        match self {
            Point::Scalar(_) => panic!("Cannot scale Scalar in elliptic curve group"),
            Point::ECPoint(x, y) => {
                let mut result = Point::ECPoint(x.clone(), y.clone());
                let mut scalar = scalar;
                let mut current = self.clone();
                while scalar > BigUint::from(0u32) {
                    if &scalar % BigUint::from(2u32) == BigUint::from(1u32) {
                        result = result.add(&current);
                    }
                    current = current.double();
                    scalar = scalar / BigUint::from(2u32);
                }
                result
            }
        }
    }
}

impl ZkpOps for EllipticCurveGroup {
    fn generate_challenge(&self) -> Result<BigUint, Error> {
        let mut arr = [0u8; 32];
        rand::thread_rng()
            .try_fill_bytes(&mut arr)
            .map_err(|e| Error::RandomGenerationError(e.to_string()))?;
        Ok(BigUint::from_bytes_be(&arr))
    }

    fn solve_challenge(
        &self,
        secret: &BigUint,
        random: &BigUint,
        challenge: &BigUint,
    ) -> BigUint {
        let cx = challenge * secret;
        let result = if *random >= cx {
            (random - cx).modpow(&BigUint::one(), &self.order())
        } else {
            self.order() - (cx - random).modpow(&BigUint::one(), &self.order())
        };
        result % self.order()
    }

    fn verify_proof(&self, params: &VerificationParams) -> Result<bool, Error> {
        if let (
            Point::ECPoint(r1x, r1y),
            Point::ECPoint(r2x, r2y),
            Point::ECPoint(y1x, y1y),
            Point::ECPoint(y2x, y2y),
            Point::ECPoint(gx, gy),
            Point::ECPoint(hx, hy),
        ) = (
            &params.r1,
            &params.r2,
            &params.y1,
            &params.y2,
            &params.g,
            &params.h,
        ) {
            let g_s = Point::ECPoint(gx.clone(), gy.clone()).scale(params.s.clone());
            let y1_c = Point::ECPoint(y1x.clone(), y1y.clone()).scale(params.c.clone());
            let h_s = Point::ECPoint(hx.clone(), hy.clone()).scale(params.s.clone());
            let y2_c = Point::ECPoint(y2x.clone(), y2y.clone()).scale(params.c.clone());
            let condition_1 = Point::ECPoint(r1x.clone(), r1y.clone()) == g_s.add(&y1_c);
            let condition_2 = Point::ECPoint(r2x.clone(), r2y.clone()) == h_s.add(&y2_c);
            Ok(condition_1 && condition_2)
        } else {
            Err(Error::PointTypeMismatch)
        }
    }
} 