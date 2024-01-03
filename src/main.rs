

// https://en.wikipedia.org/wiki/SHA-2

// Note 1: All variables are 32 bit unsigned integers and addition is calculated modulo 232
// Note 2: For each round, there is one round constant k[i] and one entry in the message schedule array w[i], 0 ≤ i ≤ 63
// Note 3: The compression function uses 8 working variables, a through h
// Note 4: Big-endian convention is used when expressing the constants in this pseudocode,
//     and when parsing message block data from bytes to words, for example,
//     the first word of the input message "abc" after padding is 0x61626380


use std::slice::Chunks;

use ark_ff::PrimeField;
use ark_r1cs_std::{
    prelude::{Boolean, EqGadget, AllocVar},
    uint8::UInt8
};
use ark_relations::r1cs::{SynthesisError, ConstraintSystem, Variable};

// Initialize hash values:
// (first 32 bits of the fractional parts of the square roots of the first 8 primes 2..19):
pub const H0: u32 = 0x6a09e667;
pub const H1: u32 = 0xbb67ae85;
pub const H2: u32 = 0x3c6ef372;
pub const H3: u32 = 0xa54ff53a;
pub const H4: u32 = 0x510e527f;
pub const H5: u32 = 0x9b05688c;
pub const H6: u32 = 0x1f83d9ab;
pub const H7: u32 = 0x5be0cd19;

// Initialize array of round constants:
// (first 32 bits of the fractional parts of the cube roots of the first 64 primes 2..311):

pub const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

pub fn sha256<ConstraintF: PrimeField>(
    input: &[u8]
) -> Result<(), SynthesisError>{

    let cs = ConstraintSystem::<ConstraintF>::new_ref();
    // let input_var = cs.new_input_variable(|| input);
    let mut input_var = input
            .iter()
            .map(|&value| cs.new_input_variable(|| Ok(value.into())))
            .collect::<Result<Vec<_>, _>>()?;
    
    let bytes_len = input_var.len() * 8;

    // Push the value 0x80 into input_var
    let val: u8 = 0x80;
    let additional_input_var = cs.new_input_variable(|| Ok(val.into()))?;
    input_var = input_var.into_iter().chain(std::iter::once(additional_input_var)).collect();

    while  input_var.len() % 64 != 56{
        let val: u8 = 0;
        let additional_input_var = cs.new_input_variable(|| Ok(val.into()))?;
        input_var = input_var.into_iter().chain(std::iter::once(additional_input_var)).collect();
    }

    for i in  bytes_len.to_be_bytes().iter(){
        input_var = input_var.into_iter().chain(std::iter::once(cs.new_input_variable(|| Ok((*i).into()))?)).collect();
    }

    for Chunk in input_var.as_slice().chunks(64) {
        
    }
    Ok(())
}
fn main() {
    println!("Hello, world!");
}
