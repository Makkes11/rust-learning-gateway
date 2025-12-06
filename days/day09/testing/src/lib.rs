pub fn double(v: i32) -> i32 {
    v * 2
}

pub fn parse_number(input: &str) -> Result<i32, String> {
    input
        .trim()
        .parse::<i32>()
        .map_err(|_| "Parse error".to_string())
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_double_2_value() {
        assert_eq!(double(2), 4);
    }

    #[test]
    fn test_double_minus_2_should_be_4() {
        assert_eq!(double(-2), -4);
    }

    #[test]
    fn test_double_0_should_be_0() {
        assert_eq!(double(0), 0);
    }

    #[test]
    fn test_parse_number_42_should_succeed() {
        assert_eq!(parse_number("42"), Ok(42));
    }

    #[test]
    fn test_parse_number_5_should_succeed() {
        assert_eq!(parse_number(" 5 "), Ok(5));
    }

    #[test]
    fn test_parse_number_abc_should_fail() {
        assert_eq!(parse_number("abc"), Err("Parse error".to_string()));
    }
}
