//! Chinese mathematics.
//!
//! Implements rod numeral arithmetic, the Chinese Remainder Theorem
//! (Sun Tzu's algorithm), and magic square construction.
//!
//! # Historical Context
//!
//! Chinese mathematics has a continuous tradition spanning over 2000 years.
//! The Sunzi Suanjing (Master Sun's Mathematical Manual, c. 3rd century CE)
//! contains the earliest known statement of the Chinese Remainder Theorem.
//! Counting rods (suanchou) were used for calculation from the Warring
//! States period (c. 475 BCE) and featured a place-value system with
//! alternating vertical/horizontal representation.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Rod numerals
// ---------------------------------------------------------------------------

/// A Chinese counting rod numeral.
///
/// Counting rods used vertical bars for units in odd places and
/// horizontal bars for units in even places. This type stores the
/// integer value and provides display formatting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RodNumeral {
    /// The integer value represented.
    pub value: i64,
}

impl RodNumeral {
    /// Create a new rod numeral.
    #[must_use]
    #[inline]
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}

impl core::fmt::Display for RodNumeral {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Display as decimal with rod-style digit representation
        // Vertical rods: | for 1-5, horizontal: - for 1-5
        // We show a simplified ASCII representation
        if self.value == 0 {
            return write!(f, "[ ]");
        }

        let abs_val = self.value.unsigned_abs();
        let sign = if self.value < 0 { "-" } else { "" };

        write!(f, "{sign}[")?;
        let digits: Vec<u8> = {
            let mut d = Vec::new();
            let mut n = abs_val;
            if n == 0 {
                d.push(0);
            } else {
                while n > 0 {
                    d.push((n % 10) as u8);
                    n /= 10;
                }
                d.reverse();
            }
            d
        };

        for (i, &digit) in digits.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            // Odd positions (ones, hundreds, ...) use vertical rods |
            // Even positions (tens, thousands, ...) use horizontal rods -
            let pos_from_right = digits.len() - 1 - i;
            let rod_char = if pos_from_right.is_multiple_of(2) {
                '|'
            } else {
                '-'
            };

            if digit == 0 {
                write!(f, " ")?;
            } else {
                for _ in 0..digit.min(5) {
                    write!(f, "{rod_char}")?;
                }
                if digit > 5 {
                    write!(f, "+")?;
                    for _ in 0..(digit - 5) {
                        write!(f, "{rod_char}")?;
                    }
                }
            }
        }
        write!(f, "]")
    }
}

/// Add two rod numerals.
#[must_use]
#[inline]
pub fn rod_add(a: RodNumeral, b: RodNumeral) -> RodNumeral {
    RodNumeral::new(a.value.wrapping_add(b.value))
}

/// Subtract two rod numerals.
#[must_use]
#[inline]
pub fn rod_subtract(a: RodNumeral, b: RodNumeral) -> RodNumeral {
    RodNumeral::new(a.value.wrapping_sub(b.value))
}

/// Multiply two rod numerals.
#[must_use]
#[inline]
pub fn rod_multiply(a: RodNumeral, b: RodNumeral) -> RodNumeral {
    RodNumeral::new(a.value.wrapping_mul(b.value))
}

// ---------------------------------------------------------------------------
// Chinese Remainder Theorem
// ---------------------------------------------------------------------------

/// Solve a system of congruences using the Chinese Remainder Theorem
/// (Sun Tzu's algorithm).
///
/// Given a list of `(remainder, modulus)` pairs, finds the smallest
/// non-negative integer x such that x = remainder_i (mod modulus_i)
/// for all i.
///
/// From the Sunzi Suanjing (c. 3rd century CE):
/// "There are certain things whose number is unknown. If we count them
/// by threes, we have two left over; by fives, three left over; by sevens,
/// two left over. How many things are there?" Answer: 23.
///
/// # Errors
///
/// Returns `None` if the moduli are not pairwise coprime, if any modulus
/// is zero, or if the input is empty.
#[must_use]
pub fn chinese_remainder(residues: &[(u64, u64)]) -> Option<u64> {
    if residues.is_empty() {
        return None;
    }

    // Check for zero moduli
    if residues.iter().any(|&(_, m)| m == 0) {
        return None;
    }

    // Single congruence
    if residues.len() == 1 {
        let (r, m) = residues[0];
        return Some(r % m);
    }

    // Compute product of all moduli
    let mut product: u128 = 1;
    for &(_, m) in residues {
        product = product.checked_mul(u128::from(m))?;
    }

    let mut sum: u128 = 0;

    for &(remainder, modulus) in residues {
        let m = u128::from(modulus);
        let r = u128::from(remainder);
        let p = product / m; // product of all other moduli

        // Find modular inverse of p mod m using extended Euclidean algorithm
        let inv = mod_inverse(p % m, m)?;

        sum = (sum + r * p % product * inv % product) % product;
    }

    // Convert back to u64 if it fits
    u64::try_from(sum % product).ok()
}

/// Compute modular multiplicative inverse of a mod m using extended
/// Euclidean algorithm. Returns None if gcd(a, m) != 1.
fn mod_inverse(a: u128, m: u128) -> Option<u128> {
    if m == 1 {
        return Some(0);
    }

    let (mut old_r, mut r) = (a as i128, m as i128);
    let (mut old_s, mut s) = (1i128, 0i128);

    while r != 0 {
        let quotient = old_r / r;
        let temp_r = r;
        r = old_r - quotient * r;
        old_r = temp_r;

        let temp_s = s;
        s = old_s - quotient * s;
        old_s = temp_s;
    }

    // GCD must be 1
    if old_r != 1 {
        return None;
    }

    // Ensure positive result
    let result = ((old_s % m as i128) + m as i128) % m as i128;
    Some(result as u128)
}

// ---------------------------------------------------------------------------
// Magic squares
// ---------------------------------------------------------------------------

/// Generate a magic square of order n.
///
/// - n=3: Returns the Lo Shu magic square (one of the oldest known magic
///   squares, from Chinese legend, c. 650 BCE). Every row, column, and
///   diagonal sums to 15.
/// - Odd n >= 3: Uses the Siamese method (de la Loubere's method),
///   which was known in various forms across Asia.
///
/// Returns `None` for n < 3 or even n (even magic squares require different
/// algorithms not covered here).
///
/// The returned grid is row-major: `result[row][col]`.
#[must_use]
pub fn magic_square(n: usize) -> Option<Vec<Vec<u64>>> {
    if n < 3 || n.is_multiple_of(2) {
        return None;
    }

    // Special case: Lo Shu (the historical magic square)
    if n == 3 {
        return Some(vec![vec![2, 7, 6], vec![9, 5, 1], vec![4, 3, 8]]);
    }

    // Siamese method for odd n
    let mut square = vec![vec![0u64; n]; n];

    // Start at top-center
    let mut row = 0;
    let mut col = n / 2;

    for num in 1..=((n * n) as u64) {
        square[row][col] = num;

        // Move up-right
        let new_row = if row == 0 { n - 1 } else { row - 1 };
        let new_col = (col + 1) % n;

        if square[new_row][new_col] != 0 {
            // Cell occupied, move down instead
            row = (row + 1) % n;
        } else {
            row = new_row;
            col = new_col;
        }
    }

    Some(square)
}

/// Verify that a square is magic (all rows, columns, and diagonals sum equally).
#[must_use]
pub fn is_magic_square(square: &[Vec<u64>]) -> bool {
    let n = square.len();
    if n == 0 {
        return false;
    }
    if square.iter().any(|row| row.len() != n) {
        return false;
    }

    let magic_sum: u64 = square[0].iter().sum();

    // Check all rows
    for row in square {
        let s: u64 = row.iter().sum();
        if s != magic_sum {
            return false;
        }
    }

    // Check all columns
    for col in 0..n {
        let s: u64 = square.iter().map(|row| row[col]).sum();
        if s != magic_sum {
            return false;
        }
    }

    // Check main diagonal
    let s: u64 = (0..n).map(|i| square[i][i]).sum();
    if s != magic_sum {
        return false;
    }

    // Check anti-diagonal
    let s: u64 = (0..n).map(|i| square[i][n - 1 - i]).sum();
    if s != magic_sum {
        return false;
    }

    true
}

// ---------------------------------------------------------------------------
// Sexagenary cycle (Heavenly Stems + Earthly Branches)
// ---------------------------------------------------------------------------

/// The 10 Heavenly Stems (Tiangan 天干) of the Chinese sexagenary cycle.
///
/// Combined with the 12 Earthly Branches, they produce a 60-year cycle
/// used for year naming since at least the Shang dynasty (c. 1250 BCE).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum HeavenlyStem {
    /// 甲 Jiǎ (Wood, Yang)
    Jia,
    /// 乙 Yǐ (Wood, Yin)
    Yi,
    /// 丙 Bǐng (Fire, Yang)
    Bing,
    /// 丁 Dīng (Fire, Yin)
    Ding,
    /// 戊 Wù (Earth, Yang)
    Wu,
    /// 己 Jǐ (Earth, Yin)
    Ji,
    /// 庚 Gēng (Metal, Yang)
    Geng,
    /// 辛 Xīn (Metal, Yin)
    Xin,
    /// 壬 Rén (Water, Yang)
    Ren,
    /// 癸 Guǐ (Water, Yin)
    Gui,
}

const HEAVENLY_STEMS: [HeavenlyStem; 10] = [
    HeavenlyStem::Jia,
    HeavenlyStem::Yi,
    HeavenlyStem::Bing,
    HeavenlyStem::Ding,
    HeavenlyStem::Wu,
    HeavenlyStem::Ji,
    HeavenlyStem::Geng,
    HeavenlyStem::Xin,
    HeavenlyStem::Ren,
    HeavenlyStem::Gui,
];

impl core::fmt::Display for HeavenlyStem {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = match self {
            Self::Jia => "Jiǎ (甲)",
            Self::Yi => "Yǐ (乙)",
            Self::Bing => "Bǐng (丙)",
            Self::Ding => "Dīng (丁)",
            Self::Wu => "Wù (戊)",
            Self::Ji => "Jǐ (己)",
            Self::Geng => "Gēng (庚)",
            Self::Xin => "Xīn (辛)",
            Self::Ren => "Rén (壬)",
            Self::Gui => "Guǐ (癸)",
        };
        write!(f, "{name}")
    }
}

/// The 12 Earthly Branches (Dizhi 地支) of the Chinese sexagenary cycle.
///
/// Each branch is associated with a zodiac animal and a two-hour period
/// of the day. Combined with the 10 Heavenly Stems to produce the
/// 60-unit sexagenary cycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum EarthlyBranch {
    /// 子 Zǐ (Rat)
    Zi,
    /// 丑 Chǒu (Ox)
    Chou,
    /// 寅 Yín (Tiger)
    Yin,
    /// 卯 Mǎo (Rabbit)
    Mao,
    /// 辰 Chén (Dragon)
    Chen,
    /// 巳 Sì (Snake)
    Si,
    /// 午 Wǔ (Horse)
    Wu,
    /// 未 Wèi (Goat)
    Wei,
    /// 申 Shēn (Monkey)
    Shen,
    /// 酉 Yǒu (Rooster)
    You,
    /// 戌 Xū (Dog)
    Xu,
    /// 亥 Hài (Pig)
    Hai,
}

const EARTHLY_BRANCHES: [EarthlyBranch; 12] = [
    EarthlyBranch::Zi,
    EarthlyBranch::Chou,
    EarthlyBranch::Yin,
    EarthlyBranch::Mao,
    EarthlyBranch::Chen,
    EarthlyBranch::Si,
    EarthlyBranch::Wu,
    EarthlyBranch::Wei,
    EarthlyBranch::Shen,
    EarthlyBranch::You,
    EarthlyBranch::Xu,
    EarthlyBranch::Hai,
];

/// The zodiac animal names for each Earthly Branch.
const ZODIAC_ANIMALS: [&str; 12] = [
    "Rat", "Ox", "Tiger", "Rabbit", "Dragon", "Snake", "Horse", "Goat", "Monkey", "Rooster", "Dog",
    "Pig",
];

impl core::fmt::Display for EarthlyBranch {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let idx = EARTHLY_BRANCHES
            .iter()
            .position(|&b| b == *self)
            .unwrap_or(0);
        let name = match self {
            Self::Zi => "Zǐ (子)",
            Self::Chou => "Chǒu (丑)",
            Self::Yin => "Yín (寅)",
            Self::Mao => "Mǎo (卯)",
            Self::Chen => "Chén (辰)",
            Self::Si => "Sì (巳)",
            Self::Wu => "Wǔ (午)",
            Self::Wei => "Wèi (未)",
            Self::Shen => "Shēn (申)",
            Self::You => "Yǒu (酉)",
            Self::Xu => "Xū (戌)",
            Self::Hai => "Hài (亥)",
        };
        write!(f, "{name} — {}", ZODIAC_ANIMALS[idx])
    }
}

/// A year in the Chinese sexagenary (60-year) cycle.
///
/// Each year is identified by a Heavenly Stem + Earthly Branch pair.
/// The cycle repeats every lcm(10, 12) = 60 years.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SexagenaryYear {
    /// The Heavenly Stem for this year.
    pub stem: HeavenlyStem,
    /// The Earthly Branch (zodiac animal) for this year.
    pub branch: EarthlyBranch,
    /// Position in the 60-year cycle (1–60).
    pub cycle_position: u8,
}

impl core::fmt::Display for SexagenaryYear {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let branch_idx = EARTHLY_BRANCHES
            .iter()
            .position(|&b| b == self.branch)
            .unwrap_or(0);
        write!(
            f,
            "{:?}-{:?} ({}, {})",
            self.stem, self.branch, ZODIAC_ANIMALS[branch_idx], self.cycle_position
        )
    }
}

/// Compute the sexagenary year for a given Gregorian/Julian year.
///
/// The cycle is anchored so that 4 CE = Jiǎ-Zǐ (position 1), the
/// traditional start of the current cycle era.
///
/// Negative years represent BCE: year 0 = 1 BCE, year -1 = 2 BCE.
#[must_use]
pub fn sexagenary_from_year(year: i64) -> SexagenaryYear {
    // 4 CE = Jia-Zi (stem 0, branch 0, position 1)
    // So offset = year - 4, mod 60
    let offset = (year - 4).rem_euclid(60);

    let stem_idx = offset.rem_euclid(10) as usize;
    let branch_idx = offset.rem_euclid(12) as usize;
    let cycle_position = (offset % 60 + 1) as u8;

    SexagenaryYear {
        stem: HEAVENLY_STEMS[stem_idx],
        branch: EARTHLY_BRANCHES[branch_idx],
        cycle_position,
    }
}

/// Compute the sexagenary year for a given Julian Day Number.
///
/// Converts the JDN to an approximate Gregorian year, then computes
/// the sexagenary year from that.
#[must_use]
pub fn sexagenary_from_jdn(jdn: f64) -> SexagenaryYear {
    let greg = crate::gregorian::jdn_to_gregorian(jdn);
    sexagenary_from_year(greg.year)
}

// ---------------------------------------------------------------------------
// Unicode rod numeral display (requires varna)
// ---------------------------------------------------------------------------

/// Render a positive number using Unicode counting rod numeral characters.
///
/// Uses the CJK Counting Rod Numerals block (U+1D360–U+1D371) from varna's
/// Chinese rod numeral system. These are the vertical forms (𝍠=1 through 𝍨=9).
/// Zero positions are shown as a space.
///
/// Requires the `varna` feature.
///
/// # Errors
///
/// Returns [`crate::SankhyaError::InvalidBase`] if `n` is zero (rod numerals have
/// no zero representation — an empty space on the counting board).
#[cfg(feature = "varna")]
#[must_use = "returns the Unicode rod numeral string without side effects"]
pub fn to_unicode_rods(n: u64) -> crate::error::Result<String> {
    if n == 0 {
        return Err(crate::error::SankhyaError::InvalidBase(
            "zero has no rod numeral representation".into(),
        ));
    }

    let system = varna::script::numerals::chinese_rod_numerals();
    let mut digits = Vec::new();
    let mut remaining = n;

    while remaining > 0 {
        let d = (remaining % 10) as u32;
        if d == 0 {
            digits.push(" ".to_string());
        } else if let Some(ch) = system.char_for(d) {
            digits.push(ch.to_string());
        }
        remaining /= 10;
    }

    digits.reverse();
    Ok(digits.join(""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rod_arithmetic() {
        let a = RodNumeral::new(42);
        let b = RodNumeral::new(17);
        assert_eq!(rod_add(a, b).value, 59);
        assert_eq!(rod_subtract(a, b).value, 25);
        assert_eq!(rod_multiply(a, b).value, 714);
    }

    #[test]
    fn crt_sun_tzu_problem() {
        // "count by 3 remainder 2, by 5 remainder 3, by 7 remainder 2"
        let result = chinese_remainder(&[(2, 3), (3, 5), (2, 7)]);
        assert_eq!(result, Some(23));
    }

    #[test]
    fn crt_single() {
        assert_eq!(chinese_remainder(&[(3, 7)]), Some(3));
    }

    #[test]
    fn crt_empty() {
        assert_eq!(chinese_remainder(&[]), None);
    }

    #[test]
    fn lo_shu_magic_square() {
        let sq = magic_square(3).unwrap();
        assert!(is_magic_square(&sq));
        // Magic constant for 3x3 is 15
        let sum: u64 = sq[0].iter().sum();
        assert_eq!(sum, 15);
    }

    #[test]
    fn magic_square_5x5() {
        let sq = magic_square(5).unwrap();
        assert!(is_magic_square(&sq));
        // Magic constant for 5x5 is 65
        let sum: u64 = sq[0].iter().sum();
        assert_eq!(sum, 65);
    }

    #[test]
    fn magic_square_even_returns_none() {
        assert!(magic_square(4).is_none());
    }

    // -- Sexagenary cycle --

    #[test]
    fn sexagenary_2024_wood_dragon() {
        let s = sexagenary_from_year(2024);
        assert_eq!(s.stem, HeavenlyStem::Jia);
        assert_eq!(s.branch, EarthlyBranch::Chen); // Dragon
        assert_eq!(s.cycle_position, 41);
    }

    #[test]
    fn sexagenary_2025_wood_snake() {
        let s = sexagenary_from_year(2025);
        assert_eq!(s.stem, HeavenlyStem::Yi);
        assert_eq!(s.branch, EarthlyBranch::Si); // Snake
    }

    #[test]
    fn sexagenary_4_ce_jia_zi() {
        // 4 CE = Jia-Zi, position 1 (cycle anchor)
        let s = sexagenary_from_year(4);
        assert_eq!(s.stem, HeavenlyStem::Jia);
        assert_eq!(s.branch, EarthlyBranch::Zi);
        assert_eq!(s.cycle_position, 1);
    }

    #[test]
    fn sexagenary_60_year_cycle() {
        // Same stem-branch pair every 60 years
        let s1 = sexagenary_from_year(2024);
        let s2 = sexagenary_from_year(2024 + 60);
        assert_eq!(s1.stem, s2.stem);
        assert_eq!(s1.branch, s2.branch);
    }

    #[test]
    fn sexagenary_all_60_unique() {
        let mut pairs = std::collections::HashSet::new();
        for i in 0..60 {
            let s = sexagenary_from_year(4 + i);
            pairs.insert((format!("{:?}", s.stem), format!("{:?}", s.branch)));
        }
        assert_eq!(pairs.len(), 60);
    }

    #[test]
    fn sexagenary_from_jdn_matches_year() {
        // JDN 2460676.5 = Jan 1, 2025
        let s = sexagenary_from_jdn(2_460_676.5);
        let s2 = sexagenary_from_year(2025);
        assert_eq!(s, s2);
    }

    #[test]
    fn sexagenary_serde_roundtrip() {
        let s = sexagenary_from_year(2024);
        let json = serde_json::to_string(&s).unwrap();
        let back: SexagenaryYear = serde_json::from_str(&json).unwrap();
        assert_eq!(s, back);
    }

    #[test]
    fn stem_display() {
        assert_eq!(HeavenlyStem::Jia.to_string(), "Jiǎ (甲)");
        assert_eq!(HeavenlyStem::Gui.to_string(), "Guǐ (癸)");
    }

    #[test]
    fn branch_display() {
        let d = EarthlyBranch::Chen.to_string();
        assert!(d.contains("Dragon"));
        assert!(d.contains("辰"));
    }

    #[cfg(feature = "varna")]
    mod unicode_rod_tests {
        use super::*;

        #[test]
        fn unicode_rods_single_digit() {
            assert_eq!(to_unicode_rods(1).unwrap(), "𝍠");
            assert_eq!(to_unicode_rods(9).unwrap(), "𝍨");
        }

        #[test]
        fn unicode_rods_multi_digit() {
            // 42 = 4, 2
            let s = to_unicode_rods(42).unwrap();
            assert_eq!(s, "𝍣𝍡");
        }

        #[test]
        fn unicode_rods_with_zero() {
            // 101 = 1, 0, 1
            let s = to_unicode_rods(101).unwrap();
            assert_eq!(s, "𝍠 𝍠");
        }

        #[test]
        fn unicode_rods_zero_errors() {
            assert!(to_unicode_rods(0).is_err());
        }
    }
}
