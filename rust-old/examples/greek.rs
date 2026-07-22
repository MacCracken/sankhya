//! Greek mathematics example.

fn main() {
    // Golden ratio
    println!("=== Golden Ratio ===");
    println!("PHI = {:.15}", sankhya::greek::PHI);
    let ratio = sankhya::greek::fibonacci_ratio(40);
    println!("F(41)/F(40) = {ratio:.15}");
    println!("Difference  = {:.2e}", (ratio - sankhya::greek::PHI).abs());

    // Sieve of Eratosthenes
    println!("\n=== Sieve of Eratosthenes ===");
    let primes = sankhya::greek::sieve(100);
    println!("Primes below 100 ({} total):", primes.len());
    let strs: Vec<String> = primes.iter().map(|p| p.to_string()).collect();
    println!("  {}", strs.join(", "));

    // Euclidean GCD/LCM
    println!("\n=== Euclidean Algorithm ===");
    println!("GCD(48, 18) = {}", sankhya::greek::gcd(48, 18));
    println!("LCM(12, 18) = {}", sankhya::greek::lcm(12, 18));

    // Archimedes' pi
    println!("\n=== Archimedes' Pi ===");
    for iters in [0, 1, 5, 10, 20] {
        let (lo, hi) = sankhya::greek::archimedes_pi(iters);
        let sides = 6 * (1u64 << iters);
        println!("  {sides:>7}-gon: {lo:.12} < pi < {hi:.12}");
    }

    // Antikythera mechanism
    println!("\n=== Antikythera Mechanism ===");
    let ratios = sankhya::greek::antikythera_gear_ratios();
    for (name, cycle) in &ratios {
        println!(
            "  {name:>15}: {} teeth — {}",
            cycle.gear_teeth, cycle.description
        );
    }
}
