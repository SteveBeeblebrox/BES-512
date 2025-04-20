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

pub fn run() {
    // let keys = crate::operations::key_schedule::<{128/8},{128/8}, 10>(&0x0f1571c9_47d9e859_0cb7add6_af7f6798_u128.to_be_bytes());
    // println!("{:02x?}", &keys[0..16]);
    // println!("{:02x?}", &keys[16..32]);
    // println!("{:02x?}", &keys[32..48]);
    // println!("{:02x?}", &keys[48..64]);
    // println!("...");
    // println!("{:02x?}", &keys[keys.len()-16..]);
    // println!("({})", keys.len());

    // println!();
    // println!();
    // println!();

    // let mut k192 = [0; 192/8];
    // k192[..64/8].copy_from_slice(&0x_8e73b0f7_da0e6452_u64.to_be_bytes());
    // k192[64/8..].copy_from_slice(&0x_c810f32b_809079e5_62f8ead2_522c6b7bu128.to_be_bytes());

    // let keys = crate::operations::key_schedule::<{128/8}, {192/8}, 12>(&k192);
    // println!("{:02x?}", &keys[0..16]);
    // println!("{:02x?}", &keys[16..32]);
    // println!("{:02x?}", &keys[32..48]);
    // println!("{:02x?}", &keys[48..64]);
    // println!("...");
    // println!("{:02x?}", &keys[keys.len()-16..]);
    // println!("({})", keys.len());
    // // https://nvlpubs.nist.gov/nistpubs/fips/nist.fips.197.pdf

    // println!();
    // println!();
    // println!();

    // let k512 = [0 ; 512/8];
    // let keys = crate::operations::key_schedule::<{512/8}, {512/8}, 10>(&k512);
    // println!("{:02x?}", &keys[0..64]);
    // println!("{:02x?}", &keys[64..128]);
    // println!("{:02x?}", &keys[126..192]);
    // println!("{:02x?}", &keys[192..256]);
    // println!("...");
    // println!("{:02x?}", &keys[keys.len()-64..]);
    // println!("({})", keys.len());

    // let mut block = 0x000102030405060708090a0b0c0d0e0f_u128.to_be_bytes();
    // let round_key = 0x01010101010101010101010101010101_u128.to_be_bytes();

    // print_mtx!(block);
    // encryption_round(&mut block, &round_key, &AES_MIX_COLUMNS_MATRIX);
    // println!("===");
    // print_mtx!(block);
    // println!("===");
    // decryption_round(&mut block, &round_key, &AES_INV_MIX_COLUMNS_MATRIX);

    // print_mtx!(block);
    let mut msg = 0x000102030405060708090a0b0c0d0e0fu128.to_be_bytes();
    println!("{:02x?}", encrypt::<16,16,10>(&mut msg, &0x01010101010101010101010101010101u128.to_be_bytes(), crate::constants::AES_MIX_COLUMNS_MATRIX));
    println!("{:02x?}", decrypt::<16,16,10>(&mut msg, &0x01010101010101010101010101010101u128.to_be_bytes(), crate::constants::AES_INV_MIX_COLUMNS_MATRIX));
}

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