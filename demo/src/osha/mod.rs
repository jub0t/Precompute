// Optimized Sha256

const BLOCK_SIZE: usize = 64;
const DIGEST_SIZE: usize = 32;
const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

#[derive(Clone, Copy)]
pub struct Sha256 {
    h: [u32; 8],
    len: u64,
    block: [u8; BLOCK_SIZE],
    block_len: usize,
}

impl Sha256 {
    pub fn new() -> Self {
        Sha256 {
            h: [
                0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
                0x5be0cd19,
            ],
            len: 0,
            block: [0; BLOCK_SIZE],
            block_len: 0,
        }
    }

    fn process_block(&mut self) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                self.block[i * 4],
                self.block[i * 4 + 1],
                self.block[i * 4 + 2],
                self.block[i * 4 + 3],
            ]);
        }

        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }

        let mut a = self.h[0];
        let mut b = self.h[1];
        let mut c = self.h[2];
        let mut d = self.h[3];
        let mut e = self.h[4];
        let mut f = self.h[5];
        let mut g = self.h[6];
        let mut h = self.h[7];

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        self.h[0] = self.h[0].wrapping_add(a);
        self.h[1] = self.h[1].wrapping_add(b);
        self.h[2] = self.h[2].wrapping_add(c);
        self.h[3] = self.h[3].wrapping_add(d);
        self.h[4] = self.h[4].wrapping_add(e);
        self.h[5] = self.h[5].wrapping_add(f);
        self.h[6] = self.h[6].wrapping_add(g);
        self.h[7] = self.h[7].wrapping_add(h);
    }

    pub fn reset(&mut self) {
        self.h = [
            0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
            0x5be0cd19,
        ];
        self.len = 0;
        self.block = [0; BLOCK_SIZE];
        self.block_len = 0;
    }

    pub fn update(&mut self, data: &[u8]) {
        self.len += data.len() as u64;

        let mut i = 0;
        while i < data.len() {
            let remaining = data.len() - i;
            if remaining >= BLOCK_SIZE - self.block_len {
                self.block[self.block_len..BLOCK_SIZE]
                    .copy_from_slice(&data[i..i + BLOCK_SIZE - self.block_len]);
                self.block_len = BLOCK_SIZE;
                self.process_block();
                i += BLOCK_SIZE - self.block_len;
                self.block_len = 0;
            } else {
                self.block[self.block_len..self.block_len + remaining].copy_from_slice(&data[i..]);
                self.block_len += remaining;
                i += remaining;
            }
        }
    }

    pub fn finalize(mut self) -> [u8; DIGEST_SIZE] {
        let mut digest = [0u8; DIGEST_SIZE];
        let len_bits = self.len * 8;

        self.update(&[0x80]);

        if self.block_len > BLOCK_SIZE - 8 {
            while self.block_len < BLOCK_SIZE {
                self.update(&[0]);
            }
            self.process_block();
        }

        while self.block_len < BLOCK_SIZE - 8 {
            self.update(&[0]);
        }

        for i in 0..8 {
            self.block[56 + i] = ((len_bits >> (8 * (7 - i))) & 0xFF) as u8;
        }

        self.process_block();

        for i in 0..8 {
            let h = self.h[i];
            digest[i * 4..i * 4 + 4].copy_from_slice(&h.to_be_bytes());
        }

        digest
    }
}

pub fn digest_string(data: [u8; 32]) -> String {
    let mut hash = String::with_capacity(data.len() * 2);

    for byte in data {
        hash.push_str(&format!("{:02x}", byte));
    }

    hash
}
