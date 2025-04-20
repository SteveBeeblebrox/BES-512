pub type CipherFunciton<const BLOCK_SIZE: usize, const KEY_SIZE: usize> = for<'a, 'b> fn(bytes: &'a mut [u8; BLOCK_SIZE], key: &'b [u8; KEY_SIZE]) -> &'a mut [u8; BLOCK_SIZE];

pub const BLOCK_SIZE: usize = 512/8;
pub const KEY_SIZE: usize = 512/8;
pub const ROUNDS: usize = 40;

pub mod aes_128 {
    pub const BLOCK_SIZE: usize = 128/8;
    pub const KEY_SIZE: usize = 128/8;
    pub const ROUNDS: usize = 10;
    pub const fn encrypt<'a,'b>(bytes: &'a mut [u8; BLOCK_SIZE], key: &'b [u8; KEY_SIZE]) -> &'a mut [u8; BLOCK_SIZE] {
        return bes_512_core::encrypt::<BLOCK_SIZE, KEY_SIZE, ROUNDS>(bytes, key, bes_512_core::AES_MIX_COLUMNS_MATRIX);
    }
    pub const fn decrypt<'a,'b>(bytes: &'a mut [u8; BLOCK_SIZE], key: &'b [u8; KEY_SIZE]) -> &'a mut [u8; BLOCK_SIZE] {
        return bes_512_core::decrypt::<BLOCK_SIZE, KEY_SIZE, ROUNDS>(bytes, key, bes_512_core::AES_INV_MIX_COLUMNS_MATRIX);
    }
}

pub mod aes_192 {
    pub const BLOCK_SIZE: usize = 128/8;
    pub const KEY_SIZE: usize = 192/8;
    pub const ROUNDS: usize = 12;
    pub const fn encrypt<'a,'b>(bytes: &'a mut [u8; BLOCK_SIZE], key: &'b [u8; KEY_SIZE]) -> &'a mut [u8; BLOCK_SIZE] {
        return bes_512_core::encrypt::<BLOCK_SIZE, KEY_SIZE, ROUNDS>(bytes, key, bes_512_core::AES_MIX_COLUMNS_MATRIX);
    }
    pub const fn decrypt<'a,'b>(bytes: &'a mut [u8; BLOCK_SIZE], key: &'b [u8; KEY_SIZE]) -> &'a mut [u8; BLOCK_SIZE] {
        return bes_512_core::decrypt::<BLOCK_SIZE, KEY_SIZE, ROUNDS>(bytes, key, bes_512_core::AES_INV_MIX_COLUMNS_MATRIX);
    }
}

pub mod aes_256 {
    pub const KEY_SIZE: usize = 256/8;
    pub const BLOCK_SIZE: usize = 128/8;
    pub const ROUNDS: usize = 14;
    pub const fn encrypt<'a,'b>(bytes: &'a mut [u8; BLOCK_SIZE], key: &'b [u8; KEY_SIZE]) -> &'a mut [u8; BLOCK_SIZE] {
        return bes_512_core::encrypt::<BLOCK_SIZE, KEY_SIZE, ROUNDS>(bytes, key, bes_512_core::AES_MIX_COLUMNS_MATRIX);
    }
    pub const fn decrypt<'a,'b>(bytes: &'a mut [u8; BLOCK_SIZE], key: &'b [u8; KEY_SIZE]) -> &'a mut [u8; BLOCK_SIZE] {
        return bes_512_core::decrypt::<BLOCK_SIZE, KEY_SIZE, ROUNDS>(bytes, key, bes_512_core::AES_INV_MIX_COLUMNS_MATRIX);
    }
}

pub mod util {
    /// Need to use 10...0 padding, PKCS won't work with 512 block
    pub fn pad(x: &[u8], n: usize) -> Box<[u8]> {
        [x, &[0x80], &vec![0; (n - x.len() % n) - 1]].concat().into_boxed_slice()
    }

    pub fn unpad(x: &[u8]) -> &[u8] {
        &x[0..x.iter().rposition(|&x| x == 0x80).unwrap_or(x.len())]
    }
}

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}