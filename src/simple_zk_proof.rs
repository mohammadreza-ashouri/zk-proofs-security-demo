//! Module: [Module Name]
//! 
//! Part of Zero-Knowledge Proofs Security Analysis
//! Author: Mohammad Reza Ashouri | ByteScan Security
//! 
//! For the complete project and more security research:
//! GitHub: github.com/mohammadreza-ashouri/zk-proofs-security-demo
//! Website: bytescan.net
//! 
//! Connect: linkedin.com/in/drashouri | @ashouri777 | @Bytescan_
//! 
use rand::Rng;

pub struct SimpleZKProof {
    p: u64,
    g: u64,
    secret_x: u64,
    h: u64,
}

impl SimpleZKProof {
    pub fn new() -> Self {
        SimpleZKProof {
            p: 23,
            g: 5,
            secret_x: 0,
            h: 0,
        }
    }

    pub fn setup(&mut self, secret_x: u64) -> u64 {
        self.secret_x = secret_x;
        self.h = Self::mod_pow(self.g, secret_x, self.p);
        self.h
    }

    pub fn prove(&self) -> (u64, u64, u64) {
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(1..self.p - 1);
        let t = Self::mod_pow(self.g, r, self.p);
        let c = rng.gen_range(1..self.p - 1);
        let s = (r + c * self.secret_x) % (self.p - 1);
        (t, c, s)
    }

    pub fn verify(&self, h: u64, t: u64, c: u64, s: u64) -> bool {
        let left = Self::mod_pow(self.g, s, self.p);
        let right = (t * Self::mod_pow(h, c, self.p)) % self.p;
        left == right
    }

    fn mod_pow(base: u64, exp: u64, modulus: u64) -> u64 {
        let mut result = 1u64;
        let mut base = base % modulus;
        let mut exp = exp;

        while exp > 0 {
            if exp % 2 == 1 {
                result = (result * base) % modulus;
            }
            exp >>= 1;
            base = (base * base) % modulus;
        }
        result
    }
}

pub fn run_demo() {
    println!("\n{}", "=".repeat(60));
    println!("SIMPLE ZERO-KNOWLEDGE PROOF DEMO");
    println!("{}\n", "=".repeat(60));

    let mut zk = SimpleZKProof::new();

    let secret = 7;
    let h = zk.setup(secret);
    println!("Public value h = g^x mod p = {}", h);
    println!("Secret x = {} (never revealed!)\n", secret);

    let (t, c, s) = zk.prove();
    println!("Proof generated:");
    println!("  Commitment t = {}", t);
    println!("  Challenge c = {}", c);
    println!("  Response s = {}\n", s);

    let is_valid = zk.verify(h, t, c, s);
    println!("Proof valid? {}", is_valid);
    println!("\n✓ Verified WITHOUT learning secret x!");
   println!("{}", "=".repeat(60));
   
}