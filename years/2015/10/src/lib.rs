fn process(mut input: &str) -> eyre::Result<String> {
    input = input.trim();

    let mut count = 1;
    let mut out = String::with_capacity(input.len());
    let mut current = input
        .chars()
        .nth(0)
        .ok_or(eyre::format_err!("empty input"))?;

    for (i, c) in input.chars().enumerate().skip(1) {
        if current != c || i == input.len() - 1 {
            out += &format!("{}{}", count, current);
            count = 1;
            current = c;
        } else {
            count += 1;
        }
    }

    out += &format!("{}{}", count, current);

    Ok(out)
}

pub fn part1(input: &str) -> eyre::Result<usize> {
    let mut out = input.to_owned();
    for _ in 0..40 {
        out = process(&out)?;
    }

    Ok(out.len())
}
pub fn part2(input: &str) -> eyre::Result<usize> {
    let mut out = input.to_owned();
    for _ in 0..50 {
        out = process(&out)?;
    }

    Ok(out.len())
}
