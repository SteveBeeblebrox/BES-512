#![feature(const_trait_impl,generic_const_exprs,adt_const_params,generic_arg_infer)]
// https://blog.rust-lang.org/inside-rust/2021/09/06/Splitting-const-generics.html

use konst::for_range;

mod constants;
use crate::constants::{AES_MIX_COLUMNS_MATRIX,S_BOX,INV_S_BOX,BES_MIX_COLUMNS_MATRIX,ROUND_CONSTANTS,AES_INV_MIX_COLUMNS_MATRIX};

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