use std::fmt;

#[derive(Clone)]
pub enum Value {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "Null"),
            Value::Boolean(b) => write!(f, "Boolean({b})"),
            Value::Integer(n) => write!(f, "Integer({n})"),
            Value::Float(n) => write!(f, "Float({n})"),
            Value::String(s) => write!(f, "String({s:?})"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "nil"),
            Value::Boolean(b) => write!(f, "{b}"),
            Value::Integer(n) => write!(f, "{n}"),
            Value::Float(n) => write!(f, "{n}"),
            Value::String(s) => write!(f, "{s}"),
        }
    }
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        !matches!(self, Value::Null | Value::Boolean(false))
    }

    pub fn as_number(&self) -> Result<f64, String> {
        match self {
            Value::Integer(n) => Ok(*n as f64),
            Value::Float(n) => Ok(*n),
            other => Err(format!("Expected number, got {other}")),
        }
    }

    pub fn as_integer(&self) -> Result<i64, String> {
        match self {
            Value::Integer(n) => Ok(*n),
            Value::Float(n) => {
                if *n == (*n as i64) as f64 {
                    Ok(*n as i64)
                } else {
                    Err(format!("Number has no integer representation: {n}"))
                }
            }
            other => Err(format!("Expected integer, got {other}")),
        }
    }

    pub fn as_string_value(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            other => format!("{other}"),
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Null => "Null",
            Value::Boolean(_) => "Boolean",
            Value::Integer(_) | Value::Float(_) => "Number",
            Value::String(_) => "String",
        }
    }

    // #region Math
    pub fn math_add(a: &Value, b: &Value) -> Result<Value, String> {
        match (a, b) {
            (Value::Integer(x), Value::Integer(y)) => Ok(Value::Integer(x.wrapping_add(*y))),
            _ => {
                let x = a.as_number()?;
                let y = b.as_number()?;
                Ok(Value::Float(x + y))
            }
        }
    }

    pub fn math_sub(a: &Value, b: &Value) -> Result<Value, String> {
        match (a, b) {
            (Value::Integer(x), Value::Integer(y)) => Ok(Value::Integer(x.wrapping_sub(*y))),
            _ => {
                let x = a.as_number()?;
                let y = b.as_number()?;
                Ok(Value::Float(x - y))
            }
        }
    }

    pub fn math_mul(a: &Value, b: &Value) -> Result<Value, String> {
        match (a, b) {
            (Value::Integer(x), Value::Integer(y)) => Ok(Value::Integer(x.wrapping_mul(*y))),
            _ => {
                let x = a.as_number()?;
                let y = b.as_number()?;
                Ok(Value::Float(x * y))
            }
        }
    }

    pub fn math_div(a: &Value, b: &Value) -> Result<Value, String> {
        match (a, b) {
            (Value::Integer(x), Value::Integer(y)) => Ok(Value::Integer(x.wrapping_div(*y))),
            _ => {
                let x = a.as_number()?;
                let y = b.as_number()?;
                if y == 0.0 {
                    return Err("Attempt to perform 'n/0'".to_string());
                }
                Ok(Value::Float(x / y))
            }
        }
    }

    pub fn math_idiv(a: &Value, b: &Value) -> Result<Value, String> {
        match (a, b) {
            (Value::Integer(x), Value::Integer(y)) => {
                if *y == 0 {
                    return Err("Attempt to perform 'n//0'".to_string());
                }
                Ok(Value::Integer(x.wrapping_div(*y)))
            }
            _ => {
                let x = a.as_number()?;
                let y = b.as_number()?;
                if y == 0.0 {
                    return Err("Attempt to perform 'n//0'".to_string());
                }
                Ok(Value::Float((x / y).floor()))
            }
        }
    }

    pub fn math_mod(a: &Value, b: &Value) -> Result<Value, String> {
        match (a, b) {
            (Value::Integer(x), Value::Integer(y)) => {
                if *y == 0 {
                    return Err("attempt to perform 'n%0'".to_string());
                }
                let r = x % y;
                let result = if (r != 0) && ((r ^ y) < 0) { r + y } else { r };
                Ok(Value::Integer(result))
            }
            _ => {
                let x = a.as_number()?;
                let y = b.as_number()?;
                if y == 0.0 {
                    return Err("attempt to perform 'n%0'".to_string());
                }
                let r = x - (x / y).floor() * y;
                Ok(Value::Float(r))
            }
        }
    }

    pub fn math_pow(a: &Value, b: &Value) -> Result<Value, String> {
        let x = a.as_number()?;
        let y = b.as_number()?;
        Ok(Value::Float(x.powf(y)))
    }

    pub fn math_neg(a: &Value) -> Result<Value, String> {
        match a {
            Value::Integer(x) => Ok(Value::Integer(-x)),
            Value::Float(x) => Ok(Value::Float(-x)),
            other => Err(format!("attempt to negate a {} value", other.type_name())),
        }
    }
    // #endregion

    // #region String
    pub fn string_concat(a: &Value, b: &Value) -> Result<Value, String> {
        let x = a.as_string_value();
        let y = b.as_string_value();
        Ok(Value::String(format!("{x}{y}")))
    }
    // #endregion
}
