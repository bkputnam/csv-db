use std::fmt;

/// The set of types a column (and its values) can have.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataType {
    Int,
    Float,
    Text,
    Bool,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::Int => write!(f, "INT"),
            DataType::Float => write!(f, "FLOAT"),
            DataType::Text => write!(f, "TEXT"),
            DataType::Bool => write!(f, "BOOL"),
        }
    }
}

/// A typed value stored in a row.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Text(String),
    Bool(bool),
    Null,
}

impl Value {
    /// Returns the `DataType` of this value, or `None` for `Null`.
    pub fn data_type(&self) -> Option<DataType> {
        match self {
            Value::Int(_) => Some(DataType::Int),
            Value::Float(_) => Some(DataType::Float),
            Value::Text(_) => Some(DataType::Text),
            Value::Bool(_) => Some(DataType::Bool),
            Value::Null => None,
        }
    }

    /// Returns `true` if this value is compatible with the given column type.
    /// `Null` is compatible with every type (nullability is enforced by the schema).
    pub fn matches_type(&self, data_type: &DataType) -> bool {
        match (self, data_type) {
            (Value::Int(_), DataType::Int) => true,
            (Value::Float(_), DataType::Float) => true,
            (Value::Text(_), DataType::Text) => true,
            (Value::Bool(_), DataType::Bool) => true,
            (Value::Null, _) => true,
            _ => false,
        }
    }

    /// Returns `true` if this value is `Null`.
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// A human-readable name for the type of this value (for error messages).
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Int(_) => "INT",
            Value::Float(_) => "FLOAT",
            Value::Text(_) => "TEXT",
            Value::Bool(_) => "BOOL",
            Value::Null => "NULL",
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{n}"),
            Value::Float(n) => write!(f, "{n}"),
            Value::Text(s) => write!(f, "{s}"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Null => write!(f, "NULL"),
        }
    }
}

// Convenience conversions so callers can write `42.into()` instead of `Value::Int(42)`.
impl From<i64> for Value {
    fn from(n: i64) -> Self {
        Value::Int(n)
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::Float(n)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::Text(s)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::Text(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_matches_its_own_type() {
        assert!(Value::Int(1).matches_type(&DataType::Int));
        assert!(Value::Float(1.0).matches_type(&DataType::Float));
        assert!(Value::Text("hi".into()).matches_type(&DataType::Text));
        assert!(Value::Bool(true).matches_type(&DataType::Bool));
    }

    #[test]
    fn value_does_not_match_wrong_type() {
        assert!(!Value::Int(1).matches_type(&DataType::Float));
        assert!(!Value::Float(1.0).matches_type(&DataType::Int));
        assert!(!Value::Text("hi".into()).matches_type(&DataType::Bool));
        assert!(!Value::Bool(true).matches_type(&DataType::Text));
    }

    #[test]
    fn null_matches_any_type() {
        assert!(Value::Null.matches_type(&DataType::Int));
        assert!(Value::Null.matches_type(&DataType::Float));
        assert!(Value::Null.matches_type(&DataType::Text));
        assert!(Value::Null.matches_type(&DataType::Bool));
    }

    #[test]
    fn value_data_type_round_trips() {
        assert_eq!(Value::Int(0).data_type(), Some(DataType::Int));
        assert_eq!(Value::Float(0.0).data_type(), Some(DataType::Float));
        assert_eq!(Value::Text("".into()).data_type(), Some(DataType::Text));
        assert_eq!(Value::Bool(false).data_type(), Some(DataType::Bool));
        assert_eq!(Value::Null.data_type(), None);
    }

    #[test]
    fn from_conversions() {
        assert_eq!(Value::from(42_i64), Value::Int(42));
        assert_eq!(Value::from(3.14_f64), Value::Float(3.14));
        assert_eq!(Value::from(true), Value::Bool(true));
        assert_eq!(Value::from("hello"), Value::Text("hello".to_string()));
        assert_eq!(Value::from("hello".to_string()), Value::Text("hello".to_string()));
    }

    #[test]
    fn display_values() {
        assert_eq!(Value::Int(42).to_string(), "42");
        assert_eq!(Value::Float(3.14).to_string(), "3.14");
        assert_eq!(Value::Text("hi".into()).to_string(), "hi");
        assert_eq!(Value::Bool(true).to_string(), "true");
        assert_eq!(Value::Null.to_string(), "NULL");
    }
}
