use crate::types::{Error, Group, Point, VerificationParams};
use crate::traits::{GroupOps, PointOps, ZkpOps};
use num::traits::One;
use num_bigint::BigUint;
use rand::RngCore;

/// Implementation of scalar group operations
pub struct ScalarGroup;

impl GroupOps for ScalarGroup {
    fn prime(&self) -> BigUint {
        BigUint::from(10009u32)
    }

    fn order(&self) -> BigUint {
        BigUint::from(5004u32)
    }

    fn generator(&self) -> Point {
        Point::Scalar(BigUint::from(3u32))
    }

    fn second_generator(&self) -> Point {
        Point::Scalar(BigUint::from(2892u32))
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
            Point::Scalar(_) => true,
            Point::ECPoint(_, _) => false,
        }
    }

    fn double(&self) -> Point {
        match self {
            Point::Scalar(x) => Point::Scalar(x.clone()),
            Point::ECPoint(_, _) => panic!("Cannot double ECPoint in scalar group"),
        }
    }

    fn scale(&self, scalar: BigUint) -> Point {
        match self {
            Point::Scalar(x) => Point::Scalar(x.modpow(&scalar, &BigUint::from(10009u32))),
            Point::ECPoint(_, _) => panic!("Cannot scale ECPoint in scalar group"),
        }
    }
}

impl ZkpOps for ScalarGroup {
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
            Point::Scalar(r1),
            Point::Scalar(r2),
            Point::Scalar(y1),
            Point::Scalar(y2),
            Point::Scalar(g),
            Point::Scalar(h),
        ) = (
            &params.r1,
            &params.r2,
            &params.y1,
            &params.y2,
            &params.g,
            &params.h,
        ) {
            let condition_1 = *r1
                == (g.modpow(&params.s, &params.p) * y1.modpow(&params.c, &params.p))
                    .modpow(&BigUint::one(), &params.p);
            let condition_2 = *r2
                == (h.modpow(&params.s, &params.p) * y2.modpow(&params.c, &params.p))
                    .modpow(&BigUint::one(), &params.p);
            Ok(condition_1 && condition_2)
        } else {
            Err(Error::PointTypeMismatch)
        }
    }
} 