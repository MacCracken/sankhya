//! Vedic mathematics example.

fn main() {
    // Baudhayana's sqrt(2)
    println!("=== Sulba Sutra ===");
    let sulba = sankhya::vedic::sulba_sqrt2();
    println!("Baudhayana's sqrt(2) = {sulba:.15}");
    println!("True sqrt(2)         = {:.15}", std::f64::consts::SQRT_2);
    println!(
        "Error                = {:.2e}",
        (sulba - std::f64::consts::SQRT_2).abs()
    );

    // Pythagorean theorem
    println!(
        "\nDiagonal of 3x4 rectangle = {}",
        sankhya::vedic::sulba_diagonal(3.0, 4.0)
    );

    // Nikhilam multiplication
    println!("\n=== Nikhilam Multiplication ===");
    let (base, da, db, cross, product) = sankhya::vedic::vedic_multiply_nikhilam(97, 96).unwrap();
    println!("97 x 96 (base {base}):");
    println!("  Complements: {da}, {db}");
    println!("  Cross: {cross}");
    println!("  Product: {product}");

    // Katapayadi
    println!("\n=== Katapayadi Encoding ===");
    for n in [0, 1, 42, 314] {
        let encoded = sankhya::vedic::katapayadi_encode(n);
        let decoded = sankhya::vedic::katapayadi_decode(&encoded).unwrap();
        println!("  {n} → {encoded} → {decoded}");
    }

    // Meru Prastara (Pascal's triangle)
    println!("\n=== Meru Prastara ===");
    let triangle = sankhya::vedic::meru_prastara(6).unwrap();
    for row in &triangle {
        let nums: Vec<String> = row.iter().map(|n| n.to_string()).collect();
        println!("  {}", nums.join(" "));
    }
}
