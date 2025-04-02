#![feature(const_trait_impl)]
pub mod gfmath;

pub fn encrypt() {
    todo!("NYI")
}

pub fn decrypt() {
    todo!("NYI")
}


// Need to use 10...0 padding, PKCS won't work with 512 block
pub fn pad<const BLOCK_SIZE: usize>(_x: &[u8]) -> [u8; BLOCK_SIZE] {
    todo!("NYI")
}

pub fn unpad() {
    todo!("NYI")
}


/// $a^{12}$
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
