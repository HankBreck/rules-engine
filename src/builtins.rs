use std::str::FromStr;
use unic_langid::LanguageIdentifier;

use crate::ast::EvalResultTypes;
use crate::errors::{EvaluationError, InvalidParameterTypeError, SymbolResolutionError};

fn as_lower(value: EvalResultTypes) -> Result<EvalResultTypes, EvaluationError> {
    match value {
        EvalResultTypes::String(s) => Ok(EvalResultTypes::String(s.to_lowercase())),
        _ => Err(InvalidParameterTypeError::new("Expected string").into()),
    }
}

fn language_code(eval_result: EvalResultTypes) -> Result<EvalResultTypes, EvaluationError> {
    // TODO: Is there a way to map english -> en?
    match eval_result {
        EvalResultTypes::String(value) => {
            let lang = LanguageIdentifier::from_str(&value).or_else(|_| {
                LanguageIdentifier::from_str(value.split('-').next().unwrap_or_default())
            });
            match lang {
                Ok(lang) => Ok(EvalResultTypes::String(lang.language.to_string())),
                Err(_) => Err(SymbolResolutionError::new(&format!(
                    "Unable to find valid language code for '{}'",
                    value,
                ))
                .into()),
            }
        }
        _ => Err(SymbolResolutionError::new("Expected string").into()),
    }
}

type BuiltinFunc = fn(EvalResultTypes) -> Result<EvalResultTypes, EvaluationError>;
pub fn resolve_builtin_methods(identifier: &str) -> Result<BuiltinFunc, SymbolResolutionError> {
    match identifier {
        "as_lower" => Ok(as_lower),
        "language_code" => Ok(language_code),
        _ => Err(SymbolResolutionError::new(&format!(
            "Builtin method {} not found",
            identifier
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_lower() {
        assert_eq!(
            as_lower(EvalResultTypes::String("Hello".to_string())).unwrap(),
            EvalResultTypes::String("hello".to_string())
        );
        assert!(as_lower(EvalResultTypes::Integer(1)).is_err());
    }

    #[test]
    fn test_language_code() {
        assert_eq!(
            language_code(EvalResultTypes::String("en".to_string())).unwrap(),
            EvalResultTypes::String("en".to_string())
        );
        assert_eq!(
            language_code(EvalResultTypes::String("en-US".to_string())).unwrap(),
            EvalResultTypes::String("en".to_string())
        );
        assert!(language_code(EvalResultTypes::Integer(1)).is_err());
        assert!(language_code(EvalResultTypes::String("".to_string())).is_err());
    }
}
