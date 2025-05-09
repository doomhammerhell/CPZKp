use pyo3::prelude::*;
use cpzkp::{Group, Point, get_constants, solve_zk_challenge_s, Error};
use num_bigint::BigUint;
use serde_json::{json, to_string};

#[pyclass]
pub struct KeyPair {
    group: Group,
    p: BigUint,
    q: BigUint,
    g: Point,
    h: Point,
    y1: Point,
    y2: Point,
}

#[pymethods]
impl KeyPair {
    #[new]
    pub fn new(group: &str) -> PyResult<Self> {
        let group = match group {
            "scalar" => Group::Scalar,
            "elliptic" => Group::EllipticCurve,
            #[cfg(feature = "curve25519")]
            "curve25519" => Group::Curve25519,
            _ => return Err(pyo3::exceptions::PyValueError::new_err("Invalid group type")),
        };

        let (p, q, g, h) = get_constants(&group)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        
        let x_secret = BigUint::from_bytes_be(&rand::random::<[u8; 32]>());
        let y1 = g.scale(x_secret.clone());
        let y2 = h.scale(x_secret);

        Ok(KeyPair {
            group,
            p,
            q,
            g,
            h,
            y1,
            y2,
        })
    }

    pub fn to_json(&self) -> PyResult<String> {
        let json = json!({
            "group": self.group,
            "p": self.p.to_string(),
            "q": self.q.to_string(),
            "g": self.g.serialize(),
            "h": self.h.serialize(),
            "y1": self.y1.serialize(),
            "y2": self.y2.serialize(),
        });
        to_string(&json).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
pub struct Proof {
    group: Group,
    r1: Point,
    r2: Point,
    y1: Point,
    y2: Point,
    g: Point,
    h: Point,
    c: BigUint,
    s: BigUint,
    p: BigUint,
}

#[pymethods]
impl Proof {
    #[staticmethod]
    pub fn generate(keypair: &KeyPair, message: &str) -> PyResult<Self> {
        let k = BigUint::from_bytes_be(&rand::random::<[u8; 32]>());
        let r1 = keypair.g.scale(k.clone());
        let r2 = keypair.h.scale(k.clone());

        let c = BigUint::from_bytes_be(&rand::random::<[u8; 32]>());
        let s = solve_zk_challenge_s(
            &BigUint::from_bytes_be(message.as_bytes()),
            &k,
            &c,
            &keypair.q,
        );

        Ok(Proof {
            group: keypair.group.clone(),
            r1,
            r2,
            y1: keypair.y1.clone(),
            y2: keypair.y2.clone(),
            g: keypair.g.clone(),
            h: keypair.h.clone(),
            c,
            s,
            p: keypair.p.clone(),
        })
    }

    pub fn to_json(&self) -> PyResult<String> {
        let json = json!({
            "group": self.group,
            "r1": self.r1.serialize(),
            "r2": self.r2.serialize(),
            "y1": self.y1.serialize(),
            "y2": self.y2.serialize(),
            "g": self.g.serialize(),
            "h": self.h.serialize(),
            "c": self.c.to_string(),
            "s": self.s.to_string(),
            "p": self.p.to_string(),
        });
        to_string(&json).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    pub fn verify(&self) -> PyResult<bool> {
        let params = cpzkp::VerificationParams {
            r1: self.r1.clone(),
            r2: self.r2.clone(),
            y1: self.y1.clone(),
            y2: self.y2.clone(),
            g: self.g.clone(),
            h: self.h.clone(),
            c: self.c.clone(),
            s: self.s.clone(),
            p: self.p.clone(),
        };
        cpzkp::verify(&params).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }
}

#[pymodule]
fn cpzkp_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<KeyPair>()?;
    m.add_class::<Proof>()?;
    Ok(())
} 