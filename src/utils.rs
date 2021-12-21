use std::num::ParseIntError;

pub fn parse_asn(input: &str) -> Result<i64, ParseIntError> {
    input[2..].parse()
}
