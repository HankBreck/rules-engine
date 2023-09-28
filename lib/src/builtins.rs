use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::time::Duration;
use chrono::{DateTime, Utc};

// TODO: Remove this once utils are ported
mod utils {
    // Define your utility functions here
    // Example: parse_datetime, parse_float, parse_timedelta
}
use crate::utils::*;

fn builtin_filter<F, T>(function: F, iterable: Vec<T>) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    iterable.into_iter().filter(|x| function(x)).collect()
}

fn builtin_map<F, T, R>(function: F, iterable: Vec<T>) -> Vec<R>
where
    F: Fn(T) -> R,
{
    iterable.into_iter().map(|x| function(x)).collect()
}

fn builtin_parse_datetime(string: &str, timezone: &chrono::FixedOffset) -> DateTime<Utc> {
    parse_datetime(string, timezone)
}

fn builtin_random(boundary: Option<u32>) -> u32 {
    match boundary {
        Some(b) => rand::random::<u32>() % (b + 1),
        None => rand::random(),
    }
}

fn builtins_split(string: &str, sep: Option<&str>, maxsplit: Option<usize>) -> Vec<&str> {
    if let Some(sep) = sep {
        string.splitn(maxsplit.unwrap_or(usize::MAX), sep).collect()
    } else {
        string.split_whitespace().collect()
    }
}

struct BuiltinValueGenerator<F>
where
    F: Fn(&Builtins) -> DateTime<Utc>,
{
    callable: F,
}

impl<F> BuiltinValueGenerator<F>
where
    F: Fn(&Builtins) -> DateTime<Utc>,
{
    fn call(&self, builtins: &Builtins) -> DateTime<Utc> {
        (self.callable)(builtins)
    }
}

struct Builtins<'a> {
    values: HashMap<&'a str, Value>,
    value_types: HashMap<&'a str, DataType>,
    namespace: Option<&'a str>,
    timezone: chrono::FixedOffset,
}

impl<'a> Builtins<'a> {
    fn resolve_type(&self, name: &'a str) -> DataType {
        self.value_types.get(name).cloned().unwrap_or(DataType::Undefined)
    }

    fn new(values: HashMap<&'a str, Value>, namespace: Option<&'a str>, timezone: chrono::FixedOffset) -> Builtins<'a> {
        let value_types = HashMap::new(); // Add types as needed
        Builtins {
            values,
            value_types,
            namespace,
            timezone,
        }
    }

    fn from_defaults() -> Builtins<'a> {
        let now = BuiltinValueGenerator(|builtins| Utc::now().with_timezone(&builtins.timezone));
        // TODO: Define your default values here

        // // Example:
        // let default_values: HashMap<&str, Value> = ...;

        // TODO: Define your default types here

        // // Example:
        // let default_value_types: HashMap<&str, DataType> = ...;

        Builtins {
            values: default_values,
            value_types: default_value_types,
            namespace: None,
            timezone: chrono::FixedOffset::west(0), // Set default timezone here
        }
    }
}
