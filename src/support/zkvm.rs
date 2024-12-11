/// This file allows users to define more efficient native implementations for
/// the zkvm target which can be used to speed up the operations on [Uint]'s.
///
/// The functions defined here are not meant to be used by the user, but rather
/// to be used by the library to define more efficient native implementations
/// for the zkvm target.
///
/// Currently these functions are specified to support only 256 bit [Uint]'s and
/// take pointers to their limbs as arguments. Providing other sizes
/// will result in an undefined behavior.
use core::{cmp::Ordering, mem::MaybeUninit};

use crate::Uint;

extern "C" {
    /// Add two 256-bit numbers and store in `result`.
    pub fn wrapping_add_impl(result: *mut u8, a: *const u8, b: *const u8);
    /// Subtract two 256-bit numbers and store in `result`.
    pub fn wrapping_sub_impl(result: *mut u8, a: *const u8, b: *const u8);
    /// Multiply two 256-bit numbers and store in `result`.
    pub fn wrapping_mul_impl(result: *mut u8, a: *const u8, b: *const u8);
    /// Bitwise XOR two 256-bit numbers and store in `result`.
    pub fn bitxor_impl(result: *mut u8, a: *const u8, b: *const u8);
    /// Bitwise AND two 256-bit numbers and store in `result`.
    pub fn bitand_impl(result: *mut u8, a: *const u8, b: *const u8);
    /// Bitwise OR two 256-bit numbers and store in `result`.
    pub fn bitor_impl(result: *mut u8, a: *const u8, b: *const u8);
    /// Shift left two 256-bit numbers and store in `result`.
    pub fn wrapping_shl_impl(result: *mut u8, a: *const u8, b: *const u8);
    /// Shift right two 256-bit numbers and store in `result`.
    pub fn wrapping_shr_impl(result: *mut u8, a: *const u8, b: *const u8);
    /// Arithmetic shift right two 256-bit numbers and store in `result`.
    pub fn arithmetic_shr_impl(result: *mut u8, a: *const u8, b: *const u8);
    /// Check if two 256-bit numbers are equal.
    pub fn eq_impl(a: *const u8, b: *const u8) -> bool;
    /// Compare two 256-bit numbers.
    pub fn cmp_impl(a: *const u8, b: *const u8) -> Ordering;
    /// Clone a 256-bit number into `result`. `zero` has to
    pub fn clone_impl(result: *mut u8, a: *const u8, zero: *const u8);
}

impl<const BITS: usize, const LIMBS: usize> Copy for Uint<BITS, LIMBS> {}

impl<const BITS: usize, const LIMBS: usize> Clone for Uint<BITS, LIMBS> {
    fn clone(&self) -> Self {
        if BITS == 256 {
            let mut uninit: MaybeUninit<Self> = MaybeUninit::uninit();
            unsafe {
                clone_impl(
                    (*uninit.as_mut_ptr()).limbs.as_mut_ptr() as *mut u8,
                    self.limbs.as_ptr() as *const u8,
                    Self::ZERO.limbs.as_ptr() as *const u8,
                );
            }
            return unsafe { uninit.assume_init() };
        }
        Self { limbs: self.limbs }
    }
}

impl<const BITS: usize, const LIMBS: usize> PartialEq for Uint<BITS, LIMBS> {
    fn eq(&self, other: &Self) -> bool {
        if BITS == 256 {
            unsafe {
                eq_impl(
                    self.limbs.as_ptr() as *const u8,
                    other.limbs.as_ptr() as *const u8,
                )
            }
        } else {
            self.limbs == other.limbs
        }
    }
}

impl<const BITS: usize, const LIMBS: usize> Eq for Uint<BITS, LIMBS> {}
