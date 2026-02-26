use crate::error::{DbError, Result};
use crate::row::Row;
use crate::types::DataType;

/// A single column definition: name, type, and whether it accepts nulls.
#[derive(Debug, Clone, PartialEq)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
    pub nullable: bool,
}

impl Column {
    pub fn new(name: impl Into<String>, data_type: DataType, nullable: bool) -> Self {
        Column {
            name: name.into(),
            data_type,
            nullable,
        }
    }

    /// Convenience constructor for a non-nullable column.
    pub fn required(name: impl Into<String>, data_type: DataType) -> Self {
        Self::new(name, data_type, false)
    }

    /// Convenience constructor for a nullable column.
    pub fn optional(name: impl Into<String>, data_type: DataType) -> Self {
        Self::new(name, data_type, true)
    }
}

/// The structure of a table: an ordered list of column definitions.
#[derive(Debug, Clone, PartialEq)]
pub struct Schema {
    pub columns: Vec<Column>,
}

impl Schema {
    pub fn new(columns: Vec<Column>) -> Self {
        Schema { columns }
    }

    /// Returns the 0-based index of the column with the given name, if it exists.
    pub fn column_index(&self, name: &str) -> Option<usize> {
        self.columns.iter().position(|c| c.name == name)
    }

    /// Returns a reference to the column with the given name, or an error.
    pub fn column(&self, name: &str) -> Result<&Column> {
        self.columns
            .iter()
            .find(|c| c.name == name)
            .ok_or_else(|| DbError::ColumnNotFound(name.to_string()))
    }

    /// Validates that a row is compatible with this schema.
    ///
    /// Checks:
    /// - The number of values equals the number of columns.
    /// - Each value's type matches its column's declared type.
    /// - No null value appears in a non-nullable column.
    pub fn validate_row(&self, row: &Row) -> Result<()> {
        if row.values.len() != self.columns.len() {
            return Err(DbError::SchemaMismatch {
                expected: self.columns.len(),
                got: row.values.len(),
            });
        }

        for (col, val) in self.columns.iter().zip(row.values.iter()) {
            if val.is_null() {
                if !col.nullable {
                    return Err(DbError::NullViolation {
                        column: col.name.clone(),
                    });
                }
                // A null value doesn't need a type check.
                continue;
            }

            if !val.matches_type(&col.data_type) {
                return Err(DbError::TypeMismatch {
                    column: col.name.clone(),
                    expected: col.data_type.to_string(),
                    got: val.type_name().to_string(),
                });
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Value;

    fn sample_schema() -> Schema {
        Schema::new(vec![
            Column::required("id", DataType::Int),
            Column::required("name", DataType::Text),
            Column::optional("score", DataType::Float),
            Column::required("active", DataType::Bool),
        ])
    }

    #[test]
    fn valid_row_passes() {
        let schema = sample_schema();
        let row = Row::new(vec![
            Value::Int(1),
            Value::Text("alice".into()),
            Value::Float(9.5),
            Value::Bool(true),
        ]);
        assert!(schema.validate_row(&row).is_ok());
    }

    #[test]
    fn null_in_nullable_column_passes() {
        let schema = sample_schema();
        let row = Row::new(vec![
            Value::Int(2),
            Value::Text("bob".into()),
            Value::Null, // score is optional
            Value::Bool(false),
        ]);
        assert!(schema.validate_row(&row).is_ok());
    }

    #[test]
    fn too_few_values_fails() {
        let schema = sample_schema();
        let row = Row::new(vec![Value::Int(1)]);
        assert_eq!(
            schema.validate_row(&row),
            Err(DbError::SchemaMismatch { expected: 4, got: 1 })
        );
    }

    #[test]
    fn too_many_values_fails() {
        let schema = sample_schema();
        let row = Row::new(vec![
            Value::Int(1),
            Value::Text("alice".into()),
            Value::Float(1.0),
            Value::Bool(true),
            Value::Int(99), // extra
        ]);
        assert_eq!(
            schema.validate_row(&row),
            Err(DbError::SchemaMismatch { expected: 4, got: 5 })
        );
    }

    #[test]
    fn type_mismatch_fails() {
        let schema = sample_schema();
        let row = Row::new(vec![
            Value::Text("not-an-int".into()), // id expects INT
            Value::Text("alice".into()),
            Value::Null,
            Value::Bool(true),
        ]);
        assert_eq!(
            schema.validate_row(&row),
            Err(DbError::TypeMismatch {
                column: "id".into(),
                expected: "INT".into(),
                got: "TEXT".into(),
            })
        );
    }

    #[test]
    fn null_in_non_nullable_column_fails() {
        let schema = sample_schema();
        let row = Row::new(vec![
            Value::Null, // id is required
            Value::Text("alice".into()),
            Value::Null,
            Value::Bool(true),
        ]);
        assert_eq!(
            schema.validate_row(&row),
            Err(DbError::NullViolation { column: "id".into() })
        );
    }

    #[test]
    fn column_index_found() {
        let schema = sample_schema();
        assert_eq!(schema.column_index("id"), Some(0));
        assert_eq!(schema.column_index("name"), Some(1));
        assert_eq!(schema.column_index("active"), Some(3));
    }

    #[test]
    fn column_index_not_found() {
        let schema = sample_schema();
        assert_eq!(schema.column_index("missing"), None);
    }

    #[test]
    fn column_lookup_ok() {
        let schema = sample_schema();
        assert_eq!(schema.column("name").unwrap().data_type, DataType::Text);
    }

    #[test]
    fn column_lookup_missing_returns_error() {
        let schema = sample_schema();
        assert_eq!(
            schema.column("ghost"),
            Err(DbError::ColumnNotFound("ghost".into()))
        );
    }
}
