#![feature(const_trait_impl,generic_const_exprs,adt_const_params,generic_arg_infer)]
// https://blog.rust-lang.org/inside-rust/2021/09/06/Splitting-const-generics.html

use konst::for_range;

mod constants;
use crate::constants::{S_BOX,INV_S_BOX};

pub use crate::constants::{
    AES_MIX_COLUMNS_MATRIX,AES_INV_MIX_COLUMNS_MATRIX,
    WHIRLPOOL_MIX_COLUMNS_MATRIX,WHIRLPOOL_INV_MIX_COLUMNS_MATRIX
};

mod operations;
use crate::operations::{
    direction, key_schedule, take_fixed,
    add_round_key, sub_bytes, shift_rows, mix_columns,
};

pub const fn encrypt<'a, 'b, 'c, const BLOCK_SIZE: usize, const KEY_SIZE: usize, const ROUNDS: usize>(bytes: &'a mut [u8; BLOCK_SIZE], key: &'b [u8; KEY_SIZE], matrix: &'c [u8; BLOCK_SIZE]) -> &'a mut [u8; BLOCK_SIZE] where [u8; BLOCK_SIZE*(1+ROUNDS)]: {
    let keys = &key_schedule::<BLOCK_SIZE,KEY_SIZE,ROUNDS>(key);

    add_round_key(bytes, &take_fixed(keys, 0));

    for_range!(r in 1..ROUNDS => {
        sub_bytes(bytes, &S_BOX);
        shift_rows::<direction::Left,_>(bytes);
        mix_columns(bytes, matrix);
        add_round_key(bytes, &take_fixed(keys, r*BLOCK_SIZE));
    });

    sub_bytes(bytes, &S_BOX);
    shift_rows::<direction::Left,_>(bytes);
    add_round_key(bytes, &take_fixed(keys, ROUNDS*BLOCK_SIZE));

    return bytes;
}

pub const fn decrypt<'a, 'b, 'c, const BLOCK_SIZE: usize, const KEY_SIZE: usize, const ROUNDS: usize>(bytes: &'a mut [u8; BLOCK_SIZE], key: &'b [u8; KEY_SIZE], inv_matrix: &'c [u8; BLOCK_SIZE]) -> &'a mut [u8; BLOCK_SIZE] where [u8; BLOCK_SIZE*(1+ROUNDS)]: {
    let keys = &key_schedule::<BLOCK_SIZE,KEY_SIZE,ROUNDS>(key);

    add_round_key(bytes, &take_fixed(keys, ROUNDS*BLOCK_SIZE));
    shift_rows::<direction::Right,_>(bytes);
    sub_bytes(bytes, &INV_S_BOX);

    for_range!(r in 1..ROUNDS => {
        add_round_key(bytes, &take_fixed(keys, (ROUNDS - r)*BLOCK_SIZE));
        mix_columns(bytes, inv_matrix);
        shift_rows::<direction::Right,_>(bytes);
        sub_bytes(bytes, &INV_S_BOX);
    });

    add_round_key(bytes, &take_fixed(keys, 0));

    return bytes;
}