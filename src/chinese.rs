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
            let rod_char = if pos_from_right.is_multiple_of(2) { '|' } else { '-' };

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
}
