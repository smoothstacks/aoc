pub fn part1(input: &str) -> eyre::Result<i64> {
    fn visit(value: &serde_json::Value) -> i64 {
        match value {
            serde_json::Value::Number(number) => number.as_i64().unwrap_or_default(),
            serde_json::Value::Array(values) => values.iter().map(visit).sum(),
            serde_json::Value::Object(map) => map.values().map(visit).sum(),
            _ => 0,
        }
    }

    let value: serde_json::Value = serde_json::de::from_str(input)?;
    Ok(visit(&value))
}
pub fn part2(input: &str) -> eyre::Result<i64> {
    fn visit(value: &serde_json::Value) -> i64 {
        match value {
            serde_json::Value::Number(number) => number.as_i64().unwrap_or_default(),
            serde_json::Value::Array(values) => values.iter().map(visit).sum(),
            serde_json::Value::Object(map) => {
                if map
                    .values()
                    .any(|v| matches!(v, serde_json::Value::String(s) if s == "red"))
                {
                    0
                } else {
                    map.values().map(visit).sum()
                }
            }
            _ => 0,
        }
    }

    let value: serde_json::Value = serde_json::de::from_str(input)?;
    Ok(visit(&value))
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"{"red": [1, 2, 3], "green": 6, "test": "red"}"#;

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 12);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 0);
        Ok(())
    }
}
