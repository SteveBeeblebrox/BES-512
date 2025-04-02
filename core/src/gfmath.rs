
/// Multiplication in $GF(2^8)$ with $m(x)=x^8+x^4+x^3+x+1$
/// ```rust
/// assert_eq!(bes_512_core::gfmath::gfmul(0xae, 0x02), 0x47);
/// ```
pub const fn gfmul(a: u8, b: u8) -> u8 {
    const T: [[u8; 256]; 256] = {
        const N: usize = 256;
        let mut t: [[u8; N]; N] = [[0u8; N]; N];
    
        const fn xtime(f: u8) -> u8 {
            (f << 1) ^ if f & 0b10000000 == 0 {
                0
            } else {
                0b00011011
            }
        }
    
        use konst::for_range;
        for_range!(a in 1..N => {
            for_range!(b in 1..N => {
                t[a][b] = match a {
                    // Identity
                    1 => b as u8,
                    // Mirror over diagonal
                    _ if a > b => t[b][a],
                    // Powers of two (1, x, x^2, ...)
                    _ if (a & (a - 1)) == 0 => xtime(t[a>>1][b]),
                    // All others computed via sum of x^i * b if x^i in a
                    _ => {
                        let mut r: u8 = 0u8;
                        for_range!(i in 0u32..8u32 => {
                            r ^= if a & (1<<i) != 0 {
                                t[2usize.pow(i)][b]
                            } else {
                                0
                            }
                        });
                        r
                    }
                }
            });
        });
        t
    };
    return T[a as usize][b as usize];
}