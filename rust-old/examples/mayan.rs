//! Mayan mathematics and calendar example.

fn main() {
    // Long Count: December 21, 2012 (JDN 2456283)
    let jdn: u64 = 2_456_283;
    let lc = sankhya::mayan::LongCount::from_julian_day(jdn).unwrap();
    println!("=== Mayan Long Count ===");
    println!("JDN {jdn} = {lc}");

    // Tzolkin and Haab for today-ish (JDN 2461031)
    let jdn_today: u64 = 2_461_031;
    let days = jdn_today - sankhya::mayan::EPOCH_JDN;
    let tzolkin = sankhya::mayan::Tzolkin::from_days(days);
    let haab = sankhya::mayan::Haab::from_days(days);
    println!("\nJDN {jdn_today}:");
    println!("  Tzolkin = {tzolkin}");
    println!("  Haab    = {haab}");

    // Venus phase
    let phase = sankhya::mayan::venus_phase(days);
    println!("  Venus   = {phase:?}");

    // Vigesimal
    let digits = sankhya::mayan::to_vigesimal(8000);
    let display: Vec<String> = digits.iter().map(|d| d.to_string()).collect();
    println!("\n8000 in base 20 = {}", display.join("."));

    // Calendar Round search
    use sankhya::mayan::{DaySign, HaabMonth};
    let next =
        sankhya::mayan::find_calendar_round(4, DaySign::Ahau, 8, HaabMonth::Kumku, 1).unwrap();
    println!("\nNext 4 Ahau 8 Kumku after day 1 = day {next}");
}
