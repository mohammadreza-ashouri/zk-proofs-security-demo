mod simple_zk_proof;
mod zk_comparison;
mod zk_vulnerabilities;

fn main() {
    println!("\n");
    println!("{}", "█".repeat(70));
    println!("      ZERO-KNOWLEDGE PROOFS: SECURITY ANALYSIS");
    println!("                  ByteScan Security");
    println!("            Complete Tutorial & Demonstration");
    println!("{}", "█".repeat(70));

    println!("\n\n📚 MODULE 1: SIMPLE ZERO-KNOWLEDGE PROOF");
    println!("Demonstrating the Schnorr Protocol\n");
    simple_zk_proof::run_demo();

    println!("\n\n📚 MODULE 2: ZK-SNARKs vs ZK-STARKs");
    println!("Comparing the two major ZK proof systems\n");
    zk_comparison::run_demo();

    println!("\n\n📚 MODULE 3: SECURITY VULNERABILITIES");
    println!("Critical flaws that have cost millions\n");
    zk_vulnerabilities::run_demo();

    println!("\n{}", "█".repeat(70));
    println!("         Tutorial complete! Visit ByteScan.net");
    println!("{}", "█".repeat(70));
    println!();
}