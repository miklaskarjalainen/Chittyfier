#[derive(Debug)]
pub enum ValueE {
    ParsingError,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Literal(String),
    Boolean(bool),
    Null,
}

impl Value {
    /**
     * Parses Ints, Floats, Bools, Null, etc.
     * Doesn't parse string literals!
     */
    pub fn parse(s: &String) -> Result<Value, ValueE> {
        if let Ok(i) = s.parse::<i64>() {
            return Ok(Value::Int(i));
        }
        if let Ok(f) = s.parse::<f64>() {
            return Ok(Value::Float(f));
        }
        if s == "true" {
            return Ok(Value::Boolean(true));
        }
        if s == "false" {
            return Ok(Value::Boolean(false));
        }
        if s == "null" {
            return Ok(Value::Null);
        }
        Err(ValueE::ParsingError)
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::Int(i) => { return i.to_string(); },
            Value::Float(f) => { return f.to_string(); },
            Value::Literal(s) => { 
                let mut output = String::from("\"");
                output += s;
                output.push('"');
                return output;
            },
            Value::Boolean(b) => { return if *b { "true".to_string() } else { "false".to_string() } }
            Value::Null => { return "null".to_string(); },
        }
    }
}
