const LEN_WEIGHT: f32 = 0.9;

/// Edit distance algorithm based on https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance
pub fn edit_distance(s1: &str, s2: &str, len_weight: Option<f32>) -> f32 {
    let s1_len = s1.len();
    let s2_len = s2.len();

    // return early if both strings are empty
    if s1_len == 0 && s2_len == 0 {
        return 0f32;
    }
    if s1_len == 0 {
        return s2_len as f32;
    }
    if s2_len == 0 {
        return s1_len as f32;
    }

    let mut i = 0;
    let mut j = 0;

    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();

    let mut changes = 0;

    while i < s1_len || j < s2_len {
        if i < s1_len && j == s2_len {
            changes += s1_len - i + 1;
            break;
        } else if i == s1_len && j < s2_len {
            changes += s2_len - j + 1;
            break;
        }

        if s1_chars[i] == s2_chars[j] {
            i += 1;
            j += 1;
            continue;
        }
        // s1 and s2 both still two or more additional characters to recurse
        // AND the current and next characters of the two strings are transposed
        let transposed = { || s1_chars[i] == s2_chars[j + 1] && s2_chars[j] == s1_chars[i + 1] };
        if (i + 2 < s1_len && j + 2 < s2_len) && transposed() {
            // skip past the next characters because we already know they will
            // be different. This is handled by the transposition.
            changes += 1;
            i += 2;
            j += 2;
        } else {
            if s1_chars[i] != s2_chars[j] {
                changes += 1;
            }
            i += 1;
            j += 1;
        }
    }
    let len_diff = (s1_len).abs_diff(s2_len);
    let len_weight = len_weight.unwrap_or(LEN_WEIGHT);
    changes as f32 - (len_weight * len_diff as f32)
}

#[cfg(test)]
mod test {
    use super::*;

    fn run_test(s1: &str, s2: &str, expected: f32) {
        assert_eq!(edit_distance(s1, s2, Some(1f32)), expected);
    }

    #[test]
    fn test_empty_strings_return_zero() {
        run_test("", "", 0.0);
    }

    #[test]
    fn test_one_empty_string_returns_the_length_of_the_other() {
        run_test("hello", "", 5.0);
    }

    #[test]
    fn test_equal_strings_have_edit_distance_of_zero() {
        run_test("hello", "hello", 0.0);
    }

    #[test]
    fn swapped_chars_count_as_one() {
        run_test("one", "noe", 1.0);
    }

    #[test]
    fn each_differing_character_is_a_change() {
        run_test("foo", "bar", 3.0);
    }

    #[test]
    fn one_added_letter_has_distance_one() {
        run_test("one", "oned", 1.0);
    }
}
