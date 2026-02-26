use std::fmt;

#[derive(Debug, PartialEq)]
pub enum DbError {
    /// Row has the wrong number of values for the schema.
    SchemaMismatch { expected: usize, got: usize },
    /// A value's type doesn't match the column's declared type.
    TypeMismatch {
        column: String,
        expected: String,
        got: String,
    },
    /// A null value was given for a non-nullable column.
    NullViolation { column: String },
    /// No column with this name exists in the schema.
    ColumnNotFound(String),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbError::SchemaMismatch { expected, got } => write!(
                f,
                "row has {got} value(s) but schema has {expected} column(s)"
            ),
            DbError::TypeMismatch { column, expected, got } => write!(
                f,
                "type mismatch in column '{column}': expected {expected}, got {got}"
            ),
            DbError::NullViolation { column } => {
                write!(f, "null value in non-nullable column '{column}'")
            }
            DbError::ColumnNotFound(name) => write!(f, "column '{name}' not found"),
        }
    }
}

impl std::error::Error for DbError {}

pub type Result<T> = std::result::Result<T, DbError>;
