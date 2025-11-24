use itertools::Itertools;

fn increment(mut input: &str) -> eyre::Result<String> {
    input = input.trim();
    let mut chars = input.chars().collect_vec();

    let mut index = chars.len() - 1;

    loop {
        let current = chars[index];
        let next =
            char::from_u32(current as u32 + 1).ok_or(eyre::format_err!("increment failed"))?;

        match next {
            'a'..='z' => {
                chars[index] = next;
                break;
            }
            // if we're outside, we need to loop
            _ => {
                if index == 0 {
                    // if we're at the start of the string, add an 'a' to the start and continue
                    chars[index] = 'a';
                    chars.insert(0, 'a');
                    break;
                } else {
                    // otherwise, reset the current, and move back one
                    chars[index] = 'a';
                    index -= 1;
                }
            }
        };
    }

    Ok(String::from_iter(chars))
}

fn is_allowed(mut input: &str) -> bool {
    input = input.trim();

    const DISALLOWED: [char; 3] = ['l', 'o', 'i'];
    let has_disallowed = input.chars().any(|c| DISALLOWED.contains(&c));
    if has_disallowed {
        return false;
    }

    let has_increasing_sequence = input
        .chars()
        .map(|c| c as u32)
        .tuple_windows()
        .any(|(a, b, c)| c.saturating_sub(b) == 1 && b.saturating_sub(a) == 1);
    if !has_increasing_sequence {
        return false;
    }

    let pair_locations = input
        .char_indices()
        .tuple_windows() // go by pairs of characters
        .filter_map(|(a, b)| (a.1 == b.1).then_some(a)) // filter by matching pairs, aa, bb etc.
        .unique_by(|p| p.1)
        .collect_vec(); // find unique pairs

    let has_non_overlapping_pairs =
        itertools::iproduct!(&pair_locations, &pair_locations).any(|(a, b)| b.0.abs_diff(a.0) > 2);

    if !has_non_overlapping_pairs {
        return false;
    }

    return true;
}

fn next_password(input: &str) -> eyre::Result<String> {
    let mut result = input.to_owned();
    loop {
        result = increment(&result)?;
        if is_allowed(&result) {
            break;
        }
    }

    Ok(result)
}

pub fn part1(input: &str) -> eyre::Result<String> {
    next_password(input)
}
pub fn part2(input: &str) -> eyre::Result<String> {
    let mut pass = next_password(input)?;
    pass = next_password(&pass)?;

    Ok(pass)
}

#[cfg(test)]
mod tests {
    #[test]
    fn increment_works() -> eyre::Result<()> {
        assert_eq!(super::increment("a")?, "b");
        assert_eq!(super::increment("az")?, "ba");
        assert_eq!(super::increment("aaz")?, "aba");
        assert_eq!(super::increment("aac")?, "aad");
        assert_eq!(super::increment("zz")?, "aaa");
        assert_eq!(super::increment("abcdfezz")?, "abcdffaa");

        let mut s = "aaa".to_string();
        for _ in 0..26 {
            s = super::increment(&s)?;
        }
        assert_eq!(s, "aba");

        Ok(())
    }

    #[test]
    fn is_allowed_works() {
        assert!(super::is_allowed("ghjaabcc"));
    }
}
