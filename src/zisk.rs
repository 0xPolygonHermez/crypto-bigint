#![allow(unsafe_code)]

use crate::{Uint, Limb};

pub const ZISK_U256_WORDS: usize = 4;

extern "C" {
    fn syscall_add_256_f(state: *mut [u64; 12]);
    fn syscall_addc_256_f(state: *mut [u64; 9]);
}

pub(crate) fn add_mod<const LIMBS: usize>(a: &Uint<LIMBS>, b: &Uint<LIMBS>, p: &Uint<LIMBS>) -> Uint<LIMBS> {
    let mut state = [0_u64; 12];
    state[0..4].copy_from_slice(&a.to_words());
    state[4..8].copy_from_slice(&b.to_words());
    state[8..12].copy_from_slice(&p.to_words());

    unsafe { syscall_add_256_f(&mut state) };

    let mut result = [0u64; LIMBS];
    result[0..4].copy_from_slice(&state[0..4]);
    let result: Uint<LIMBS> = Uint::from_words(result);

    return result;
}

pub(crate) fn adc<const LIMBS: usize>(a: &Uint<LIMBS>, b: &Uint<LIMBS>, mut carry: Limb) -> (Uint<LIMBS>, Limb) {
    let mut state = [0_u64; 9];
    state[0..4].copy_from_slice(&a.to_words());
    state[4..8].copy_from_slice(&b.to_words());
    state[8] = carry.into();

    unsafe { syscall_addc_256_f(&mut state) };

    let mut result = [0u64; LIMBS];
    result[0..4].copy_from_slice(&state[0..4]);
    let result: Uint<LIMBS> = Uint::from_words(result);

    carry = Limb(state[8]);

    return (result, carry);
}