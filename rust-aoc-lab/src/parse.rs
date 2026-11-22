//! Turning text into numbers and blocks.

/// Split the input on blank lines and drop empty blocks.
///
/// Both `\n\n` and `\r\n\r\n` are accepted, so a file saved on Windows behaves
/// like one saved on Linux. The slices borrow the caller's input.
pub fn blocks(input: &str) -> Vec<&str> {
    let separator = if input.contains("\r\n\r\n") {
        "\r\n\r\n"
    } else {
        "\n\n"
    };

    input
        .split(separator)
        .map(str::trim)
        .filter(|block| !block.is_empty())
        .collect()
}

/// Read one unsigned decimal number from a byte slice.
///
/// The slice must contain only ASCII digits.
pub fn atoi(bytes: &[u8]) -> u64 {
    let mut value: u64 = 0;
    for &byte in bytes {
        value = value * 10 + (byte - b'0') as u64;
    }
    value
}

/// Pull every unsigned integer out of the text, in order.
///
/// A `-` is treated as a separator, not a sign. Use [`signed`] when negative
/// numbers matter.
pub fn unsigned(input: &str) -> Vec<u64> {
    let bytes = input.as_bytes();
    let mut numbers = Vec::new();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i].is_ascii_digit() {
            let start = i;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            numbers.push(atoi(&bytes[start..i]));
        } else {
            i += 1;
        }
    }

    numbers
}

/// Pull every signed integer out of the text, in order.
pub fn signed(input: &str) -> Vec<i64> {
    let bytes = input.as_bytes();
    let mut numbers = Vec::new();
    let mut i = 0;

    while i < bytes.len() {
        let negative = bytes[i] == b'-' && i + 1 < bytes.len() && bytes[i + 1].is_ascii_digit();
        if negative || bytes[i].is_ascii_digit() {
            if negative {
                i += 1;
            }
            let start = i;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            let value = atoi(&bytes[start..i]) as i64;
            numbers.push(if negative { -value } else { value });
        } else {
            i += 1;
        }
    }

    numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_ignores_empty_blocks() {
        let input = "one\n\n\ntwo\nthree\n\n";
        assert_eq!(blocks(input), vec!["one", "two\nthree"]);
    }

    #[test]
    fn blocks_accepts_windows_line_endings() {
        let input = "one\r\n\r\ntwo\r\nthree\r\n";
        assert_eq!(blocks(input), vec!["one", "two\r\nthree"]);
    }

    #[test]
    fn unsigned_reads_in_order() {
        assert_eq!(unsigned("a 12, b 345; c 6"), vec![12, 345, 6]);
    }

    #[test]
    fn unsigned_does_not_read_a_negative_sign() {
        assert_eq!(unsigned("3-1"), vec![3, 1]);
    }

    #[test]
    fn signed_reads_a_negative_sign() {
        assert_eq!(signed("3 -1 +2"), vec![3, -1, 2]);
    }

    #[test]
    fn atoi_reads_a_number() {
        assert_eq!(atoi(b"2048"), 2048);
    }
}
