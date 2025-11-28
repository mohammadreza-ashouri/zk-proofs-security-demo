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
use std::collections::HashMap;

pub struct ZKComparison;

impl ZKComparison {
    pub fn snark_characteristics() -> HashMap<&'static str, &'static str> {
        let mut map = HashMap::new();
        map.insert("Proof Size", "~200 bytes (tiny)");
        map.insert("Verification Time", "~5ms (very fast)");
        map.insert("Trusted Setup", "Required ⚠️");
        map.insert("Quantum Resistant", "No");
        map.insert("Best For", "Privacy coins, rollups with small proofs");
        map.insert("Examples", "Zcash, zkSync, Polygon zkEVM");
        map
    }

    pub fn stark_characteristics() -> HashMap<&'static str, &'static str> {
        let mut map = HashMap::new();
        map.insert("Proof Size", "~100KB (larger)");
        map.insert("Verification Time", "~50ms (slower)");
        map.insert("Trusted Setup", "Not required ✓");
        map.insert("Quantum Resistant", "Yes ✓");
        map.insert("Best For", "Transparent systems, future-proof");
        map.insert("Examples", "StarkNet, StarkEx");
        map
    }

    pub fn print_comparison() {
        println!("\n{}", "=".repeat(70));
        println!("ZK-SNARKs vs ZK-STARKs COMPARISON");
        println!("{}\n", "=".repeat(70));

        println!("📊 ZK-SNARKs:");
        println!("{}", "-".repeat(70));
        let snarks = Self::snark_characteristics();
        let keys = ["Proof Size", "Verification Time", "Trusted Setup", 
                    "Quantum Resistant", "Best For", "Examples"];
        for key in keys {
            if let Some(value) = snarks.get(key) {
                println!("  {:20} {}", format!("{}:", key), value);
            }
        }

        println!("\n📊 ZK-STARKs:");
        println!("{}", "-".repeat(70));
        let starks = Self::stark_characteristics();
        for key in keys {
            if let Some(value) = starks.get(key) {
                println!("  {:20} {}", format!("{}:", key), value);
            }
        }

        println!("\n{}", "=".repeat(70));
        println!("🎯 KEY TAKEAWAY:");
        println!("  SNARKs = Small & fast, but need trusted setup");
        println!("  STARKs = Transparent & quantum-safe, but bigger");
        println!("{}\n", "=".repeat(70));
    }
}

pub fn run_demo() {
    ZKComparison::print_comparison();
}

