//! Roman numeral system example.

fn main() {
    // Conversion
    println!("=== Roman Numeral Conversion ===");
    for n in [1, 4, 9, 14, 42, 99, 399, 1776, 1999, 2024, 3999] {
        let roman = sankhya::roman::to_roman_str(n).unwrap();
        println!("  {n:>4} = {roman}");
    }

    // Parsing
    println!("\n=== Parsing ===");
    for s in ["XIV", "XLII", "MCMXCIX", "MMXXIV"] {
        let value = sankhya::roman::from_roman(s).unwrap();
        println!("  {s:>10} = {value}");
    }

    // Arithmetic
    println!("\n=== Arithmetic ===");
    let a = sankhya::roman::RomanNumeral::from_value(1776).unwrap();
    let b = sankhya::roman::RomanNumeral::from_value(248).unwrap();
    println!("  {a} + {b} = {}", a.add(&b).unwrap());
    println!("  {a} - {b} = {}", a.subtract(&b).unwrap());

    let x = sankhya::roman::RomanNumeral::from_value(12).unwrap();
    let y = sankhya::roman::RomanNumeral::from_value(12).unwrap();
    println!("  {x} × {y} = {}", x.multiply(&y).unwrap());

    let (q, r) = sankhya::roman::roman_divide(17, 5).unwrap();
    print!("  XVII / V = {q}");
    if let Some(rem) = r {
        println!(" remainder {rem}");
    } else {
        println!();
    }

    // Validation
    println!("\n=== Validation ===");
    for s in ["XIV", "IIII", "VV", "IC", "MCMXCIX"] {
        println!(
            "  {s:>10} — {}",
            if sankhya::roman::is_valid_roman(s) {
                "valid"
            } else {
                "INVALID"
            }
        );
    }
}
