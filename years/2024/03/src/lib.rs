#[derive(Debug)]
enum Expression {
    Do,
    Dont,
    Mul(usize, usize),
}

type Expressions = Vec<Expression>;

fn parse(input: &str) -> eyre::Result<Expressions> {
    let regex = regex::RegexBuilder::new(r"mul\((\d+,\d+)\)|(don't\(\))|(do\(\))").build()?;
    let mut out = Expressions::new();
    for (_, [a]) in regex.captures_iter(input).map(|c| c.extract()) {
        match a {
            "do()" => out.push(Expression::Do),
            "don't()" => out.push(Expression::Dont),
            _ => {
                let mut a = a.split(",");
                let first = a.next().unwrap().parse::<usize>()?;
                let second = a.next().unwrap().parse::<usize>()?;
                out.push(Expression::Mul(first, second))
            }
        }
    }

    Ok(out)
}

pub fn part1(input: &str) -> eyre::Result<usize> {
    let expressions = parse(input)?;
    Ok(expressions.iter().fold(0, |acc, current| match current {
        Expression::Mul(a, b) => acc + a * b,
        _ => acc,
    }))
}
pub fn part2(input: &str) -> eyre::Result<usize> {
    let expressions = parse(input)?;

    let mut enabled = true;

    Ok(expressions.iter().fold(0, |acc, current| match current {
        Expression::Mul(a, b) if enabled => acc + a * b,
        Expression::Do => {
            enabled = true;
            acc
        }
        Expression::Dont => {
            enabled = false;
            acc
        }
        _ => acc,
    }))
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_works() -> eyre::Result<()> {
        const INPUT: &str =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(super::part1(INPUT)?, 161);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        const INPUT: &str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(super::part2(INPUT)?, 48);
        Ok(())
    }
}
