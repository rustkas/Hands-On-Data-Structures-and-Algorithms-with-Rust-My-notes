//! # Random Number Generators
//!
//! This module provides implementations of random number generators (`RandGen` and `BigGen`) for generating pseudo-random numbers.
//! 
//! ## Usage
//!
//! ```
//! use my_random::rand;
//!
//! // Generate a random number between 0 and 100
//! let random_number = rand(101);
//!
//! println!("Random number: {}", random_number);
//! ```
//!
//! ## `RandGen`
//!
//! `RandGen` is a pseudo-random number generator that generates `usize` values.
//!
//! ### Example
//!
//! ```
//! use my_random::RandGen;
//!
//! let mut rand_gen = RandGen::new(23, 21321, 21323, 314);
//!
//! // Generate a sequence of random numbers
//! for _ in 0..10 {
//!     let random_number = rand_gen.next().unwrap();
//!     println!("{}", random_number);
//! }
//! ```
//!
//! ## `BigGen`
//!
//! `BigGen` is a pseudo-random number generator that generates `usize` values using `BigUint` for larger ranges.
//!
//! ### Example
//!
//! ```
//! use my_random::BigGen;
//!
//! let mut big_gen = BigGen::new(123, 1000);
//!
//! // Generate a sequence of random numbers
//! for _ in 0..10 {
//!     let random_number = big_gen.next().unwrap();
//!     println!("{}", random_number);
//! }
//! ```
//!
//! ## Note
//!
//! These random number generators are not cryptographically secure.
//! ```

use num_bigint::BigUint;
use num_traits::ToPrimitive;
use std::ops::Rem;
use std::sync::Mutex;
use lazy_static::lazy_static;


lazy_static!{
    static ref RG: Mutex<RandGen> = Mutex::new(RandGen::new(23,21321,21323,314));
}


pub fn rand(max:usize)->usize{ 
    RG.lock().unwrap().next().unwrap()%max
}

pub struct RandGen {
    curr: usize,
    mul: usize,
    inc: usize,
    modulo: usize,
}

impl RandGen {
    pub fn new(curr: usize, mul: usize, inc: usize, modulo: usize) -> Self {
        RandGen {
            curr,
            mul,
            inc,
            modulo,
        }
    }
}

impl Iterator for RandGen {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr = (self.curr * self.mul + self.inc) % self.modulo;
        Some(self.curr )
    }
}

pub struct BigGen {
    curr: BigUint,
    mul: usize,
    inc: usize,
    modulo: BigUint,
    max: usize,
}

impl BigGen {
    pub fn new(curr: usize, max: usize) -> Self {
        let mut mm: BigUint = usize::max_value().into();
        mm = mm + (53 as usize);
        BigGen {
            curr: curr.into(),
            mul: 4531345392834523213,
            inc: 3251235234162363461,
            modulo: mm * usize::max_value(),
            max,
        }
    }
}

impl Iterator for BigGen {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr = (self.curr.clone() * self.mul + self.inc) % self.modulo.clone();
        self.curr.clone().rem(self.max).to_usize()
    }
}
