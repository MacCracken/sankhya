//! Egyptian mathematics example.

fn main() {
    // Egyptian fractions
    println!("=== Egyptian Fractions ===");
    for (n, d) in [(2, 3), (3, 7), (5, 11), (7, 13)] {
        let fracs = sankhya::egyptian::decompose(n, d).unwrap();
        let terms: Vec<String> = fracs.iter().map(|d| format!("1/{d}")).collect();
        println!("  {n}/{d} = {}", terms.join(" + "));
    }

    // Doubling multiplication
    println!("\n=== Doubling Multiplication ===");
    let (a, b) = (12, 13);
    let result = sankhya::egyptian::egyptian_multiply(a, b);
    println!("  {a} x {b} = {result}");

    // Egyptian division
    println!("\n=== Egyptian Division ===");
    let (q, rem) = sankhya::egyptian::egyptian_divide(10, 3).unwrap();
    let rem_str: Vec<String> = rem.iter().map(|d| format!("1/{d}")).collect();
    println!("  10 / 3 = {q} + {}", rem_str.join(" + "));

    // Stellar decans
    println!("\n=== Stellar Decans ===");
    let sopdet = sankhya::egyptian::sopdet();
    println!(
        "  Sopdet (Sirius) = decan #{}, {} degrees",
        sopdet.number, sopdet.ecliptic_longitude
    );

    // Sothic cycle
    println!("\n=== Sothic Cycle ===");
    let drift = sankhya::egyptian::sothic_drift(100);
    println!("  After 100 years: {drift:.1} days drift");
    let (cycle, year, drift) = sankhya::egyptian::sothic_position(2_451_545.0);
    println!("  At J2000.0: cycle {cycle}, year {year}, drift {drift:.1} days");
}
