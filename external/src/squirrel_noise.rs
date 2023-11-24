// based on http://eiserloh.net/noise/SquirrelNoise5.hpp

/////////////////////////////////////////////////////////////////////////////////////////////////
// SquirrelNoise5 - Squirrel's Raw Noise utilities (version 5)
//
// This code is made available under the Creative Commons attribution 3.0 license (CC-BY-3.0 US):
//	Attribution in source code comments (even closed-source/commercial code) is sufficient.
//	License summary and text available at: https://creativecommons.org/licenses/by/3.0/us/
//
// These noise functions were written by Squirrel Eiserloh as a cheap and simple substitute for
//	the [sometimes awful] bit-noise sample code functions commonly found on the web, many of which
//	are hugely biased or terribly patterned, e.g. having bits which are on (or off) 75% or even
//	100% of the time (or are excessively overkill/slow for our needs, such as MD5 or SHA).
/////////////////////////////////////////////////////////////////////////////////////////////////

use std::num::Wrapping;

/// Fast hash of an int32 into a different (unrecognizable) uint32.
///
/// Returns an unsigned integer containing 32 reasonably-well-scrambled bits, based on the hash
///	of a given (signed) integer input parameter (position/index) and [optional] seed.  Kind of
///	like looking up a value in an infinitely large table of previously generated random numbers.
///
/// I call this particular approach SquirrelNoise5 (5th iteration of my 1D raw noise function).
///
/// Many thanks to Peter Schmidt-Nielsen whose outstanding analysis helped identify a weakness
///	in the SquirrelNoise3 code I originally used in my GDC 2017 talk, "Noise-based RNG".
///	Version 5 avoids a noise repetition found in version 3 at extremely high position values
///	caused by a lack of influence by some of the high input bits onto some of the low output bits.
///
/// The revised SquirrelNoise5 function ensures all input bits affect all output bits, and to
///	(for me) a statistically acceptable degree.  I believe the worst-case here is in the amount
///	of influence input position bit #30 has on output noise bit #0 (49.99%, vs. 50% ideal).
///
pub fn squirrel_noise5(x: i32, seed: u32) -> i32 {
    const SQ5_BIT_NOISE1: u32 = 0xd2a80a3f; // 11010010101010000000101000111111
    const SQ5_BIT_NOISE2: u32 = 0xa884f197; // 10101000100001001111000110010111
    const SQ5_BIT_NOISE3: u32 = 0x6C736F4B; // 01101100011100110110111101001011
    const SQ5_BIT_NOISE4: u32 = 0xB79F3ABB; // 10110111100111110011101010111011
    const SQ5_BIT_NOISE5: u32 = 0x1b56c4f5; // 00011011010101101100010011110101

    let mut mangled_bits = Wrapping(x as u32);
    mangled_bits *= SQ5_BIT_NOISE1;
    mangled_bits += seed;
    mangled_bits ^= mangled_bits >> 9;
    mangled_bits += SQ5_BIT_NOISE2;
    mangled_bits ^= mangled_bits >> 11;
    mangled_bits *= SQ5_BIT_NOISE3;
    mangled_bits ^= mangled_bits >> 13;
    mangled_bits += SQ5_BIT_NOISE4;
    mangled_bits ^= mangled_bits >> 15;
    mangled_bits *= SQ5_BIT_NOISE5;
    mangled_bits ^= mangled_bits >> 17;
    mangled_bits.0 as i32
}

//-----------------------------------------------------------------------------------------------
pub fn get1d_noise_zero_to_one_f32(index: i32, seed: u32) -> f32 {
    get1d_noise_zero_to_one_f64(index, seed) as f32
}

//-----------------------------------------------------------------------------------------------
pub fn get1d_noise_zero_to_one_f64(index: i32, seed: u32) -> f64 {
    const ONE_OVER_MAX_UINT: f64 = 1.0 / 0xFFFFFFFFu32 as f64;
    ONE_OVER_MAX_UINT * squirrel_noise5(index, seed) as f64
}
