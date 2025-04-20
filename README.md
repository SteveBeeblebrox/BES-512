# BES-512
https://www.rust-lang.org/tools/install
<!-- https://stackoverflow.com/questions/46495063/how-to-write-math-formulas-for-rust-documentation -->
<!-- https://crates.io/crates/haybale-pitchfork -->
<!-- https://docs.rs/gf256/0.3.0/gf256/p/index.html#constant-time -->
<!-- https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html -->
<!-- set html header via build.rs? -->
```bash
echo 'Hello World! <3' | cargo run -- --mode BLOCK --cipher AES-128 --key 0x0f1571c947d9e8590cb7add6af7f6798 enc | cargo run -- --mode BLOCK --cipher AES-128 --key 0x0f1571c947d9e8590cb7add6af7f6798 dec
```