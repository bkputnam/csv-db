use crate::types::Value;

/// A single row of data: an ordered sequence of `Value`s matching a `Schema`.
#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    pub values: Vec<Value>,
}

impl Row {
    pub fn new(values: Vec<Value>) -> Self {
        Row { values }
    }
}

/// Build a `Row` from anything that produces `Value`s.
impl<I> From<I> for Row
where
    I: IntoIterator<Item = Value>,
{
    fn from(iter: I) -> Self {
        Row::new(iter.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_from_vec() {
        let row = Row::new(vec![Value::Int(1), Value::Text("alice".into())]);
        assert_eq!(row.values.len(), 2);
        assert_eq!(row.values[0], Value::Int(1));
    }

    #[test]
    fn row_from_iterator() {
        let values = vec![Value::Bool(true), Value::Null];
        let row = Row::from(values);
        assert_eq!(row.values.len(), 2);
    }
}
