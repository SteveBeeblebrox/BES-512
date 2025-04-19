use crate::constants::{S_BOX,GF_MUL};
use konst::for_range;

struct ConstSqrt<const N: usize>;
impl<const N: usize> ConstSqrt<N> {
    const SQRT: usize = N.isqrt();
}

pub(crate) const fn key_schedule<const BLOCK_SIZE: usize, const KEY_SIZE: usize, const ROUNDS: usize>(key: &[u8; KEY_SIZE]) where [(); BLOCK_SIZE*(1+ROUNDS)]: {
    let mut s= [0u8; BLOCK_SIZE*(1+ROUNDS)];
    for_range!(n in 0..KEY_SIZE => {
        s[n] = key[n];
    });

    


    
    unimplemented!();
}

pub(crate) const fn xtime(f: u8) -> u8 {
    (f << 1) ^ if f & 0b10000000 == 0 {
        0
    } else {
        0b00011011
    }
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
pub(crate) const fn add_round_key<'a, 'b, const BLOCK_SIZE: usize>(bytes: &'a mut [u8; BLOCK_SIZE], key: &'b [u8; BLOCK_SIZE]) -> &'a mut [u8; BLOCK_SIZE] {
    for_range!(i in 0..BLOCK_SIZE => {
        bytes[i] ^= key[i]
    });
    return bytes;
}

#[inline(always)]
pub(crate) const fn sub_bytes<'a, 'b, const BLOCK_SIZE: usize>(bytes: &'a mut [u8; BLOCK_SIZE], s_box: &'b [u8; 256]) -> &'a mut [u8; BLOCK_SIZE] {
    for_range!(i in 0..BLOCK_SIZE => {
        bytes[i] = s_box[bytes[i] as usize];
    });

    return bytes;
}



#[allow(non_snake_case)]
#[allow(private_interfaces)]
pub(crate) mod direction {
    enum DirectionEnum {
        Left, Right
    }
    pub(super) trait DirectionTy {
        const VALUE: DirectionEnum;
    }
    pub(crate) struct Left;
    pub(crate) struct Right;
    impl DirectionTy for Left {
        const VALUE: DirectionEnum = DirectionEnum::Left;
    }
    impl DirectionTy for Right {
        const VALUE: DirectionEnum = DirectionEnum::Right;
    }
    pub(super) const fn shift_offset<DIRECTION: DirectionTy>(lhs: usize, rhs: usize) -> usize {
        match DIRECTION::VALUE {
            DirectionEnum::Left => lhs - rhs,
            DirectionEnum::Right => lhs + rhs
        }
    }
}


#[allow(private_bounds)]
#[inline(always)]
pub(crate) const fn shift_rows<DIRECTION: direction::DirectionTy, const BLOCK_SIZE: usize>(bytes: &mut [u8; BLOCK_SIZE]) -> &mut [u8; BLOCK_SIZE] {
    for_range!(r in 1..ConstSqrt::<BLOCK_SIZE>::SQRT => {
        if r % 2 == 0 {
            let mut t: u8 = bytes[r]; /* (r, 0) */
            for_range!(n in 0..ConstSqrt::<BLOCK_SIZE>::SQRT/2 => {
                std::mem::swap(&mut t, &mut bytes[(r + direction::shift_offset::<DIRECTION>(ConstSqrt::<BLOCK_SIZE>::SQRT, (n + 1)*r%ConstSqrt::<BLOCK_SIZE>::SQRT)*ConstSqrt::<BLOCK_SIZE>::SQRT)%BLOCK_SIZE]); /* (r, ConstMath::<N>::SQRT - (n + 1)*r%ConstMath::<N>::SQRT) */
            });
            let mut t: u8 = bytes[(r + ConstSqrt::<BLOCK_SIZE>::SQRT)%BLOCK_SIZE]; /* (r, 1) */
            for_range!(n in 0..ConstSqrt::<BLOCK_SIZE>::SQRT/2 => {
                std::mem::swap(&mut t, &mut bytes[(r + ((direction::shift_offset::<DIRECTION>(ConstSqrt::<BLOCK_SIZE>::SQRT, (n + 1)*r) + 1)%ConstSqrt::<BLOCK_SIZE>::SQRT)*ConstSqrt::<BLOCK_SIZE>::SQRT)%BLOCK_SIZE]); /* (r, (ConstMath::<N>::SQRT - (n + 1)*r + 1)%ConstMath::<N>::SQRT) */
            });
        } else {
            let mut t: u8 = bytes[r]; /* (r, 0) */
            for_range!(n in 0..ConstSqrt::<BLOCK_SIZE>::SQRT => {
                std::mem::swap(&mut t, &mut bytes[(r + direction::shift_offset::<DIRECTION>(ConstSqrt::<BLOCK_SIZE>::SQRT, (n + 1)*r%ConstSqrt::<BLOCK_SIZE>::SQRT)*ConstSqrt::<BLOCK_SIZE>::SQRT)%BLOCK_SIZE]); /* (r, ConstMath::<N>::SQRT - (n + 1)*r%ConstMath::<N>::SQRT) */
            });
        }
    });

    return bytes;
}

#[inline(always)]
pub(crate) const fn mix_columns<'a, 'b, const BLOCK_SIZE: usize>(bytes: &'a mut [u8; BLOCK_SIZE], matrix: &'b [u8; BLOCK_SIZE]) -> &'a mut [u8; BLOCK_SIZE] {
    let imm: [u8; BLOCK_SIZE] = *bytes;
    for_range!(r in 0..ConstSqrt::<BLOCK_SIZE>::SQRT => {
        for_range!(c in 0..ConstSqrt::<BLOCK_SIZE>::SQRT => {
            bytes[r + c*ConstSqrt::<BLOCK_SIZE>::SQRT] = 0;
            for_range!(n in 0..ConstSqrt::<BLOCK_SIZE>::SQRT => {
                bytes[r + c*ConstSqrt::<BLOCK_SIZE>::SQRT] ^= GF_MUL[matrix[r + n*ConstSqrt::<BLOCK_SIZE>::SQRT] as usize][imm[n + c*ConstSqrt::<BLOCK_SIZE>::SQRT] as usize];
            });
        });
    });
    
    return bytes;
}