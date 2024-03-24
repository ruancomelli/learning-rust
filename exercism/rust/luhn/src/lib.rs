/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let trimmed = code.trim();

    if let 0 | 1 = trimmed.len() {
        return false;
    }

    trimmed
        .chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .zip([false, true].iter().cycle())
        .try_fold(0, |sum, (c, &is_even_position)| {
            c.to_digit(10).map(|digit| {
                sum + if is_even_position {
                    let twice = digit * 2;
                    if twice >= 10 {
                        twice - 9
                    } else {
                        twice
                    }
                } else {
                    digit
                }
            })
        })
        .map_or(false, |sum| sum % 10 == 0)
}
