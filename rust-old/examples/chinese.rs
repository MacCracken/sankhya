//! Chinese mathematics example.

fn main() {
    // Chinese Remainder Theorem
    println!("=== Chinese Remainder Theorem ===");
    println!("Sun Tzu's problem: x ≡ 2 (mod 3), x ≡ 3 (mod 5), x ≡ 2 (mod 7)");
    let result = sankhya::chinese::chinese_remainder(&[(2, 3), (3, 5), (2, 7)]);
    println!("Answer: x = {}", result.unwrap());

    // Rod numerals
    println!("\n=== Rod Numerals ===");
    let a = sankhya::chinese::RodNumeral::new(42);
    let b = sankhya::chinese::RodNumeral::new(17);
    println!("  {a} + {b} = {}", sankhya::chinese::rod_add(a, b));
    println!("  {a} - {b} = {}", sankhya::chinese::rod_subtract(a, b));
    println!("  {a} x {b} = {}", sankhya::chinese::rod_multiply(a, b));

    // Magic squares
    println!("\n=== Lo Shu Magic Square (3x3) ===");
    let sq = sankhya::chinese::magic_square(3).unwrap();
    for row in &sq {
        let nums: Vec<String> = row.iter().map(|n| format!("{n:2}")).collect();
        println!("  {}", nums.join(" "));
    }
    let magic_sum: u64 = sq[0].iter().sum();
    println!("  Magic constant = {magic_sum}");
    println!("  Valid = {}", sankhya::chinese::is_magic_square(&sq));

    // 5x5 magic square
    println!("\n=== Siamese Method (5x5) ===");
    let sq5 = sankhya::chinese::magic_square(5).unwrap();
    for row in &sq5 {
        let nums: Vec<String> = row.iter().map(|n| format!("{n:2}")).collect();
        println!("  {}", nums.join(" "));
    }
    let sum5: u64 = sq5[0].iter().sum();
    println!("  Magic constant = {sum5}");
}
