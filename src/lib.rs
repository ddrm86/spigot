use std::convert::TryFrom;
use wasm_bindgen::prelude::*;
use num_bigint::BigInt;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Based on: https://mail.python.org/pipermail/edu-sig/2006-July/006810.html
#[wasm_bindgen]
pub struct PiGenerator {
    q: BigInt,
    r: BigInt,
    t: BigInt,
    k: BigInt,
    n: BigInt,
    l: BigInt,
}

#[wasm_bindgen]
impl PiGenerator {
    pub fn new() -> Self {
        Self {
            q: BigInt::from(1),
            r: BigInt::from(0),
            t: BigInt::from(1),
            k: BigInt::from(1),
            n: BigInt::from(3),
            l: BigInt::from(3),
        }
    }

    pub fn next_digit(&mut self) -> u8 {
        let mut next_digit: u8 = 0;
        let mut qc: BigInt;
        let mut rc: BigInt;
        let mut tc: BigInt;
        let mut kc: BigInt;
        let mut nc: BigInt;
        let mut lc: BigInt;
        let mut generated = false;

        while !generated {
            if 4 * &self.q + &self.r - &self.t < &self.n * &self.t {
                generated = true;
                next_digit = u8::try_from(&self.n).unwrap();
                qc = 10 * &self.q;
                rc = 10 * (&self.r - &self.n * &self.t);
                tc = self.t.clone();
                kc = self.k.clone();
                nc = (10 * (3 * &self.q + &self.r)) / &self.t - 10 * &self.n;
                lc = self.l.clone();
            } else {
                qc = &self.q * &self.k;
                rc = (2 * &self.q + &self.r) * &self.l;
                tc = &self.t * &self.l;
                kc = &self.k + 1;
                nc = (&self.q * (7 * &self.k + 2) + &self.r * &self.l) / (&self.t * &self.l);
                lc = &self.l + 2;
            }
            self.q = qc;
            self.r = rc;
            self.t = tc;
            self.k = kc;
            self.n = nc;
            self.l = lc;
        }
        next_digit
    }
}

impl Default for PiGenerator {
    fn default() -> Self {
        PiGenerator::new()
    }
}
