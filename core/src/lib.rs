#![feature(const_trait_impl,generic_const_exprs,adt_const_params)]
// https://blog.rust-lang.org/inside-rust/2021/09/06/Splitting-const-generics.html

use konst::for_range;

mod constants;
use crate::constants::{AES_MIX_COLUMNS_MATRIX,BES_MIX_COLUMNS_MATRIX,ROUND_CONSTANTS};

mod operations;
use crate::operations::{add_round_key, sub_bytes, shift_rows, mix_columns};

pub fn xencrypt<'a, 'b, const BLOCK_SIZE: usize, const KEY_SIZE: usize>(block: &'a mut [u8; BLOCK_SIZE], key: &'b [u8; KEY_SIZE]) -> &'a mut [u8; BLOCK_SIZE] {
    unimplemented!();
}

pub fn xdecrypt<'a, 'b, const BLOCK_SIZE: usize, const KEY_SIZE: usize>(block: &'a mut [u8; BLOCK_SIZE], key: &'b [u8; KEY_SIZE]) -> &'a mut [u8; BLOCK_SIZE] {
    unimplemented!();
}


// Need to use 10...0 padding, PKCS won't work with 512 block
pub fn pad(x: &[u8], n: usize) -> Box<[u8]> {
    [x, &[0x80], &vec![0; (n - x.len() % n) - 1]].concat().into_boxed_slice()
}

pub fn unpad(x: &[u8]) -> &[u8] {
    &x[0..x.iter().rposition(|&x| x == 0x80).unwrap_or(x.len())]
}

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

pub fn encrypt() {
    const BLOCK_SIZE: usize = 128/8;
    const NUM_ROUNDS: usize = 10;

    let k = 0x01010101010101010101010101010101_u128.to_be_bytes();
    let mut m = 0x000102030405060708090a0b0c0d0e0f_u128.to_be_bytes();

    // Initial AddRoundKey
    add_round_key(&mut m, &k);
    check_eq!(m, 0x010003020504070609080b0a0d0c0f0e);
    
    for_range!(round in 0..NUM_ROUNDS-1 => {
        // SubBytes
        sub_bytes(&mut m);
        check_eq!(m, 0x7c637b776bf2c56f01302b67d7fe76ab);

        // ShiftRows
        shift_rows(&mut m);
        check_eq!(m, 0x7cf22bab6b30767701fe7b6fd763c567);

        
        // Mix Columns
        mix_columns(&mut m, &AES_MIX_COLUMNS_MATRIX);
        check_eq!(m, 0x75553e1087e62e150f04b858b2228c0a);

        // AddRoundKey
        // add_round_key(&mut m, 0);
        panic!();
    });

    check_eq!(m,0x3a0352540ea9ec5626fa83c03d3b8403);
    print!("Ok!");

}