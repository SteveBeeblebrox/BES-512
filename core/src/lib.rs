#![feature(const_trait_impl)]
pub mod gfmath;

pub fn encrypt() {
    todo!("NYI")
}

pub fn decrypt() {
    todo!("NYI")
}


// Need to use 10...0 padding, PKCS won't work with 512 block
pub fn pad(x: &[u8], n: usize) -> Box<[u8]> {
    [x, &[0x80], &vec![0; (n - x.len() % n) - 1]].concat().into_boxed_slice()
}

pub fn unpad(x: &[u8]) -> &[u8] {
    &x[0..x.iter().rposition(|&x| x == 0x80).unwrap_or(x.len())]
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

    // I don't know if I'm misunderstanding the block, I'm not sure if it's supposed to be adding a full other block at the end
    #[test]
    fn pad_fullBlock() {
       let fullBlock: &[u8] = &[0x01,0x02,0x03,0x04,0x05,0x06,0x07,0x08];
       let output: &[u8] = &[0x01,0x02,0x03,0x04,0x05,0x06,0x07,0x08,0x80,0x00,0x00,0x00,0x00,0x00,0x00,0x00];
       let result = pad(fullBlock,8);
       assert_eq!(result.as_ref(),output);
    }
    
    #[test]
    fn pad_halfFullBlock() {
       let halfFullBlock: &[u8] = &[0x01,0x02,0x03,0x04];
       let output: &[u8] = &[0x01,0x02,0x03,0x04,0x80,0x00,0x00,0x00];
       let result = pad(halfFullBlock,8);
       assert_eq!(result.as_ref(),output);
    }

    #[test]
    fn pad_oddBlock() {
       let oddBlock: &[u8] = &[0x01,0x02,0x03];
       let output: &[u8] = &[0x01,0x02,0x03,0x80,0x00,0x00,0x00,0x00];
       let result = pad(oddBlock,8);
       assert_eq!(result.as_ref(),output);
    }

    #[test]
    fn pad_nMinus1Block() {
        let nMinus1Block: &[u8] = &[0x01,0x02,0x03,0x04,0x05,0x06,0x07];
        let output: &[u8] = &[0x01,0x02,0x03,0x04,0x05,0x06,0x07,0x80];
        let result = pad(nMinus1Block,8);
        assert_eq!(result.as_ref(),output);
    }

    #[test]
    fn pad_emptyBlock() {
        let emptyBlock: &[u8] = &[];
        let output: &[u8] = &[0x80,0x00,0x00,0x00,0x00,0x00,0x00,0x00];
        let result = pad(emptyBlock,8);
        assert_eq!(result.as_ref(),output);   
    }

    #[test]
    fn pad_size1Block() {
        let size1Block: &[u8] = &[0x01];
        let output: &[u8] = &[0x01,0x80,0x00,0x00,0x00,0x00,0x00,0x00];
        let result = pad(size1Block,8);
        assert_eq!(result.as_ref(),output);   
    }

    #[test]
    fn pad_size1andahalfNBlock() {
        let size1andahalfNBlock : &[u8] = &[0x01,0x02,0x03,0x04,0x05,0x06,0x07,0x08,0x09,0x10,0x11,0x12];
        let output: &[u8] = &[0x01,0x02,0x03,0x04,0x05,0x06,0x07,0x08,0x09,0x10,0x11,0x12,0x80,0x00,0x00,0x00];
        let result = pad(size1andahalfNBlock,8);
        assert_eq!(result.as_ref(),output);
    }


    #[test]
    fn unpad_size1Block() {
        let output: &[u8] = &[0x01];
        let size1Block: &[u8] = &[0x01,0x80,0x00,0x00,0x00,0x00,0x00,0x00];
        let result = unpad(size1Block);
        assert_eq!(result.as_ref(),output);
    }

}
