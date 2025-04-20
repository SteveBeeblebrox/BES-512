#![feature(const_trait_impl,generic_const_exprs,adt_const_params,generic_arg_infer)]
// https://blog.rust-lang.org/inside-rust/2021/09/06/Splitting-const-generics.html

use konst::for_range;

mod constants;
use crate::constants::{AES_MIX_COLUMNS_MATRIX,S_BOX,INV_S_BOX,BES_MIX_COLUMNS_MATRIX,AES_INV_MIX_COLUMNS_MATRIX};

mod operations;
use crate::operations::{direction,add_round_key, sub_bytes, shift_rows, mix_columns};


macro_rules! check_eq {
    ($lhs:expr,$rhs:expr) => {
        {
            let mut t = [0u8; 16];
            t.copy_from_slice(&$lhs);
            assert_eq!(u128::from_be_bytes(t), $rhs);

        }
    };
}

macro_rules! print_mtx {
    ($x:expr) => {
        {
            for r in 0..4 {
                for c in 0..4 {
                    print!("{:02x} ",$x[r+4*c])
                }
                println!()
            }
        }
    };
}

fn encryption_round<'a, 'b, 'c, const BLOCK_SIZE: usize>(block: &'a mut [u8; BLOCK_SIZE], round_key: &'b [u8; BLOCK_SIZE], matrix: &'c [u8; BLOCK_SIZE]) -> &'a mut [u8; BLOCK_SIZE] {
    sub_bytes(block, &S_BOX);
    shift_rows::<direction::Left,_>(block);
    mix_columns(block, matrix);
    add_round_key(block, round_key);
    block
}

fn decryption_round<'a, 'b, 'c, const BLOCK_SIZE: usize>(block: &'a mut [u8; BLOCK_SIZE], round_key: &'b [u8; BLOCK_SIZE], inv_matrix: &'c [u8; BLOCK_SIZE]) -> &'a mut [u8; BLOCK_SIZE] {
    add_round_key(block, round_key);
    mix_columns(block, inv_matrix);
    shift_rows::<direction::Right,_>(block);
    sub_bytes(block, &INV_S_BOX);
    block
}

pub fn run() {
    let keys = crate::operations::key_schedule::<{128/8},{128/8}, 10>(&0x0f1571c9_47d9e859_0cb7add6_af7f6798_u128.to_be_bytes());
    println!("{:02x?}", &keys[0..16]);
    println!("{:02x?}", &keys[16..32]);
    println!("{:02x?}", &keys[32..48]);
    println!("{:02x?}", &keys[48..64]);
    println!("...");
    println!("{:02x?}", &keys[176-16..]);
    println!("({})", keys.len());

    println!();
    println!();
    println!();

    let mut k192 = [0; 192/8];
    k192[..64/8].copy_from_slice(&0x_8e73b0f7_da0e6452_u64.to_be_bytes());
    k192[64/8..].copy_from_slice(&0x_c810f32b_809079e5_62f8ead2_522c6b7bu128.to_be_bytes());

    let keys = crate::operations::key_schedule::<{128/8}, {192/8}, 12>(&k192);
    println!("{:02x?}", &keys[0..16]);
    println!("{:02x?}", &keys[16..32]);
    println!("{:02x?}", &keys[32..48]);
    println!("{:02x?}", &keys[48..64]);
    println!("...");
    println!("{:02x?}", &keys[208-16..]);
    println!("({})", keys.len());
    // https://nvlpubs.nist.gov/nistpubs/fips/nist.fips.197.pdf
    // let mut block = 0x000102030405060708090a0b0c0d0e0f_u128.to_be_bytes();
    // let round_key = 0x01010101010101010101010101010101_u128.to_be_bytes();

    // print_mtx!(block);
    // encryption_round(&mut block, &round_key, &AES_MIX_COLUMNS_MATRIX);
    // println!("===");
    // print_mtx!(block);
    // println!("===");
    // decryption_round(&mut block, &round_key, &AES_INV_MIX_COLUMNS_MATRIX);

    // print_mtx!(block);

}