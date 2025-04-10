from Crypto.Cipher import AES
k=0x01010101010101010101010101010101
m=0x000102030405060708090a0b0c0d0e0f
print(AES.new(k.to_bytes(16,byteorder='big'), AES.MODE_ECB).encrypt(m.to_bytes(16,byteorder='big')).hex())