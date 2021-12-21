use std::num::ParseIntError;

pub fn parse_asn(input: &str) -> Result<i64, ParseIntError> {
    let asn: u64 = input[2..].parse()?;
    Ok(asn as i64)
}
