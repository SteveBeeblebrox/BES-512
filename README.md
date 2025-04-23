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

```bash
echo 'In a hole in the ground there lived a hobbit... - Bilbo Baggins' | cargo run -- -m block -c BES-512 -k ./512-bits.key enc | cargo run -- -m block -c BES-512 -k ./512-bits.key dec
```

```bash
cargo run -- -m cbc -c aes-128 -I hobbit.txt -k 0x000102030405060708090a0b0c0d0e0f -i 0x00112233445566778899aabbccddeeff enc | cargo run -- -m cbc -c aes-128 -I hobbit.txt -k 0x000102030405060708090a0b0c0d0e0f -i 0x00112233445566778899aabbccddeeff dec
```

```bash
time (./target/debug/bes-512 -m cbc -c aes-128 -I hobbit.txt -k 0x000102030405060708090a0b0c0d0e0f -i 0x00112233445566778899aabbccddeeff enc | ./target/debug/bes-512 -m cbc -c aes-128 -k 0x000102030405060708090a0b0c0d0e0f -i 0x00112233445566778899aabbccddeeff dec)
```


```bash
time (./target/debug/bes-512 -m cbc -c bes-512 -I hobbit.txt -k ./512-bits.key -i ./512-bits.iv enc | ./target/debug/bes-512 -m cbc -c bes-512 -k ./512-bits.key -i ./512-bits.iv dec)
```



