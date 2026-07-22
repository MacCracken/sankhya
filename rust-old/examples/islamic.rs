//! Islamic mathematics example.

fn main() {
    // Al-Khwarizmi's algebra
    println!("=== Al-Khwarizmi's Algebra ===");

    // Famous example: x² + 10x = 39 → x = 3
    let sol = sankhya::islamic::solve_al_jabr(1.0, 10.0, -39.0).unwrap();
    println!("x² + 10x = 39");
    println!("  Form: {:?}", sol.form);
    println!("  Roots: {:?}", sol.roots);

    // Two positive roots: x² - 5x + 6 = 0 → x = 2, 3
    let sol = sankhya::islamic::solve_al_jabr(1.0, -5.0, 6.0).unwrap();
    println!("\nx² - 5x + 6 = 0");
    println!("  Form: {:?}", sol.form);
    println!("  Roots: {:?}", sol.roots);

    // Completion of the square
    println!("\n=== Completion of the Square ===");
    let (x, area) = sankhya::islamic::complete_the_square(10.0, 39.0).unwrap();
    println!("x² + 10x = 39");
    println!("  x = {x:.6}");
    println!("  Completed square area = {area:.6}");

    // Khayyam cubic
    println!("\n=== Omar Khayyam's Cubics ===");
    let (ctype, roots) = sankhya::islamic::classify_khayyam_cubic(1.0, 0.0, 0.0, -8.0).unwrap();
    println!("x³ = 8");
    println!("  Type: {ctype:?}");
    println!("  Roots: {roots:?}");

    let (ctype, roots) = sankhya::islamic::classify_khayyam_cubic(1.0, 0.0, 6.0, -20.0).unwrap();
    println!("\nx³ + 6x = 20");
    println!("  Type: {ctype:?}");
    println!("  Roots: {roots:?}");

    // Hijri calendar
    println!("\n=== Hijri Calendar ===");
    let date = sankhya::islamic::jdn_to_hijri(sankhya::islamic::HIJRI_EPOCH_JDN);
    println!("Epoch: {date}");

    let today_jdn = 2_460_767.5; // ~April 2025
    let today = sankhya::islamic::jdn_to_hijri(today_jdn);
    println!("JDN {today_jdn} = {today}");
    println!("Leap year? {}", sankhya::islamic::hijri_is_leap(today.year));
}
