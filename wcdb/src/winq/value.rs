pub enum Value {
    Null,
    Integer32(i32),
    Integer64(i64),
    Float(f64),
    Text(String),
    Blob(Vec<u8>),
}

impl Value {
    pub fn int32(&self) -> i32 {
        match self {
            Value::Integer32(value) => *value as _,
            Value::Integer64(value) => *value as _,
            Value::Float(value) => *value as _,
            Value::Text(value) => value.parse().unwrap_or(0),
            _ => 0,
        }
    }

    pub fn int64(&self) -> i64 {
        match self {
            Value::Integer32(value) => *value as _,
            Value::Integer64(value) => *value as _,
            Value::Float(value) => *value as _,
            Value::Text(value) => value.parse().unwrap_or(0),
            _ => 0,
        }
    }

    pub fn float(&self) -> f64 {
        match self {
            Value::Integer32(value) => *value as _,
            Value::Integer64(value) => *value as _,
            Value::Float(value) => *value as _,
            Value::Text(value) => value.parse().unwrap_or(0.0),
            _ => 0.0,
        }
    }

    pub fn text(&self) -> String {
        match self {
            Value::Integer32(value) => format!("{}", value),
            Value::Integer64(value) => format!("{}", value),
            Value::Float(value) => format!("{}", value),
            Value::Text(value) => value.clone(),
            Value::Blob(value) => String::from_utf8_lossy(value).to_string(),
            _ => String::new(),
        }
    }

    pub fn blob(&self) -> Vec<u8> {
        match self {
            Value::Blob(value) => value.clone(),
            _ => self.text().into_bytes(),
        }
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Value::Null
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Integer32(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Integer64(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::Text(value.to_string())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::Text(value)
    }
}

impl From<Vec<u8>> for Value {
    fn from(value: Vec<u8>) -> Self {
        Value::Blob(value)
    }
}

impl From<&[u8]> for Value {
    fn from(value: &[u8]) -> Self {
        Value::Blob(value.to_vec())
    }
}
