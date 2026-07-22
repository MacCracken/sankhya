//! Babylonian mathematics example.

fn main() {
    // Sexagesimal
    println!("=== Sexagesimal ===");
    let n = 3600u64;
    let digits = sankhya::babylonian::to_sexagesimal(n);
    let display: Vec<String> = digits.iter().map(|d| d.to_string()).collect();
    println!("{n} in base 60 = {}", display.join(";"));

    // Babylonian sqrt(2) — YBC 7289 tablet
    println!("\n=== Heron's Square Root ===");
    let sqrt2 = sankhya::babylonian::babylonian_sqrt(2.0, 10).unwrap();
    println!("sqrt(2) = {sqrt2:.15}");
    println!("true    = {:.15}", std::f64::consts::SQRT_2);

    // Plimpton 322
    println!("\n=== Plimpton 322 Triples ===");
    let triples = sankhya::babylonian::generate_plimpton_triples();
    for (i, (a, b, c)) in triples.iter().enumerate().take(5) {
        println!("  Row {}: ({a}, {b}, {c})", i + 1);
    }
    println!("  ... ({} total)", triples.len());

    // Reciprocal table
    println!("\n=== Reciprocal Table (selection) ===");
    let table = sankhya::babylonian::reciprocal_table();
    for &n in &[2u64, 3, 4, 5, 8] {
        let recip = table.get(&n).unwrap();
        let display: Vec<String> = recip.iter().map(|d| d.to_string()).collect();
        println!("  1/{n} = 0;{}", display.join(","));
    }

    // Saros cycle
    println!("\n=== Saros Cycle ===");
    let next_eclipse = sankhya::babylonian::saros_cycle(2_451_545.0);
    println!("Eclipse at J2000.0 → next at JDN {next_eclipse:.1}");
}
