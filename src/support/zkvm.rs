/// This file allows users to define more efficient native implementations for the zkvm target 
/// which can be used to speed up the operations on [Uint]'s.
///
/// The functions defined here are not meant to be used by the user, but rather to be used by the
/// library to define more efficient native implementations for the zkvm target.
///
/// Currently these functions are specified to support only 256 bit [Uint]'s and take pointers to
/// the limbs `[u64;4]` as arguments. Providing other sizes will result in an undefined behavior.

use core::{cmp::Ordering, mem::MaybeUninit};

use crate::Uint;

extern "C" {
    /// Add two 256-bit numbers and store in `result`.
    pub fn wrapping_add_impl(a: *const u64, b: *const u64, result: *mut u64);
    /// Subtract two 256-bit numbers and store in `result`.
    pub fn wrapping_sub_impl(a: *const u64, b: *const u64, result: *mut u64);
    /// Multiply two 256-bit numbers and store in `result`.
    pub fn wrapping_mul_impl(a: *const u64, b: *const u64, result: *mut u64);
    /// Bitwise XOR two 256-bit numbers and store in `result`.
    pub fn bitxor_impl(a: *const u64, b: *const u64, result: *mut u64);
    /// Bitwise AND two 256-bit numbers and store in `result`.
    pub fn bitand_impl(a: *const u64, b: *const u64, result: *mut u64);
    /// Bitwise OR two 256-bit numbers and store in `result`.
    pub fn bitor_impl(a: *const u64, b: *const u64, result: *mut u64);
    /// Shift left two 256-bit numbers and store in `result`.
    pub fn wrapping_shl_impl(a: *const u64, b: *const u64, result: *mut u64);
    /// Shift right two 256-bit numbers and store in `result`.
    pub fn wrapping_shr_impl(a: *const u64, b: *const u64, result: *mut u64);
    /// Arithmetic shift right two 256-bit numbers and store in `result`.
    pub fn arithmetic_shr_impl(a: *const u64, b: *const u64, result: *mut u64);
    /// Check if two 256-bit numbers are equal.
    pub fn eq_impl(a: *const u64, b: *const u64) -> bool;
    /// Compare two 256-bit numbers.
    pub fn cmp_impl(a: *const u64, b: *const u64) -> Ordering;
    /// Clone a 256-bit number into `result`. `zero` has to 
    pub fn clone_impl(a: *const u64, zero: *const u64, result: *mut u64);
}

impl<const BITS: usize, const LIMBS: usize> Copy for Uint<BITS, LIMBS> {}

impl<const BITS: usize, const LIMBS: usize> Clone for Uint<BITS, LIMBS> {
    fn clone(&self) -> Self {
        if BITS == 256 {
            let mut uninit: MaybeUninit<Self> = MaybeUninit::uninit();
            unsafe {
                clone_impl(
                    self.limbs.as_ptr(),
                    Self::ZERO.limbs.as_ptr(),
                    (*uninit.as_mut_ptr()).limbs.as_mut_ptr(),
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
            unsafe { eq_impl(self.limbs.as_ptr(), other.limbs.as_ptr()) }
        } else {
            self.limbs == other.limbs
        }
    }
}

impl<const BITS: usize, const LIMBS: usize> Eq for Uint<BITS, LIMBS> {}
