// Need to use 10...0 padding, PKCS won't work with 512 block
pub fn pad(x: &[u8], n: usize) -> Box<[u8]> {
    [x, &[0x80], &vec![0; (n - x.len() % n) - 1]].concat().into_boxed_slice()
}

pub fn unpad(x: &[u8]) -> &[u8] {
    &x[0..x.iter().rposition(|&x| x == 0x80).unwrap_or(x.len())]
}