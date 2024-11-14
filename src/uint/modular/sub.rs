use crate::Uint;

pub(crate) fn sub_montgomery_form<const LIMBS: usize>(
    a: &Uint<LIMBS>,
    b: &Uint<LIMBS>,
    modulus: &Uint<LIMBS>,
) -> Uint<LIMBS> {
    a.sub_mod(b, modulus)
}
