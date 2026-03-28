//! Basic usage example for sankhya.
//!
//! Demonstrates converting dates to Mayan Long Count, computing
//! Tzolkin day signs, and Babylonian sexagesimal representation.

fn main() {
    // --- Mayan Long Count ---
    // March 26, 2026 as Julian Day Number.
    // JDN for March 26, 2026 = 2461031 (approximate).
    let jdn_today: u64 = 2_461_031;
    let lc = sankhya::mayan::LongCount::from_julian_day(jdn_today)
        .expect("JDN should be after Mayan epoch");
    println!("=== Mayan Long Count ===");
    println!("Today (JDN {jdn_today}) = {lc}");
    println!(
        "  {}.{}.{}.{}.{}",
        lc.baktun, lc.katun, lc.tun, lc.uinal, lc.kin
    );

    // Tzolkin day sign for today
    let days_from_epoch = jdn_today - sankhya::mayan::EPOCH_JDN;
    let tzolkin = sankhya::mayan::Tzolkin::from_days(days_from_epoch);
    println!("\n=== Tzolkin ===");
    println!("Today = {tzolkin}");

    // Haab date
    let haab = sankhya::mayan::Haab::from_days(days_from_epoch);
    println!("Haab = {haab}");

    // Venus phase
    let phase = sankhya::mayan::venus_phase(days_from_epoch);
    println!("Venus phase = {phase:?}");

    // --- Babylonian sexagesimal ---
    println!("\n=== Babylonian Sexagesimal ===");
    let n = 3600u64;
    let digits = sankhya::babylonian::to_sexagesimal(n);
    let display: Vec<String> = digits.iter().map(|d| d.to_string()).collect();
    println!("{n} in base 60 = {}", display.join(";"));

    // Babylonian sqrt(2) - as on the YBC 7289 tablet
    let sqrt2 = sankhya::babylonian::babylonian_sqrt(2.0, 10).expect("should converge");
    println!("Babylonian sqrt(2) = {sqrt2:.15}");

    // --- Greek: Archimedes' pi ---
    println!("\n=== Archimedes' Pi ===");
    let (lower, upper) = sankhya::greek::archimedes_pi(5);
    println!("After 5 iterations (192-gon):");
    println!("  {lower:.10} < pi < {upper:.10}");

    // --- Egyptian fractions ---
    println!("\n=== Egyptian Fractions ===");
    let fractions = sankhya::egyptian::decompose(3, 7).expect("valid fraction");
    let terms: Vec<String> = fractions.iter().map(|d| format!("1/{d}")).collect();
    println!("3/7 = {}", terms.join(" + "));

    // --- Chinese Remainder Theorem ---
    println!("\n=== Chinese Remainder Theorem ===");
    println!("Sun Tzu's problem: x = 2 (mod 3), x = 3 (mod 5), x = 2 (mod 7)");
    let result = sankhya::chinese::chinese_remainder(&[(2, 3), (3, 5), (2, 7)]);
    println!("Answer: x = {}", result.unwrap_or(0));

    // --- Vedic: Baudhayana's sqrt(2) ---
    println!("\n=== Vedic Mathematics ===");
    let sulba = sankhya::vedic::sulba_sqrt2();
    println!("Baudhayana's sqrt(2) = {sulba:.15}");
    println!("True sqrt(2)         = {:.15}", std::f64::consts::SQRT_2);
    println!(
        "Error                = {:.2e}",
        (sulba - std::f64::consts::SQRT_2).abs()
    );
}
