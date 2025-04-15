use crate::constants::{S_BOX,GF_MUL};
use konst::for_range;

struct ConstSqrt<const N: usize>;
impl<const N: usize> ConstSqrt<N> {
    const SQRT: usize = N.isqrt();
}

pub(crate) const fn transpose<const N: usize>(matrix: [u8; N]) -> [u8; N] {
    let mut t = [0; N];
    for_range!(r in 0..ConstSqrt::<N>::SQRT => {
        for_range!(c in 0..ConstSqrt::<N>::SQRT => {
            t[c + r*ConstSqrt::<N>::SQRT] = matrix[r + c*ConstSqrt::<N>::SQRT];
        });
    });
    return t;
}

#[inline(always)]
pub(crate) const fn add_round_key<'a, 'b, const N: usize>(bytes: &'a mut [u8; N], key: &'b [u8; N]) -> &'a mut [u8; N] {
    for_range!(i in 0..N => {
        bytes[i] ^= key[i]
    });
    return bytes;
}

#[inline(always)]
pub(crate) const fn sub_bytes<const N: usize>(bytes: &mut [u8; N]) -> &mut [u8; N] {
    for_range!(i in 0..N => {
        bytes[i] = S_BOX[bytes[i] as usize];
    });

    return bytes;
}

#[inline(always)]
pub(crate) const fn shift_rows<const N: usize>(bytes: &mut [u8; N]) -> &mut [u8; N] {
    for_range!(r in 1..ConstSqrt::<N>::SQRT => {
        if r % 2 == 0 {
            let mut t: u8 = bytes[r]; /* (r, 0) */
            for_range!(n in 0..ConstSqrt::<N>::SQRT/2 => {
                std::mem::swap(&mut t, &mut bytes[(r + (ConstSqrt::<N>::SQRT - (n + 1)*r%ConstSqrt::<N>::SQRT)*ConstSqrt::<N>::SQRT)%N]); /* (r, ConstMath::<N>::SQRT - (n + 1)*r%ConstMath::<N>::SQRT) */
            });
            let mut t: u8 = bytes[(r + ConstSqrt::<N>::SQRT)%N]; /* (r, 1) */
            for_range!(n in 0..ConstSqrt::<N>::SQRT/2 => {
                std::mem::swap(&mut t, &mut bytes[(r + ((ConstSqrt::<N>::SQRT - (n + 1)*r + 1)%ConstSqrt::<N>::SQRT)*ConstSqrt::<N>::SQRT)%N]); /* (r, (ConstMath::<N>::SQRT - (n + 1)*r + 1)%ConstMath::<N>::SQRT) */
            });
        } else {
            let mut t: u8 = bytes[r]; /* (r, 0) */
            for_range!(n in 0..ConstSqrt::<N>::SQRT => {
                std::mem::swap(&mut t, &mut bytes[(r + (ConstSqrt::<N>::SQRT - (n + 1)*r%ConstSqrt::<N>::SQRT)*ConstSqrt::<N>::SQRT)%N]); /* (r, ConstMath::<N>::SQRT - (n + 1)*r%ConstMath::<N>::SQRT) */
            });
        }
    });

    return bytes;
}

#[inline(always)]
pub(crate) const fn mix_columns<'a, 'b, const N: usize>(bytes: &'a mut [u8; N], matrix: &'b [u8; N]) -> &'a mut [u8; N] {
    let imm: [u8; N] = *bytes;
    for_range!(r in 0..ConstSqrt::<N>::SQRT => {
        for_range!(c in 0..ConstSqrt::<N>::SQRT => {
            bytes[r + c*ConstSqrt::<N>::SQRT] = 0;
            for_range!(n in 0..ConstSqrt::<N>::SQRT => {
                bytes[r + c*ConstSqrt::<N>::SQRT] ^= GF_MUL[matrix[r + n*ConstSqrt::<N>::SQRT] as usize][imm[n + c*ConstSqrt::<N>::SQRT] as usize];
            });
        });
    });
    
    return bytes;
}