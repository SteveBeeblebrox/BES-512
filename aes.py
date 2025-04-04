#!/usr/bin/python3.11
def nhex(N,*,len=0,prefix='0x'):
    return f'{prefix}{hex(N)[2:]:0>{len}}'

def display(**kwargs):
    for k,v in kwargs.items():
        globals()[k]=v
        print(k,'=', v)
        match v:
            case Block():
                print('\n'.join([
                    ' '.join([nhex(v[r,c],len=2,prefix='') for c in range(4)])
                for r in range(4)]))
        print()

# S-Box
S = [
    [0x63,0x7c,0x77,0x7b,0xf2,0x6b,0x6f,0xc5,0x30,0x01,0x67,0x2b,0xfe,0xd7,0xab,0x76],
    [0xca,0x82,0xc9,0x7d,0xfa,0x59,0x47,0xf0,0xad,0xd4,0xa2,0xaf,0x9c,0xa4,0x72,0xc0],
    [0xb7,0xfd,0x93,0x26,0x36,0x3f,0xf7,0xcc,0x34,0xa5,0xe5,0xf1,0x71,0xd8,0x31,0x15],
    [0x04,0xc7,0x23,0xc3,0x18,0x96,0x05,0x9a,0x07,0x12,0x80,0xe2,0xeb,0x27,0xb2,0x75],
    [0x09,0x83,0x2c,0x1a,0x1b,0x6e,0x5a,0xa0,0x52,0x3b,0xd6,0xb3,0x29,0xe3,0x2f,0x84],
    [0x53,0xd1,0x00,0xed,0x20,0xfc,0xb1,0x5b,0x6a,0xcb,0xbe,0x39,0x4a,0x4c,0x58,0xcf],
    [0xd0,0xef,0xaa,0xfb,0x43,0x4d,0x33,0x85,0x45,0xf9,0x02,0x7f,0x50,0x3c,0x9f,0xa8],
    [0x51,0xa3,0x40,0x8f,0x92,0x9d,0x38,0xf5,0xbc,0xb6,0xda,0x21,0x10,0xff,0xf3,0xd2],
    [0xcd,0x0c,0x13,0xec,0x5f,0x97,0x44,0x17,0xc4,0xa7,0x7e,0x3d,0x64,0x5d,0x19,0x73],
    [0x60,0x81,0x4f,0xdc,0x22,0x2a,0x90,0x88,0x46,0xee,0xb8,0x14,0xde,0x5e,0x0b,0xdb],
    [0xe0,0x32,0x3a,0x0a,0x49,0x06,0x24,0x5c,0xc2,0xd3,0xac,0x62,0x91,0x95,0xe4,0x79],
    [0xe7,0xc8,0x37,0x6d,0x8d,0xd5,0x4e,0xa9,0x6c,0x56,0xf4,0xea,0x65,0x7a,0xae,0x08],
    [0xba,0x78,0x25,0x2e,0x1c,0xa6,0xb4,0xc6,0xe8,0xdd,0x74,0x1f,0x4b,0xbd,0x8b,0x8a],
    [0x70,0x3e,0xb5,0x66,0x48,0x03,0xf6,0x0e,0x61,0x35,0x57,0xb9,0x86,0xc1,0x1d,0x9e],
    [0xe1,0xf8,0x98,0x11,0x69,0xd9,0x8e,0x94,0x9b,0x1e,0x87,0xe9,0xce,0x55,0x28,0xdf],
    [0x8c,0xa1,0x89,0x0d,0xbf,0xe6,0x42,0x68,0x41,0x99,0x2d,0x0f,0xb0,0x54,0xbb,0x16]
]

M = [
    [0x02,0x03,0x01,0x01],
    [0x01,0x02,0x03,0x01],
    [0x01,0x01,0x02,0x03],
    [0x03,0x01,0x01,0x02]
]

class Block:
    def __init__(self, /, value = 0):
        self.value = value

    def __repr__(self):
        return f'Block({nhex(self.value,len=32)})'
   
    def __str__(self):
        return nhex(self.value,len=32)
       
    def __getitem__(self,key):
        match key:
            case int() as n:
                if n < 0:
                    return self.__getitem__(16+n)
                elif n > 15:
                    raise IndexError()
                else:
                    return (self.value & (0xff << (15-n)*8)) >> ((15-n)*8)
            case (int() as r, int() as c):
                return self.__getitem__(r+4*c)
            case (int() as r, slice() as s):
                return [self.__getitem__((r,c)) for c in range(4)[s]]
            case (slice() as s, int() as c):
                return [self.__getitem__((r,c)) for r in range(4)[s]]
            case _:
                raise TypeError('Invalid index')

    def __setitem__(self,key,value):
        match key:
            case int() as n:
                if n < 0:
                    return self.__setitem__(16+n,value)
                elif n > 15:
                    raise IndexError()
                else:
                    self.value = (self.value & ~(0xff << (15-n)*8)) | (value << (15-n)*8)
            case (int() as r, int() as c):
                return self.__setitem__(r+4*c,value)
            case (int() as r, slice() as s):
                for c in range(4)[s]:
                    self.__setitem__((r,c),value[c])
            case (slice() as s, int() as c):
                for r in range(4)[s]:
                    self.__setitem__((r,c),value[r])
            case _:
                raise TypeError('Invalid index')
   
    def __xor__(self,other):
        match other:
            case (int() as n) | Block(value=n):
                return Block(self.value ^ n)
            case _:
                raise TypeError('Invalid xor rhs')
    def __rxor__(self,other):
        return self.__xor__(other)
    def __ixor__(self,other):
        self.value = self.__xor__(other).value
        return self
       
    def __add__(self,other):
        return self.__xor__(other)
    def __radd__(self,other):
        return self.__rxor__(other)
    def __iadd__(self,other):
        return self.__ixor__(other)

    def addRoundKey(self,K):
        self ^= K
        return self
    
    def subBytes(self):
        for r in range(4):
            for c in range(4):
                self[r,c] = S[self[r,c] >> 4][self[r,c] & 0xf]
        return self
    
    def shiftRows(self):
        for r in range(1,4):
            t = self[r,:]
            for _ in range(r):
                t=[*t[1:],t[0]]
            self[r,:] = t
        return self
    
    def mixColumns(self):
        def gfsum(l):
            import operator, functools
            return functools.reduce(operator.xor,l,0)
        
        def gfmul(a,b):
            def repeat(f, x, n):
                for _ in range(n):
                    x = f(x)
                return x
            def xmul(f):
                return ((f<<1) & 0xff) ^ (0b00011011 if 0b10000000 & f else 0)
            return gfsum(repeat(xmul,b,i) if a & (1<<i) else 0 for i in range(8))

        t = Block()
        for r in range(4):
            for c in range(4):
                t[r,c] = gfsum([gfmul(M[r][i], self[i,c]) for i in range(4)])

        self.value = t.value
        return self

###########################################

display(K=Block(0x01010101010101010101010101010101))
display(P=Block(0x000102030405060708090A0B0C0D0E0F))

State=P

# Initial AddRoundKey
K0=K
print('addRoundKey(K0):')
display(State=State.addRoundKey(K0))

# S-Box
print('subBytes():')
display(State=State.subBytes())

# Shift
print('shiftRows():')
display(State=State.shiftRows())

# Mix
print('mixColumns():')
display(State=State.mixColumns())
