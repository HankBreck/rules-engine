// Define a sentinel value to specify that something is undefined.
pub struct Undefined;

impl Undefined {
    fn new() -> Self {
        Undefined
    }
}

impl std::fmt::Display for Undefined {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "UNDEFINED")
    }
}

impl std::fmt::Debug for Undefined {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "UNDEFINED")
    }
}

#[derive(Debug)]
pub struct EngineError {
    message: String,
}

impl EngineError {
    fn new(message: &str) -> Self {
        EngineError {
            message: message.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct EvaluationError {
    pub message: String,
}

impl EvaluationError {
    fn new(message: &str) -> Self {
        EvaluationError {
            message: message.to_string(),
        }
    }
}

#[derive(Debug)]
struct SyntaxError {
    message: String,
}

impl SyntaxError {
    fn new(message: &str) -> Self {
        SyntaxError {
            message: message.to_string(),
        }
    }
}

#[derive(Debug)]
struct DatetimeSyntaxError {
    message: String,
    value: String,
}

impl DatetimeSyntaxError {
    fn new(message: &str, value: &str) -> Self {
        DatetimeSyntaxError {
            message: message.to_string(),
            value: value.to_string(),
        }
    }
}

#[derive(Debug)]
struct FloatSyntaxError {
    message: String,
    value: String,
}

impl FloatSyntaxError {
    fn new(message: &str, value: &str) -> Self {
        FloatSyntaxError {
            message: message.to_string(),
            value: value.to_string(),
        }
    }
}

#[derive(Debug)]
struct TimedeltaSyntaxError {
    message: String,
    value: String,
}

impl TimedeltaSyntaxError {
    fn new(message: &str, value: &str) -> Self {
        TimedeltaSyntaxError {
            message: message.to_string(),
            value: value.to_string(),
        }
    }
}

#[derive(Debug)]
struct RegexSyntaxError {
    message: String,
    error: String,
    value: String,
}

impl RegexSyntaxError {
    fn new(message: &str, error: &str, value: &str) -> Self {
        RegexSyntaxError {
            message: message.to_string(),
            error: error.to_string(),
            value: value.to_string(),
        }
    }
}

#[derive(Debug)]
struct RuleSyntaxError {
    message: String,
    token: Option<String>,
}

impl RuleSyntaxError {
    fn new(message: &str, token: Option<&str>) -> Self {
        let token_str = match token {
            Some(t) => t.to_string(),
            None => "EOF".to_string(),
        };
        let message = format!("{} at: {}", message, token_str);
        RuleSyntaxError {
            message,
            token: token.map(|t| t.to_string()),
        }
    }
}

#[derive(Debug)]
struct AttributeResolutionError {
    attribute_name: String,
    object: String,
    thing: Undefined,
    suggestion: Option<String>,
}

impl AttributeResolutionError {
    fn new(attribute_name: &str, object: &str, suggestion: Option<&str>) -> Self {
        AttributeResolutionError {
            attribute_name: attribute_name.to_string(),
            object: object.to_string(),
            thing: Undefined::new(),
            suggestion: suggestion.map(|s| s.to_string()),
        }
    }
}

#[derive(Debug)]
struct AttributeTypeError {
    attribute_name: String,
    object_type: String,
    is_value: String,
    is_type: String,
    expected_type: String,
}

impl AttributeTypeError {
    fn new(
        attribute_name: &str,
        object_type: &str,
        is_value: &str,
        is_type: &str,
        expected_type: &str,
    ) -> Self {
        let message = format!(
            "attribute '{}' resolved to incorrect datatype (is: {}, expected: {})",
            attribute_name, is_type, expected_type
        );
        AttributeTypeError {
            attribute_name: attribute_name.to_string(),
            object_type: object_type.to_string(),
            is_value: is_value.to_string(),
            is_type: is_type.to_string(),
            expected_type: expected_type.to_string(),
        }
    }
}

#[derive(Debug)]
struct LookupError {
    container: String,
    item: String,
}

impl LookupError {
    fn new(container: &str, item: &str) -> Self {
        LookupError {
            container: container.to_string(),
            item: item.to_string(),
        }
    }
}

#[derive(Debug)]
struct SymbolResolutionError {
    symbol_name: String,
    symbol_scope: Option<String>,
    thing: Undefined,
    suggestion: Option<String>,
}

impl SymbolResolutionError {
    fn new(symbol_name: &str, symbol_scope: Option<&str>, suggestion: Option<&str>) -> Self {
        SymbolResolutionError {
            symbol_name: symbol_name.to_string(),
            symbol_scope: symbol_scope.map(|s| s.to_string()),
            thing: Undefined::new(),
            suggestion: suggestion.map(|s| s.to_string()),
        }
    }
}

#[derive(Debug)]
struct SymbolTypeError {
    symbol_name: String,
    is_value: String,
    is_type: String,
    expected_type: String,
}

impl SymbolTypeError {
    fn new(symbol_name: &str, is_value: &str, is_type: &str, expected_type: &str) -> Self {
        let message = format!(
            "symbol '{}' resolved to incorrect datatype (is: {}, expected: {})",
            symbol_name, is_type, expected_type
        );
        SymbolTypeError {
            symbol_name: symbol_name.to_string(),
            is_value: is_value.to_string(),
            is_type: is_type.to_string(),
            expected_type: expected_type.to_string(),
        }
    }
}

#[derive(Debug)]
struct FunctionCallError {
    message: String,
    error: Option<String>,
    function_name: Option<String>,
}

impl FunctionCallError {
    fn new(message: &str, error: Option<&str>, function_name: Option<&str>) -> Self {
        FunctionCallError {
            message: message.to_string(),
            error: error.map(|e| e.to_string()),
            function_name: function_name.map(|f| f.to_string()),
        }
    }
}