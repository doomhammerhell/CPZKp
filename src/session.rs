use crate::types::{Error, Group, Point, VerificationParams};
use crate::traits::{GroupOps, ZkpOps};
use num_bigint::BigUint;
use serde_json::{json, to_string};
use std::collections::HashMap;

/// State of a multi-round session
#[derive(Debug, Clone)]
pub enum SessionState {
    /// Initial state, waiting for first round
    Initial,
    /// Active state, can accept new rounds
    Active,
    /// Finalized state, no more rounds allowed
    Finalized,
}

/// A multi-round session for zero-knowledge proofs
pub struct Session {
    state: SessionState,
    group: Group,
    p: BigUint,
    q: BigUint,
    g: Point,
    h: Point,
    y1: Point,
    y2: Point,
    rounds: HashMap<usize, (Point, Point, BigUint, BigUint)>,
    current_round: usize,
}

impl Session {
    /// Create a new session
    pub fn new(group: Group) -> Result<Self, Error> {
        let (p, q, g, h) = get_constants(&group)?;
        let x_secret = BigUint::from_bytes_be(&rand::random::<[u8; 32]>());
        let y1 = g.scale(x_secret.clone());
        let y2 = h.scale(x_secret);

        Ok(Session {
            state: SessionState::Initial,
            group,
            p,
            q,
            g,
            h,
            y1,
            y2,
            rounds: HashMap::new(),
            current_round: 0,
        })
    }

    /// Start the next round of the session
    pub fn next_round(&mut self) -> Result<(Point, Point), Error> {
        if matches!(self.state, SessionState::Finalized) {
            return Err(Error::InvalidArguments);
        }

        self.state = SessionState::Active;
        let k = BigUint::from_bytes_be(&rand::random::<[u8; 32]>());
        let r1 = self.g.scale(k.clone());
        let r2 = self.h.scale(k.clone());

        self.rounds.insert(self.current_round, (r1.clone(), r2.clone(), k, BigUint::from(0u32)));
        self.current_round += 1;

        Ok((r1, r2))
    }

    /// Solve the challenge for the current round
    pub fn solve_challenge(&mut self, round: usize, challenge: &BigUint) -> Result<BigUint, Error> {
        if matches!(self.state, SessionState::Finalized) {
            return Err(Error::InvalidArguments);
        }

        let (_, _, k, _) = self.rounds.get_mut(&round)
            .ok_or(Error::InvalidArguments)?;

        let s = solve_zk_challenge_s(
            &BigUint::from_bytes_be(&rand::random::<[u8; 32]>()),
            k,
            challenge,
            &self.q,
        );

        Ok(s)
    }

    /// Verify a proof for a specific round
    pub fn verify_round(&self, round: usize) -> Result<bool, Error> {
        let (r1, r2, _, s) = self.rounds.get(&round)
            .ok_or(Error::InvalidArguments)?;

        let params = VerificationParams {
            r1: r1.clone(),
            r2: r2.clone(),
            y1: self.y1.clone(),
            y2: self.y2.clone(),
            g: self.g.clone(),
            h: self.h.clone(),
            c: BigUint::from_bytes_be(&rand::random::<[u8; 32]>()),
            s: s.clone(),
            p: self.p.clone(),
        };

        verify(&params)
    }

    /// Finalize the session
    pub fn finalize(&mut self) -> Result<(), Error> {
        self.state = SessionState::Finalized;
        Ok(())
    }

    /// Get the session state
    pub fn state(&self) -> &SessionState {
        &self.state
    }

    /// Get the number of rounds
    pub fn round_count(&self) -> usize {
        self.current_round
    }

    /// Convert the session to JSON
    pub fn to_json(&self) -> Result<String, Error> {
        let rounds: Vec<_> = self.rounds.iter()
            .map(|(round, (r1, r2, _, s))| {
                json!({
                    "round": round,
                    "r1": r1.serialize(),
                    "r2": r2.serialize(),
                    "s": s.to_string(),
                })
            })
            .collect();

        let json = json!({
            "state": format!("{:?}", self.state),
            "group": self.group,
            "p": self.p.to_string(),
            "q": self.q.to_string(),
            "g": self.g.serialize(),
            "h": self.h.serialize(),
            "y1": self.y1.serialize(),
            "y2": self.y2.serialize(),
            "rounds": rounds,
            "current_round": self.current_round,
        });

        to_string(&json).map_err(|e| Error::InvalidSerialization(e.to_string()))
    }
} 