extern crate rand_core;
extern crate rand;

use rand_core::{RngCore, Error, impls, SeedableRng};

#[derive(Debug,Clone, PartialEq, Eq)]
pub struct RandXorShift {
    u: u64,
    v: u64,
    s: u32,
}

impl RngCore for RandXorShift {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }
    
    #[inline]
    fn next_u64(&mut self) -> u64 {
        //rotates are modulo the number of bits
        self.u ^= self.v.rotate_right(self.s);
        self.v ^= self.u.rotate_right(self.s);
        
        //this would cause u and v to swap value when s is zero
        //see XOR swap algorithm
        //self.u ^= self.v;
        
        //this will eventually roll over.  That's ok.
        self.s = self.s.wrapping_add(1);
        
        self.u
    }
    
    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }
    
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}

impl SeedableRng for RandXorShift {
    type Seed = [u8; 16];
    fn from_seed(seed: Self::Seed) -> Self {
        let mut u = 0u64;
        let mut v = 0u64;
        for (i,b) in seed.iter().enumerate() {
            let w = if *b == 0 {i as u64} else {*b as u64};
            u = (u ^ w).rotate_right(23);
            v = (v ^ w).rotate_right(37);
        }
        RandXorShift {u,v,s:0}
    }
    
    #[inline]
    fn seed_from_u64(seed: u64) -> Self {
        RandXorShift {
            u: seed,
            v: seed,
            s: 1,
        }
    }

    fn from_rng<R: RngCore>(mut rng: R) -> Result<Self, Error> {
        let mut b = [0u8; 16];
        loop {
            rng.try_fill_bytes(&mut b[..])?;
            if !b.iter().all(|&x| x == 0) {
                break;
            }
        }

        Ok(Self::from_seed(b))
    }
}

//  ============================================================
//  Unit Test Cases
//  ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    
    #[test]
    fn test_seeded(){
        let mut g1 = RandXorShift::from_entropy();
        let mut g2 = RandXorShift::from_entropy();
        let mut g3 = RandXorShift::from_entropy();
        let r1 = g1.next_u64();
        let r2 = g2.next_u64();
        let r3 = g3.next_u64();
        assert_ne!(r1,r2);
        assert_ne!(r1,r3);
        assert_ne!(r2,r3);
        println!("r1: {}",r1);
        println!("r2: {}",r2);
        println!("r3: {}",r3);
    }
    
    fn test_avg(){
        
    }
    
    
}