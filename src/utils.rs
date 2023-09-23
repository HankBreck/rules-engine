use chrono::{NaiveDateTime, Duration};
use regex::Regex;

use crate::errors::{DatetimeSyntaxError, FloatSyntaxError, TimedeltaSyntaxError};

lazy_static::lazy_static! {
    static ref SUB_REGEX: Regex = Regex::new(r"[0-9]+([,.][0-9]+)?").unwrap();
    static ref TIMDELTA_REGEX: Regex = Regex::new(
        r"P(?!\b)(?P<weeks>"
            .to_string() + &SUB_REGEX.to_string() + r"W)?(?P<days>"
            .to_string() + &SUB_REGEX.to_string() + r"D)?(T(?P<hours>"
            .to_string() + &SUB_REGEX.to_string() + r"H)?(?P<minutes>"
            .to_string() + &SUB_REGEX.to_string() + r"M)?(?P<seconds>"
            .to_string() + &SUB_REGEX.to_string() + r"S)?)?"
    ).unwrap();
}

fn parse_datetime(string: &str, default_timezone: &chrono::FixedOffset) -> Result<NaiveDateTime, DatetimeSyntaxError> {
    let dt = match NaiveDateTime::parse_from_str(string, "%+") {
        Ok(dt) => dt,
        Err(_) => return Err(DatetimeSyntaxError::new("invalid datetime", string)),
    };
    Ok(match dt.timezone() {
        Some(_) => dt,
        None => dt.with_timezone(default_timezone),
    })
}

fn parse_float(string: &str) -> Result<rust_decimal::Decimal, FloatSyntaxError> {
    if let Some(c) = string.chars().nth(0) {
        if c == '0' && !string.chars().nth(1).map_or(false, |c| c.is_digit(10)) {
            return Err(FloatSyntaxError::new("invalid floating point literal (leading zeros in decimal literals are not permitted)", string));
        }
    }

    let val = match string.chars().nth(1) {
        Some('b') | Some('o') | Some('x') => match py_literal_eval(string) {
            Ok(val) => val,
            Err(_) => return Err(FloatSyntaxError::new("invalid floating point literal", string)),
        },
        _ => match rust_decimal::Decimal::from_str(string) {
            Ok(val) => val,
            Err(_) => return Err(FloatSyntaxError::new("invalid floating point literal", string)),
        },
    };
    Ok(val)
}

fn parse_timedelta(periodstring: &str) -> Result<Duration, TimedeltaSyntaxError> {
    if periodstring == "P" {
        return Err(TimedeltaSyntaxError::new("empty timedelta string", periodstring));
    }

    if let Some(caps) = TIMDELTA_REGEX.captures(periodstring) {
        let groups = caps
            .iter_named()
            .map(|(name, val)| (name.to_string(), val.unwrap_or("0n").as_str()))
            .collect::<std::collections::HashMap<_, _>>();
        
        let weeks = groups.get("weeks").unwrap_or(&"0n").replace(',', ".").parse::<f64>().unwrap();
        let days = groups.get("days").unwrap_or(&"0n").replace(',', ".").parse::<f64>().unwrap();
        let hours = groups.get("hours").unwrap_or(&"0n").replace(',', ".").parse::<f64>().unwrap();
        let minutes = groups.get("minutes").unwrap_or(&"0n").replace(',', ".").parse::<f64>().unwrap();
        let seconds = groups.get("seconds").unwrap_or(&"0n").replace(',', ".").parse::<f64>().unwrap();

        Ok(Duration::weeks(weeks as i64) +
            Duration::days(days as i64) +
            Duration::hours(hours as i64) +
            Duration::minutes(minutes as i64) +
            Duration::seconds(seconds as i64)
        )
    } else {
        Err(TimedeltaSyntaxError::new("invalid timedelta string", periodstring))
    }
}

// Mocking the behavior of py_literal_eval for demonstration purposes
fn py_literal_eval(string: &str) -> Result<rust_decimal::Decimal, rust_decimal::Error> {
    // Implement your logic for py_literal_eval here
    // This is a placeholder
    rust_decimal::Decimal::from_str(string)
}
