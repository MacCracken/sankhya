//! Cross-civilizational epoch correlation example.

fn main() {
    // Precessional ages
    println!("=== Precessional Ages ===");
    let ages = [
        (
            "Younger Dryas (~10,800 BCE)",
            sankhya::epoch::YOUNGER_DRYAS_JDN,
        ),
        (
            "~2500 BCE (Pyramid Age)",
            sankhya::epoch::julian_year_to_jdn(-2499),
        ),
        ("~0 CE", sankhya::epoch::julian_year_to_jdn(1)),
        ("J2000.0", sankhya::epoch::J2000_JDN),
    ];
    for (label, jdn) in &ages {
        let pos = sankhya::epoch::precessional_age(*jdn);
        println!(
            "  {label}: Age of {:?} ({:.0}y in, {:.1}%)",
            pos.age,
            pos.years_into_age,
            pos.fraction * 100.0
        );
    }

    // Seven Sages
    println!("\n=== Seven Sages Traditions ===");
    for tradition in sankhya::epoch::all_sages_traditions() {
        println!(
            "  {:?}: {} ({} sages)",
            tradition.civilization,
            tradition.group_name,
            tradition.sage_names.len()
        );
    }

    // Multi-calendar correlation
    println!("\n=== Epoch Correlation (Rosetta Stone) ===");
    let date = sankhya::epoch::correlate(sankhya::epoch::J2000_JDN).unwrap();
    println!("At J2000.0 (JDN {:.1}):", date.jdn);
    if let Some(lc) = &date.mayan_long_count {
        println!("  Mayan Long Count: {lc}");
    }
    if let Some(tz) = &date.tzolkin {
        println!("  Tzolkin: {tz}");
    }
    println!(
        "  Precessional Age: {:?} ({:.1}% through)",
        date.precessional_age.age,
        date.precessional_age.fraction * 100.0
    );
    println!(
        "  Sothic cycle: year {} (drift {:.1} days)",
        date.sothic_position.1, date.sothic_position.2
    );
    println!("  Julian year: {:.1}", date.julian_year);

    // BP conversions
    println!("\n=== BP Conversions ===");
    for bp in [0.0, 5_000.0, 12_800.0] {
        let jdn = sankhya::epoch::bp_to_jdn(bp);
        let year = sankhya::epoch::jdn_to_julian_year(jdn);
        println!("  {bp:.0} BP = {year:.0} CE (JDN {jdn:.1})");
    }

    // Cycle periods
    println!("\n=== Cycle Periods ===");
    use sankhya::epoch::CycleName;
    for cycle in [
        CycleName::Precession,
        CycleName::Sothic,
        CycleName::Saros,
        CycleName::VenusSynodic,
        CycleName::CalendarRound,
        CycleName::Metonic,
    ] {
        let days = sankhya::epoch::cycle_period(cycle);
        let years = days / 365.25;
        println!("  {cycle:?}: {days:.1} days ({years:.2} years)");
    }
}
